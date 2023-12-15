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
	tiles: TileMap,
	waves: Vec<Wave>,
	puffins: Vec<Puffin>,
}

impl Map {
	pub fn new() -> Self {
		Self {
			tiles: TileMap::new(Default::default()),
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
		self.tiles.update(external, messenger);

		if let Some(wave) = Wave::maybe_spawn(&self.tiles, external) {
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
		{
			let span = trace_span!("Map");
			let _guard = span.enter();

			self.tiles.render(win);
		}

		{
			let span = trace_span!("Waves + Puffin");
			let _guard = span.enter();
			win.reserve(self.waves.len() + self.puffins.len());
			self.waves.iter().for_each(|wave| wave.render(win));
			self.puffins.iter().for_each(|puffin| puffin.render(win));
		}
	}

	fn cleanup(&mut self) {
		self.tiles.cleanup();
	}
}
