use super::glsl::*;

use std::hash::Hash;
use std::time::Instant;
use strum_macros::{EnumIter, IntoStaticStr};

pub type TextureMap = fnv::FnvHashMap<Texture, Instance>;

#[derive(IntoStaticStr, EnumIter, Hash, PartialEq, Eq, Clone, Copy)]
pub enum Texture {
	Flat,
	ShipSheet,
}

impl Texture {
	pub fn frame_count(&self) -> u32 {
		match self {
			Self::ShipSheet => 7,
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

	pub fn screen_to_world_pos(&self, (x, y): (f32, f32)) -> (f32, f32) {
		(self.scale * x + self.pos.0, self.scale * y + self.pos.1)
	}
}

pub trait Renderable {
	fn render(&mut self, text_map: &TextureMap, out: &mut Vec<Instance>, now: Instant);
}

#[derive(Clone)]
pub struct Animation {
	start: Instant,
	text: Texture,
	duration: f32,
	repeat: Option<f32>, //None means repeat forever
}

impl Animation {
	pub fn new(text: Texture, duration: f32, repeat: Option<f32>) -> Self {
		Self {
			start: Instant::now(),
			text,
			duration,
			repeat,
		}
	}

	pub fn get_frame(&self, text_map: &TextureMap, now: Instant) -> Instance {
		let elapsed = now.duration_since(self.start).as_secs_f32();
		let frame_count = self.text.frame_count();

		let frame = match self.repeat {
			Some(reps) if elapsed > reps * self.duration => frame_count - 1,
			_ => (frame_count as f32 * (elapsed / self.duration % 1.)) as u32,
		};

		text_map[&self.text].at_frame_n(frame, frame_count)
	}

	pub fn restart(&mut self) {
		self.start = Instant::now()
	}
}

impl Renderable for Animation {
	fn render(&mut self, text_map: &TextureMap, out: &mut Vec<Instance>, now: Instant) {
		out.push(self.get_frame(text_map, now))
	}
}
