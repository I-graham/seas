mod puffin;
mod tilemap;
mod wave;

use puffin::*;
use tilemap::*;
use wave::*;

use super::*;
use crate::eng::*;
use crate::window::*;

pub struct Environment {
	pub boats: Grid<Raft>,
	pub tiles: TileMap,
	waves: Vec<Wave>,
	puffins: Vec<Puffin>,
}

impl Environment {
	const SMALL_RENDER_SCALE: f32 = 6000.;

	pub fn new() -> Self {
		let mut boats = Grid::new(256.);
		boats.insert(Raft::new());

		Self {
			boats,
			tiles: TileMap::new(Default::default()),
			waves: vec![],
			puffins: vec![],
		}
	}

	pub fn act(&mut self, action: UIAction) {
		match action {
			UIAction::Routing(boat, route) => self.boats.get_mut(boat).unwrap().follow(route),
		}
	}
}

impl GameObject for Environment {
	type Scene = World;
	type Action = ();

	fn plan(&self, world: &World, external: &External, messenger: &Sender<Dispatch<Signal>>) {
		self.boats.plan(world, external, messenger);
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
		self.boats.update(external, messenger);

		if external.camera.scale < Self::SMALL_RENDER_SCALE {
			if let Some(wave) = Wave::maybe_spawn(&mut self.tiles, external) {
				self.waves.push(wave)
			}

			if let Some(puffin) = Puffin::maybe_spawn(external) {
				self.puffins.push(puffin)
			}
		}
		

		self.waves
			.retain_mut(|wave| wave.update(external, messenger) != Some(wave::Action::Die));

		self.puffins
			.retain_mut(|puffin| puffin.update(external, messenger) != Some(puffin::Action::Die));

		None
	}

	#[cfg_attr(feature = "profile", instrument(skip_all, name = "Environment"))]
	fn render(&self, win: &mut Window) {
		self.tiles.render(win);

		if win.external().camera.scale < Self::SMALL_RENDER_SCALE {
			win.reserve(self.waves.len() + self.puffins.len());
			self.waves.iter().for_each(|wave| wave.render(win));
			self.puffins.iter().for_each(|puffin| puffin.render(win));
		}

		self.boats.render(win);
	}

	fn cleanup(&mut self) {
		self.tiles.cleanup();
	}
}
