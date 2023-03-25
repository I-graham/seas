use super::glsl::*;

use std::hash::Hash;
use std::time::Instant;
use strum_macros::{EnumIter, IntoStaticStr};

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
		let k = self.camera.scale;

		(k * self.aspect(), k)
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

	pub fn emit(&self, out: &mut Vec<Instance>, instance: Instance) {
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
}

impl Texture {
	pub fn frame_count(&self) -> u32 {
		match self {
			Self::Wave => 27,
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
	pub fn scale(self, r: f32) -> Self {
		Self {
			scale: GLvec2(r * self.scale.0, r * self.scale.1),
			..self
		}
	}

	pub fn nth_frame(self, n: u32, out_of: u32) -> Self {
		let GLvec4(ulx, uly, lrx, lry) = self.texture;
		let shift = (lry - uly) / out_of as f32;
		let starty = uly + n as f32 * shift;

		let anti_bleed = shift * f32::EPSILON;

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
			-aspect * self.scale + self.pos.0,
			aspect * self.scale + self.pos.0,
			-self.scale + self.pos.1,
			self.scale + self.pos.1,
			-100.,
			100.,
		)
	}
}

type Curve = fn(f32) -> f32;

#[derive(Clone)]
pub struct Animation {
	start: Instant,
	texture: Texture,
	duration: f32,
	curve: Curve,
	repeat: Option<f32>, //None means repeat forever
}

use std::f32::consts::*;
impl Animation {
	pub const LINEAR: Curve = |f| f;

	pub const SIN: Curve = |f| (1. - (f * PI).cos()) / 2.;
	pub const SIN_BOUNCE: Curve = |f| Self::SIN(2. * f);

	pub fn new(
		texture: Texture,
		duration: f32,
		curve: fn(f32) -> f32,
		repeat: Option<f32>,
	) -> Self {
		Self {
			start: Instant::now(),
			texture,
			duration,
			curve,
			repeat,
		}
	}

	pub fn frame(&self, context: &External) -> Instance {
		let elapsed = self.age(context.now);
		let frames = self.texture.frame_count();

		let reps = elapsed / self.duration;

		let proportion = self.repeat.unwrap_or(reps).min(reps);

		let frame = (frames as f32 * (self.curve)(proportion % 1.)) as u32;

		context
			.instance(self.texture)
			.nth_frame(frame.min(frames - 1), frames)
	}

	pub fn finished(&self, now: Instant) -> bool {
		matches!(self.repeat, Some(reps) if self.age(now) > reps * self.duration)
	}

	pub fn age(&self, now: Instant) -> f32 {
		now.duration_since(self.start).as_secs_f32()
	}

	pub fn restart(&mut self) {
		self.start = Instant::now()
	}
}
