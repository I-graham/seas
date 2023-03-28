mod puffin;
mod wave;
use puffin::*;
use wave::*;

use super::{Action, GameObject};
use crate::game::random;
use crate::window::{glsl::*, External, Instance, Texture};

pub struct Map {
	pub size: u32,
	pub waves: Vec<Wave>,
	pub puffins: Vec<Puffin>,
}

impl Map {
	pub fn new(size: u32) -> Self {
		Self {
			size,
			waves: vec![],
			puffins: vec![],
		}
	}
}

impl GameObject for Map {
	fn update(&mut self, external: &External) -> Option<Action> {
		Wave::maybe_spawn(external).map(|wave| self.waves.push(wave));
		Puffin::maybe_spawn(external).map(|puffin| self.puffins.push(puffin));

		self.waves
			.retain_mut(|wave| wave.update(external) != Some(Action::Die));

		self.puffins
			.retain_mut(|puffin| puffin.update(external) != Some(Action::Die));

		None
	}

	fn render(&self, context: &External, out: &mut Vec<Instance>) {
		//Ocean
		context.clip(
			out,
			Instance {
				color_tint: GLvec4::rgba(57, 120, 168, 255),
				scale: GLvec2((self.size / 2) as f32, (self.size / 2) as f32),
				..context.instance(Texture::Flat)
			},
		);

		self.waves.iter().for_each(|wave| wave.render(context, out));
		self.puffins.iter().for_each(|puffin| puffin.render(context, out));

	}
}
