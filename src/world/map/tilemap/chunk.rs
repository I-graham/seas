use std::cell::Cell;
use std::mem::MaybeUninit;

use super::*;
use cgmath::*;
use noise::*;

pub struct Chunk {
	pub tiles: [[Tile; Self::DIMENSION]; Self::DIMENSION],
	pub cell_pos: Vector2<i32>,
	cache: Cell<Option<CacheId>>,
}

impl Chunk {
	//# of tiles in a chunk row
	pub const DIMENSION: usize = 32;

	//Size of a chunk, in pixels
	pub const WIDTH: f32 = Self::DIMENSION as f32 * Tile::SIZE;

	pub fn cell_id(v: Vector2<f32>) -> Vector2<i32> {
		v.map(|d| (d / Chunk::WIDTH).floor() as i32)
	}

	pub fn generate<F: NoiseFn<f64, 2>>(
		settings: TileMapSettings,
		cell_pos: Vector2<i32>,
		noise: F,
	) -> Self {
		let cell = cell_pos.cast::<f64>().unwrap() * Self::WIDTH as f64;

		let mut tiles: [[MaybeUninit<Tile>; Self::DIMENSION]; Self::DIMENSION] =
			unsafe { MaybeUninit::uninit().assume_init() };
		for (i, row) in tiles.iter_mut().enumerate() {
			for (j, entry) in row.iter_mut().enumerate() {
				let offset = vec2(i as f64 + 0.5, j as f64 + 0.5) * Tile::SIZE as f64;

				let pos = (cell + offset) / Chunk::WIDTH as f64;

				let reading = noise.get((pos / settings.scale).into());
				let height = reading.abs().powf(settings.height_pow) * reading.signum();

				let tile = Tile {
					kind: if height > settings.sea_level {
						TileKind::Land
					} else {
						TileKind::Sea
					},
				};

				entry.write(tile);
			}
		}

		let tiles = unsafe { std::mem::transmute(tiles) };

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

			let cell = self.cell_pos.cast::<f32>().unwrap() * Self::WIDTH;

			for (i, row) in self.tiles.iter().enumerate() {
				for (j, tile) in row.iter().enumerate() {
					let offset = vec2(i as f32 + 0.5, j as f32 + 0.5) * Tile::SIZE;
					out.push(
						Instance {
							position: (cell + offset).into(),
							color_tint: tile.color(),
							..win.external().instance(Texture::Flat)
						}
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
