mod puffin;
mod wave;
use puffin::*;
use wave::*;

use super::{Action, GameObject};
use crate::window::{glsl::*, External, Instance, Texture};

pub struct Map {
	pub size: u32,
	pub waves: Vec<Wave>,
	pub puffins: Vec<Puffin>,
}

impl Map {
	const BACKGROUND : GLvec4 = GLvec4(57., 120., 168., 255.);

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
		if let Some(wave) = Wave::maybe_spawn(external) {
			self.waves.push(wave)
		}
		
		if let Some(puffin) = Puffin::maybe_spawn(external) {
			self.puffins.push(puffin)
		}

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
				color_tint: Self::BACKGROUND.rgba(),
				scale: GLvec2((self.size / 2) as f32, (self.size / 2) as f32),
				..context.instance(Texture::Flat)
			},
		);

		self.waves.iter().for_each(|wave| wave.render(context, out));
		self.puffins
			.iter()
			.for_each(|puffin| puffin.render(context, out));
	}
}
