use super::GameObject;
use crate::window::{glsl::*, Animation, Context, Instance, Texture};
use std::time::Instant;

pub struct Map {
	pub size: u32,
	tile: Animation,
}

impl Map {
	pub fn new(context: &Context, size: u32) -> Self {
		Self {
			size,
			tile: Animation::new(
				context,
				Texture::Wave,
				2.0,
				Animation::SIN,
				None,
			),
		}
	}
}

impl GameObject for Map {
	fn render(&self, context: &Context, out: &mut Vec<Instance>, now: Instant) {
		context.emit(
			out,
			Instance {
				color_tint: GLvec4::rgba(99, 155, 255, 255),
				scale: GLvec2((self.size / 2) as f32, (self.size / 2) as f32),
				..context.instance(Texture::Flat)
			},
		);

		context.emit(out, self.tile.frame(now));
	}
}
