mod chunk;
mod settings;
mod tile;

pub use settings::*;
pub use tile::*;

use super::*;
use cgmath::*;
use chunk::*;
use fnv::FnvHashMap;
use noise::*;

type Noise = noise::OpenSimplex;
pub struct TileMap {
	settings: TileMapSettings,
	chunks: FnvHashMap<Vector2<i32>, Chunk>,
	noise_fn: Noise,
	chunks_in_view: ((i32, i32), (i32, i32)),
}

impl TileMap {
	pub fn new(settings: TileMapSettings) -> Self {
		let seed = settings.seed;
		Self {
			settings,
			chunks: Default::default(),
			noise_fn: Noise::default().set_seed(seed),
			chunks_in_view: Default::default(),
		}
	}

	fn load_chunk(&mut self, cell: Vector2<i32>) -> &mut Chunk {
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
		let lli: (i32, i32) = Chunk::cell_id(ll).into();
		let uri: (i32, i32) = Chunk::cell_id(ur).into();

		if (lli, uri) != self.chunks_in_view {
			let (old_ll, old_ur) = self.chunks_in_view;
			for cx in (lli.0..old_ll.0).chain(old_ur.0..=uri.0) {
				for cy in lli.1..=uri.1 {
					self.load_chunk(vec2(cx, cy));
				}
			}

			for cx in lli.0..=uri.0 {
				for cy in (lli.1..old_ll.1).chain(old_ur.1..=uri.1) {
					self.load_chunk(vec2(cx, cy));
				}
			}
		}

		self.chunks_in_view = (lli, uri);

		None
	}

	fn render(&self, win: &mut Window) {
		let (ll, ur) = self.chunks_in_view;
		for cx in ll.0..=ur.0 {
			for cy in ll.1..=ur.1 {
				self.chunks[&vec2(cx, cy)].render(win);
			}
		}
	}

	fn cleanup(&mut self) {
		let (ll, ur) = self.chunks_in_view;

		for chunk in self.chunks.values_mut() {
			let cell = chunk.cell_pos;
			let in_view = (ll.0..=ur.0).contains(&cell.x)
						&& (ll.1..=ur.1).contains(&cell.y);
			if !in_view {
				chunk.cleanup();
			}
		}
	}
}
