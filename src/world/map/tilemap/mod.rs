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
#[cfg(feature = "profile")]
use tracing::instrument;

type Noise = noise::OpenSimplex;

pub struct TileMap {
	pub settings: TileMapSettings,
	chunks: FnvHashMap<Vector2<i32>, Chunk>,
	noise_fn: Noise,
	chunks_in_view: [Vector2<i32>; 2],
}

impl TileMap {
	pub fn new(settings: TileMapSettings) -> Self {
		let seed = settings.seed;
		Self {
			settings,
			chunks: Default::default(),
			noise_fn: Noise::default().set_seed(seed),
			chunks_in_view: [vec2(0, 0); 2],
		}
	}

	//Returns None if tile not yet generated
	pub fn tile(&self, tile: Vector2<i32>) -> Option<&Tile> {
		self.tile_f(tile.map(|i| i as f32))
	}

	//Returns None if tile not yet generated
	pub fn tile_f(&self, pos: Vector2<f32>) -> Option<&Tile> {
		let (chunk_id, tile_id) = Chunk::tile_id(pos);
		let [i, j] = tile_id.into();

		self.chunks.get(&chunk_id).map(|chunk| chunk.get_tile(i, j))
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
		let lli = Chunk::chunk_id(ll) - vec2(1, 1);
		let uri = Chunk::chunk_id(ur) + vec2(1, 1);

		if [lli, uri] != self.chunks_in_view {
			let [old_ll, old_ur] = self.chunks_in_view;
			for cx in (lli.x..old_ll.x).chain(old_ur.x..=uri.x) {
				for cy in lli.y..=uri.y {
					self.load_chunk(vec2(cx, cy));
				}
			}

			for cx in lli.x..=uri.x {
				for cy in (lli.y..old_ll.y).chain(old_ur.y..=uri.y) {
					self.load_chunk(vec2(cx, cy));
				}
			}
		}

		self.chunks_in_view = [lli, uri];

		None
	}

	#[cfg_attr(feature = "profile", instrument(skip_all, name = "TileMap"))]
	fn render(&self, win: &mut Window) {
		let [ll, ur] = self.chunks_in_view;
		for cx in ll.x..=ur.x {
			for cy in ll.y..=ur.y {
				self.chunks[&vec2(cx, cy)].render(win);
			}
		}
	}

	fn cleanup(&mut self) {
		let [ll, ur] = self.chunks_in_view;

		for chunk in self.chunks.values_mut() {
			let cell = chunk.cell_pos;
			let in_view = (ll.x..=ur.x).contains(&cell.x) && (ll.y..=ur.y).contains(&cell.y);
			if !in_view {
				chunk.cleanup();
			}
		}
	}
}
