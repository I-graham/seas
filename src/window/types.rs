use super::glsl::*;

use std::hash::Hash;
use std::time::Instant;
use strum_macros::{EnumIter, IntoStaticStr};

pub type TextureMap = fnv::FnvHashMap<Texture, Instance>;

pub struct Context {
	pub texture_map: TextureMap,
	pub size: (u32, u32),
	pub camera: Camera,
}

impl Context {
	pub fn visible(&self, instance: Instance) -> bool {
		let (cx, cy) = self.camera.pos;
		let GLvec2(px, py) = instance.position;
		let GLvec2(sx, sy) = instance.scale;

		let rad = self.camera.scale * self.aspect().hypot(1.);

		instance.screen_relative == GLbool::True
			|| (px - cx * self.aspect()).hypot(py - cy) < rad + sx.hypot(sy)
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
	ReadyButton,
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

	pub fn at_frame_n(self, n: u32, out_of: u32) -> Self {
		let GLvec4(ulx, uly, lrx, lry) = self.texture;
		let dy = lry - uly;
		Self {
			texture: GLvec4(
				ulx,
				uly + dy * n as f32 / out_of as f32,
				lrx,
				uly + dy * (n + 1) as f32 / out_of as f32,
			),
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
	pub fn proj(&self) -> cgmath::Matrix4<f32> {
		cgmath::ortho(
			-self.scale + self.pos.0,
			self.scale + self.pos.0,
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
	text: Texture,
	inst: Instance,
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
		context: &Context,
		texture: Texture,
		duration: f32,
		curve: fn(f32) -> f32,
		repeat: Option<f32>,
	) -> Self {
		Self {
			start: Instant::now(),
			text: texture,
			inst: context.instance(texture),
			duration,
			curve,
			repeat,
		}
	}

	pub fn frame(&self, now: Instant) -> Instance {
		let elapsed = now.duration_since(self.start).as_secs_f32();
		let frames = self.text.frame_count();

		let reps = elapsed / self.duration;

		let proportion = self.repeat.unwrap_or(reps).min(reps);

		let frame = (frames as f32 * (self.curve)(proportion % 1.)) as u32;

		self.inst.at_frame_n(frame, frames)
	}

	pub fn restart(&mut self) {
		self.start = Instant::now()
	}
}
