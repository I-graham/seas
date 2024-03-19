mod chunk;
mod gen;
mod kinds;
mod settings;
mod tile;

pub use gen::*;
pub use kinds::*;
pub use settings::*;
pub use tile::*;

use super::*;
use cgmath::*;
use chunk::*;
use fnv::FnvHashMap;

#[cfg(feature = "profile")]
use tracing::instrument;

pub struct TileMap {
	settings: TileMapSettings,
	chunks: FnvHashMap<Vector2<i32>, Task<Chunk>>,
	noise_fn: Generator,
	chunks_in_view: [Vector2<i32>; 2],
}

impl TileMap {
	const PRELOAD_RADIUS: usize = 5;
	const PREGEN_CHUNK_RAD: i32 = 2;

	pub fn new(settings: TileMapSettings) -> Self {
		let rad = Self::PRELOAD_RADIUS as i32;
		let corner = vec2(rad, rad);

		let noise_fn = Generator::init(settings.seed);

		let mut out = Self {
			settings,
			chunks: Default::default(),
			noise_fn,
			chunks_in_view: [-corner, corner],
		};

		for cx in -rad..rad {
			for cy in -rad..rad {
				out.launch_chunk_gen(vec2(cx, cy));
			}
		}

		out
	}

	pub fn tile(&mut self, tile: Vector2<i32>) -> &mut Tile {
		self.tile_f(tile.map(|i| i as f32))
	}

	pub fn tile_f(&mut self, pos: Vector2<f32>) -> &mut Tile {
		let (chunk_id, tile_id) = Chunk::tile_id(pos);
		let [i, j] = tile_id.into();

		self.load_chunk(chunk_id).get_tile_mut(i, j)
	}

	//get tile if it has already been loaded in
	pub fn maybe_tile(&self, tile: Vector2<i32>) -> Option<&Tile> {
		self.maybe_tile_f(tile.map(|i| i as f32))
	}

	pub fn maybe_tile_f(&self, pos: Vector2<f32>) -> Option<&Tile> {
		let (chunk_id, tile_id) = Chunk::tile_id(pos);
		let [i, j] = tile_id.into();

		self.maybe_chunk(chunk_id).map(|chunk| chunk.get_tile(i, j))
	}

	fn launch_chunk_gen(&mut self, cell: Vector2<i32>) {
		let settings = self.settings;
		let noise = self.noise_fn.clone();

		let generate = move || Chunk::generate(settings, cell, &noise);

		self.chunks
			.entry(cell)
			.or_insert_with(|| Task::launch(generate));
	}

	//return chunk if it has loaded in
	fn maybe_chunk(&self, cell: Vector2<i32>) -> Option<&Chunk> {
		self.chunks.get(&cell).and_then(|task| task.if_done())
	}

	fn load_chunk(&mut self, cell: Vector2<i32>) -> &mut Chunk {
		self.chunks
			.entry(cell)
			.or_insert_with(|| Task::from_val(Chunk::generate(self.settings, cell, &self.noise_fn)))
			.get_mut()
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
			let gen_lli = lli - Self::PREGEN_CHUNK_RAD * vec2(1, 1);
			let gen_uri = uri + Self::PREGEN_CHUNK_RAD * vec2(1, 1);

			let [old_ll, old_ur] = self.chunks_in_view;
			for cx in (gen_lli.x..old_ll.x).chain(old_ur.x..=gen_uri.x) {
				for cy in gen_lli.y..=gen_uri.y {
					self.launch_chunk_gen(vec2(cx, cy));
				}
			}

			for cx in gen_lli.x..=gen_uri.x {
				for cy in (gen_lli.y..old_ll.y).chain(old_ur.y..=gen_uri.y) {
					self.launch_chunk_gen(vec2(cx, cy));
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
				self.chunks[&vec2(cx, cy)].get().render(win);
			}
		}
	}

	fn cleanup(&mut self) {
		let [ll, ur] = self.chunks_in_view;

		for chunk in self
			.chunks
			.values_mut()
			.filter_map(|task| task.if_done_mut())
		{
			let cell = chunk.cell_pos;
			let in_view = (ll.x..=ur.x).contains(&cell.x) && (ll.y..=ur.y).contains(&cell.y);
			if !in_view {
				chunk.cleanup();
			}
		}
	}
}
