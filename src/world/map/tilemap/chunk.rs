use std::cell::Cell;

use super::*;
use cgmath::*;
use noise::*;

pub struct Chunk {
	tiles: Box<[Tile; Self::DIMENSION * Self::DIMENSION]>,
	pub cell_pos: Vector2<i32>,
	cache: Cell<Option<CacheId>>,
}

impl Chunk {
	//# of tiles in a chunk row
	pub const DIMENSION: usize = 32;

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

	pub fn generate<F: NoiseFn<f64, 2>>(
		settings: TileMapSettings,
		cell_pos: Vector2<i32>,
		noise: F,
	) -> Self {
		let cell = cell_pos.cast::<f32>().unwrap() * Self::WIDTH;

		let mut tiles = Vec::with_capacity(Self::DIMENSION * Self::DIMENSION);

		for i in 0..Self::DIMENSION {
			for j in 0..Self::DIMENSION {
				let offset = vec2(i as f32 + 0.5, j as f32 + 0.5) * Tile::SIZE;

				let pos = ((cell + offset) / settings.scale).map(|d| d as f64);

				let reading = noise.get(pos.into()) as f32;

				tiles.push(Tile::generate(&settings, reading));
			}
		}

		let boxed_tiles = tiles.into_boxed_slice();
		let tiles: Box<[Tile; Self::DIMENSION * Self::DIMENSION]> = boxed_tiles.try_into().unwrap();

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
		let span = trace_span!("Rendering Chunk");
		let _guard = span.enter();

		let cache_id = self.cache.take().unwrap_or_else(|| {
			let mut out = Vec::with_capacity(Self::DIMENSION * Self::DIMENSION);

			let cell = self.cell_pos.cast::<f32>().unwrap() * Self::WIDTH;

			for i in 0..Self::DIMENSION {
				for j in 0..Self::DIMENSION {
					let tile = self.get_tile(i, j);

					let offset = vec2(i as f32 + 0.5, j as f32 + 0.5) * Tile::SIZE;
					out.push(
						Instance {
							position: (cell + offset).into(),
							color_tint: tile.color,
							..win.external().instance(Texture::Flat)
						}
						.scale_rgba()
						.scale(Tile::SIZE),
					);
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
