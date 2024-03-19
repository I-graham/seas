use cgmath::Vector2;

use super::*;
use crate::eng::*;
use crate::window::*;

pub struct Wave {
	pos: Vector2<f32>,
	animation: Animation<Texture>,
}

#[derive(PartialEq, Copy, Clone)]
pub enum Action {
	Die,
}

impl Wave {
	const DENSITY: f32 = 1. / 500_000.;
	const SPAWN_MARGIN: f32 = 1.5;

	pub fn maybe_spawn(map: &mut TileMap, external: &External) -> Option<Self> {
		let v = external.view_dims() / 2.;
		let cam = external.camera.pos;
		let offset = v.map(|f| rand_in(-f, f)) * Self::SPAWN_MARGIN;
		let pos = cam + offset;

		let tile_id = snap_to_grid(pos, (Tile::SIZE, Tile::SIZE));

		if probability(Self::DENSITY * external.delta * v.x * v.y)
			&& map
				.maybe_tile(tile_id)
				.is_some_and(|tile| tile.kind == TileKind::DeepSea)
		{
			Some(Wave {
				pos,
				animation: Animation::new(Texture::Wave, 4., curves::SIN_SQ, 1.0),
			})
		} else {
			None
		}
	}
}

impl GameObject for Wave {
	type Scene = World;
	type Action = Action;
	fn update(&mut self, external: &External, _messenger: &Messenger<Signal>) -> Option<Action> {
		if self.animation.finished(external.now) {
			Some(Action::Die)
		} else {
			None
		}
	}

	fn instance(&self, external: &External) -> Option<Instance> {
		Some(Instance {
			position: self.pos.into(),
			..self.animation.frame(external)
		})
	}
}
