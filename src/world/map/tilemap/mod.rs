mod chunk;
mod tile;

pub use chunk::*;
pub use tile::*;

use super::*;
use cgmath::*;
use fnv::FnvHashMap;
use noise::*;

#[derive(Clone, Copy)]
pub struct TileMapSettings {
	sea_level: f64,
	seed: u32,
}

pub struct TileMap {
	settings: TileMapSettings,
	chunks: FnvHashMap<Vector2<i32>, Chunk>,
	noise_fn: PerlinSurflet,
}

impl Default for TileMapSettings {
	fn default() -> Self {
		Self {
			sea_level: 0.0,
			seed: rand::random(),
		}
	}
}

impl TileMap {
	pub fn new(settings: TileMapSettings) -> Self {
		let seed = settings.seed;
		Self {
			settings,
			chunks: Default::default(),
			noise_fn: PerlinSurflet::default().set_seed(seed),
		}
	}

	fn chunk_at(&mut self, cell: Vector2<i32>) -> &mut Chunk {
		self.chunks
			.entry(cell)
			.or_insert_with(|| Chunk::generate(self.settings, cell, self.noise_fn))
	}
}

impl GameObject for TileMap {
	type Scene = World;
	type Action = ();

	fn update(
		&mut self,
		external: &External,
		_messenger: &Messenger<Signal>,
	) -> Option<Self::Action> {
		//Generate all chunks in view
		let (ll, ur) = external.view_bounds();
		let (llx, lly) = Chunk::cell_id(ll).into();
		let (urx, ury) = Chunk::cell_id(ur).into();
		for cx in llx - 1..=urx + 1 {
			for cy in lly - 1..=ury + 1 {
				self.chunk_at(vec2(cx, cy));
			}
		}

		None
	}

	fn render(&self, win: &mut Window) {
		let (ll, ur) = win.external().view_bounds();
		let (llx, lly) = Chunk::cell_id(ll).into();
		let (urx, ury) = Chunk::cell_id(ur).into();

		for cx in llx..=urx {
			for cy in lly..=ury {
				self.chunks[&vec2(cx, cy)].render(win);
			}
		}
	}
}
