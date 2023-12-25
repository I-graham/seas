use std::cell::Cell;

use super::*;
use cgmath::*;

pub struct Chunk {
	pub cell_pos: Vector2<i32>,
	tiles: Box<[Tile; Self::DIMENSION * Self::DIMENSION]>,
	cache: Cell<Option<CacheId>>,
}

impl Chunk {
	//# of tiles in a chunk row
	pub const DIMENSION: usize = 64;

	//Size of a chunk, in pixels
	pub const WIDTH: f32 = Self::DIMENSION as f32 * Tile::SIZE;

	pub fn get_tile(&self, i: usize, j: usize) -> &Tile {
		&self.tiles[i * Self::DIMENSION + j]
	}

	pub fn chunk_id(v: Vector2<f32>) -> Vector2<i32> {
		v.map(|d| d.div_euclid(Chunk::WIDTH) as i32)
	}

	pub fn tile_id(v: Vector2<f32>) -> (Vector2<i32>, Vector2<usize>) {
		let chunk = v.map(|d| d.div_euclid(Chunk::WIDTH) as i32);
		let tile = v.map(|d| (d.rem_euclid(Chunk::WIDTH) / Tile::SIZE) as usize);
		(chunk, tile)
	}

	#[cfg_attr(feature = "profile", instrument(skip_all, name = "Generating Chunks"))]
	pub fn generate_chunk(
		settings: TileMapSettings,
		cell_pos: Vector2<i32>,
		noise: &NoiseGenerator,
	) -> Self {
		//Generate geography

		let cell = cell_pos.map(|f| f as f32) * Self::WIDTH;

		let mut tiles = Vec::with_capacity(Self::DIMENSION * Self::DIMENSION);

		for i in 0..Self::DIMENSION {
			for j in 0..Self::DIMENSION {
				let offset = vec2(i as f32 + 0.5, j as f32 + 0.5) * Tile::SIZE;

				let pos = (cell + offset) / settings.scale;

				let reading = noise.read(pos.into());

				tiles.push(Tile::generate_geography(&settings, reading));
			}
		}

		let boxed_tiles = tiles.into_boxed_slice();
		let tiles = boxed_tiles.try_into().unwrap();

		//Attempt to place dock in this chunk
		if probability(settings.dock_prob) {
			//TODO!
		}

		Self {
			cell_pos,
			tiles,
			cache: None.into(),
		}
	}
}

impl GameObject for Chunk {
	type Scene = World;
	type Action = ();

	fn render(&self, win: &mut Window) {
		let cache_id = self.cache.take().unwrap_or_else(|| {
			let mut out = Vec::with_capacity(Self::DIMENSION * Self::DIMENSION);

			let cell = self.cell_pos.map(|f| f as f32) * Self::WIDTH;

			let external = win.external();

			for i in 0..Self::DIMENSION {
				for j in 0..Self::DIMENSION {
					let tile = self.get_tile(i, j);

					let offset = vec2(i as f32 + 0.5, j as f32 + 0.5) * Tile::SIZE;
					out.push(Instance {
						position: (cell + offset).into(),
						..tile.instance(external)
					});
				}
			}

			win.cache(&out)
		});

		win.draw_cached(&cache_id, &vec2(0., 0.), 1.);
		self.cache.set(Some(cache_id));
	}

	fn cleanup(&mut self) {
		self.cache.take();
	}
}
