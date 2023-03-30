mod animation;

use super::glsl::*;

use std::hash::Hash;
use std::time::Instant;
use strum_macros::{EnumIter, IntoStaticStr};

pub use animation::Animation;

pub type TextureMap = fnv::FnvHashMap<Texture, Instance>;

pub struct External {
	pub texture_map: TextureMap,
	pub size: (u32, u32),
	pub camera: Camera,
	pub now: Instant,
	pub delta: f32,
}

impl External {
	pub fn refresh(&mut self) {
		let now = Instant::now();
		self.delta = now.duration_since(self.now).as_secs_f32();
		self.now = now;
	}

	pub fn view_dims(&self) -> (f32, f32) {
		let k = 2. * self.camera.scale;

		(k * self.aspect(), k)
	}

	pub fn point_in_view(&self, (x, y): (f32, f32)) -> bool {
		self.visible(Instance {
			position: (x, y).into(),
			scale: (0., 0.).into(),
			..Default::default()
		})
	}

	pub fn visible(&self, instance: Instance) -> bool {
		let (cx, cy) = self.camera.pos;
		let GLvec2(px, py) = instance.position;
		let GLvec2(sx, sy) = instance.scale;

		//maximal possible distance, since instances may be rotated
		let max = sx.hypot(sy);

		let (dx, dy) = self.view_dims();

		instance.screen_relative == GLbool::True
			|| ((px - cx).abs() < max + dx && (py - cy).abs() < max + dy)
	}

	pub fn clip(&self, out: &mut Vec<Instance>, instance: Instance) {
		//clip unseen instances
		if self.visible(instance) {
			out.push(instance);
		}
	}

	pub fn corner_relative(&self, (dx, dy): (f32, f32)) -> (f32, f32) {
		(dx / self.aspect() - dx.signum(), dy - dy.signum())
	}

	pub fn instance(&self, texture: Texture) -> Instance {
		self.texture_map[&texture]
	}

	pub fn aspect(&self) -> f32 {
		self.size.0 as f32 / self.size.1 as f32
	}
}

#[derive(IntoStaticStr, EnumIter, Hash, PartialEq, Eq, Clone, Copy)]
pub enum Texture {
	Flat,
	Wave,
	Wood,
	Puffin,
	PuffinPeck,
	PuffinFlip,
	PuffinFly,
	PuffinFlap,
}

impl Texture {
	pub fn frame_count(&self) -> u32 {
		match self {
			Self::Wave => 27,
			Self::PuffinPeck => 4,
			Self::PuffinFly => 8,
			Self::PuffinFlap => 5,
			_ => 1,
		}
	}
}

#[repr(C, align(16))]
#[derive(Copy, Clone, Debug)]
pub struct Instance {
	pub color_tint: GLvec4,
	pub texture: GLvec4,
	pub scale: GLvec2,
	pub position: GLvec2,
	pub rotation: GLfloat,
	pub screen_relative: GLbool,
}

impl Instance {
	pub fn scale(self, x: f32, y: f32) -> Self {
		Self {
			scale: GLvec2(x * self.scale.0, y * self.scale.1),
			..self
		}
	}

	pub fn nth_frame(self, n: u32, out_of: u32) -> Self {
		let GLvec4(ulx, uly, lrx, lry) = self.texture;
		let shift = (lry - uly) / out_of as f32;
		let starty = uly + n as f32 * shift;

		const ANTI_BLEED_MULTIPLIER: f32 = 10. * f32::EPSILON;
		let anti_bleed = shift * ANTI_BLEED_MULTIPLIER;

		Self {
			texture: GLvec4(ulx, starty + anti_bleed, lrx, starty + shift - anti_bleed),
			..self
		}
	}
}

impl Default for Instance {
	fn default() -> Self {
		Instance {
			color_tint: GLvec4(1.0, 1.0, 1.0, 1.0),
			texture: GLvec4(0.0, 0.0, 1.0, 1.0),
			scale: GLvec2(1.0, 1.0),
			position: GLvec2(0.0, 0.0),
			rotation: GLfloat(0.0),
			screen_relative: GLbool::False,
		}
	}
}

#[derive(Clone, Copy)]
pub struct Camera {
	pub pos: (f32, f32),
	pub scale: f32,
}

impl Camera {
	pub fn proj(&self, aspect: f32) -> cgmath::Matrix4<f32> {
		cgmath::ortho(
			self.pos.0 - aspect * self.scale,
			self.pos.0 + aspect * self.scale,
			self.pos.1 - self.scale,
			self.pos.1 + self.scale,
			-100.,
			100.,
		)
	}
}
