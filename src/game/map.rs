use super::GameObject;
use crate::window::{glsl::*, Animation, Context, Instance, PlayMode, Texture};

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
				Texture::Water,
				4.0,
				PlayMode::Forever,
			),
		}
	}
}

impl GameObject for Map {
	fn render(&mut self, context: &mut Context, now: std::time::Instant) {
		let lim = self.size as i32 / 2;
		for i in -lim..lim {
			for j in -lim..lim {
				const SCALE: f32 = 0.25;
				let (x, y) = (i as f32 * SCALE, j as f32 * SCALE);
				context.emit(Instance {
					translate: GLvec2(2. * x, 2. * y),
					..self.tile.get_frame(now).scaled(SCALE)
				});
			}
		}
	}
}
