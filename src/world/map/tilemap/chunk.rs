use std::mem::MaybeUninit;

use super::*;
use cgmath::*;
use noise::*;

pub struct Chunk {
	cell_pos: Vector2<i32>,
	tiles: [[Tile; Self::DIMENSION]; Self::DIMENSION],
}

impl Chunk {
	//# of tiles in a chunk row
	pub const DIMENSION: usize = 16;

	//Size of a chunk, in pixels
	pub const WIDTH: f32 = Self::DIMENSION as f32 * Tile::SIZE;

	pub fn cell_id(v: Vector2<f32>) -> Vector2<i32> {
		v.map(|d| (d / Chunk::WIDTH).floor() as i32)
	}

	pub fn generate<F: NoiseFn<f64, 3>>(cell_pos: Vector2<i32>, noise: F) -> Self {
		let mut tiles: [[MaybeUninit<Tile>; Self::DIMENSION]; Self::DIMENSION] =
			unsafe { MaybeUninit::uninit().assume_init() };

		for rows in &mut tiles[..] {
			for row in &mut rows[..] {
				let tile = Tile {
					kind: if probability(0.5) {
						TileKind::Land
					} else {
						TileKind::Sea
					},
				};

				row.write(tile);
			}
		}

		let tiles = unsafe { std::mem::transmute(tiles) };

		Self { cell_pos, tiles }
	}
}

impl GameObject for Chunk {
	type Scene = World;
	type Action = ();

	fn render(&self, win: &mut Window) {
		let cell = self.cell_pos.cast::<f32>().unwrap() * Self::WIDTH;

		for (i, row) in self.tiles.iter().enumerate() {
			for (j, tile) in row.iter().enumerate() {
				let offset = vec2(i as f32 + 0.5, j as f32 + 0.5) * Tile::SIZE;
				win.clip(
					Instance {
						position: (cell + offset).into(),
						color_tint: tile.color(),
						..win.external().instance(Texture::Flat)
					}
					.scale(Tile::SIZE),
				);
			}
		}
	}
}
