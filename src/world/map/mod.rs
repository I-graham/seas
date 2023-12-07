mod puffin;
mod wave;

use puffin::*;
use wave::*;

use super::*;
use crate::eng::*;
use crate::window::*;

pub struct Map {
	size: u32,
	waves: Vec<Wave>,
	puffins: Vec<Puffin>,
}

impl Map {
	const BACKGROUND: GLvec4 = GLvec4(57., 120., 168., 255.);

	pub fn new(size: u32) -> Self {
		Self {
			size,
			waves: vec![],
			puffins: vec![],
		}
	}
}

impl GameObject for Map {
	type Scene = World;
	type Action = ();

	fn plan(&self, world: &World, external: &External, messenger: &Sender<Dispatch<Signal>>) {
		for puffin in &self.puffins {
			puffin.plan(world, external, messenger);
		}
	}

	fn update(
		&mut self,
		external: &External,
		messenger: &Messenger<Signal>,
	) -> Option<Self::Action> {
		if let Some(wave) = Wave::maybe_spawn(external) {
			self.waves.push(wave)
		}

		if let Some(puffin) = Puffin::maybe_spawn(external) {
			self.puffins.push(puffin)
		}

		self.waves
			.retain_mut(|wave| wave.update(external, messenger) != Some(wave::Action::Die));

		self.puffins
			.retain_mut(|puffin| puffin.update(external, messenger) != Some(puffin::Action::Die));

		None
	}

	fn render(&self, win: &mut Window) {
		//Ocean
		win.clip(Instance {
			color_tint: Self::BACKGROUND.rgba(),
			scale: GLvec2((self.size / 2) as f32, (self.size / 2) as f32),
			..win.inputs().instance(crate::world::Texture::Flat)
		});

		self.waves.iter().for_each(|wave| wave.render(win));
		self.puffins.iter().for_each(|puffin| puffin.render(win));
	}
}
