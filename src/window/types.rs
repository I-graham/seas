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
	text: Texture,
	last_update: Instant,
	frame: u32,
	duration: f32,
}

impl Animation {
	pub fn new(text: Texture, duration: f32) -> Self {
		Self {
			text,
			last_update: Instant::now(),
			frame: 0,
            duration,
		}
	}

	pub fn reset(&mut self) {
		self.last_update = Instant::now()
	}
}

impl Renderable for Animation {
	fn render(&mut self, text_map: &TextureMap, out: &mut Vec<Instance>, now: Instant) {
		let delta = now.duration_since(self.last_update).as_secs_f32();
		let frames = self.text.frame_count();
		if delta > self.duration / (frames as f32) {
			self.reset();
			self.frame += 1;
			self.frame %= frames;
		}
		out.push(text_map[&self.text].at_frame_n(self.frame, frames));
	}
}
