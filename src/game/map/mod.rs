mod wave;
use wave::Wave;

use super::{Action, GameObject};
use crate::game::random;
use crate::window::{glsl::*, External, Instance, Texture};

pub struct Map {
	pub size: u32,
	pub waves: Vec<Wave>,
}

impl Map {
	pub fn new(size: u32) -> Self {
		Self {
			size,
			waves: vec![],
		}
	}
}

impl GameObject for Map {
	fn update(&mut self, external: &External) -> Option<Action> {

		Wave::spawn_into(external, &mut self.waves);

		let mut i = 0;
		while i < self.waves.len() {
			let wave = &mut self.waves[i];
			if wave.update(external) == Some(Action::Die) {
				self.waves.swap_remove(i);
			} else {
				i += 1
			}
		}

		None
	}

	fn render(&self, context: &External, out: &mut Vec<Instance>) {
		context.emit(
			out,
			Instance {
				color_tint: GLvec4::rgba(99, 155, 255, 255),
				scale: GLvec2((self.size / 2) as f32, (self.size / 2) as f32),
				..context.instance(Texture::Flat)
			},
		);

		for wave in &self.waves {
			wave.render(context, out);
		}
	}
}
