use super::glsl::*;

use std::hash::Hash;
use std::time::Instant;
use strum_macros::{EnumIter, IntoStaticStr};

pub type TextureMap = fnv::FnvHashMap<Texture, Instance>;

pub struct Context {
	pub texture_map: TextureMap,
	pub size: (u32, u32),
	pub camera: Camera,
	pub instances: Vec<Instance>,
}

impl Context {
	pub fn emit(&mut self, instance: Instance) {
        //clip unseen instances
		
        let (cx, cy) = self.camera.pos;
        let rad = self.camera.scale * self.aspect().hypot(1.);
        let GLvec2(ix, iy) = instance.translate;
        let GLvec2(sx, sy) = instance.scale;

        if (ix-cx).abs().hypot((iy-cy).abs()) < (rad + sx.hypot(sy)) {
		    self.instances.push(instance);
        }
	}

	//World Coordinates of a position described relative to a corner.
	//Useful for things with fixed position regardless of window dimensions.
	//dx, dy is the position of the object relative to some corner. Interpreted
	//such that it refers to a point inside the screen if |dx|,|dy| < 1
	pub fn corner_relative_to_world(&self, pos: (f32, f32)) -> (f32, f32) {
		self.screen_to_world_pos(self.corner_relative(pos))
	}

	pub fn screen_to_world_pos(&self, (x, y): (f32, f32)) -> (f32, f32) {
		(
			self.camera.scale * x * self.aspect() + self.camera.pos.0,
			self.camera.scale * y + self.camera.pos.1,
		)
	}

	pub fn corner_relative(&self, (dx, dy): (f32, f32)) -> (f32, f32) {
		(dx / self.aspect() - dx.signum(), dy - dy.signum())
	}

	pub fn get_inst(&self, texture: Texture) -> Instance {
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
	Water,
}

impl Texture {
	pub fn frame_count(&self) -> u32 {
		match self {
			Self::Water => 32,
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
	pub translate: GLvec2,
	pub rotation: GLfloat,
}

impl Instance {
	pub fn scaled(self, r: f32) -> Self {
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
			translate: GLvec2(0.0, 0.0),
			rotation: GLfloat(0.0),
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

#[derive(Clone)]
pub enum PlayMode {
	Repeat(f32),
	Functional(fn(f32) -> f32),
	Forever,
}

#[derive(Clone)]
pub struct Animation {
	start: Instant,
	text: Texture,
	inst: Instance,
	duration: f32,
	repeat: PlayMode, //None means repeat forever
}

impl Animation {
	pub fn new(context: &Context, texture: Texture, duration: f32, repeat: PlayMode) -> Self {
		Self {
			start: Instant::now(),
			text: texture,
			inst: context.get_inst(texture),
			duration,
			repeat,
		}
	}

	pub fn get_frame(&self, now: Instant) -> Instance {
		let elapsed = now.duration_since(self.start).as_secs_f32();
		let frames = self.text.frame_count();

		let proportion = elapsed / self.duration;

		let frame = match self.repeat {
			PlayMode::Repeat(reps) if proportion > reps => frames - 1,
			PlayMode::Functional(f) => (frames as f32 * f(proportion % 1.)) as u32,
			_ => (frames as f32 * (proportion % 1.)) as u32,
		};

		self.inst.at_frame_n(frame, frames)
	}

	pub fn restart(&mut self) {
		self.start = Instant::now()
	}
}
