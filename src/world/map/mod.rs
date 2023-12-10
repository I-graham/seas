mod puffin;
mod tilemap;
mod wave;

use puffin::*;
use tilemap::*;
use wave::*;

use super::*;
use crate::eng::*;
use crate::window::*;

pub struct Map {
	size: u32,
	tilemap: TileMap,
	waves: Vec<Wave>,
	puffins: Vec<Puffin>,
}

impl Map {
	pub fn new(size: u32) -> Self {
		Self {
			size,
			tilemap: TileMap::new(Default::default()),
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

		self.tilemap.update(external, messenger);

		self.waves
			.retain_mut(|wave| wave.update(external, messenger) != Some(wave::Action::Die));

		self.puffins
			.retain_mut(|puffin| puffin.update(external, messenger) != Some(puffin::Action::Die));

		None
	}

	fn render(&self, win: &mut Window) {
		self.tilemap.render(win);
		self.waves.iter().for_each(|wave| wave.render(win));
		self.puffins.iter().for_each(|puffin| puffin.render(win));
	}
}
