use super::*;
use crate::window::GLvec4;

#[derive(Debug)]
pub struct Tile {
	pub kind: TileKind,
	pub height: f32,
	pub tint: GLvec4,
}

impl Tile {
	pub const SIZE: f32 = 32.;

	pub fn generate_geography(settings: &TileMapSettings, reading: f32) -> Self {
		let height = reading.abs().powf(settings.height_pow) * reading.signum();

		let boundaries = [
			(TileKind::Land, settings.land_lvl),
			(TileKind::Shore, settings.shore_lvl),
			(TileKind::Shore, settings.sea_lvl),
			(TileKind::Sea, settings.sea_lvl),
			(TileKind::Sea, settings.deep_sea_lvl),
			(TileKind::DeepSea, settings.sea_floor_lvl),
			(TileKind::DeepSea, f32::NEG_INFINITY),
		];

		let mut kind = None;
		let mut upper_kind = boundaries[0].0;
		let mut upper_bound = f32::INFINITY;
		let mut color = vec4(0., 0., 0., 0.);

		for (tile_kind, boundary) in boundaries {
			let lower_color: Vector4<f32> = tile_kind.color().into();
			let upper_color: Vector4<f32> = upper_kind.color().into();
			if height > boundary {
				kind = Some(tile_kind);

				let t = (height - boundary) / (upper_bound - boundary);
				let t = t.max(0.).min(1.);

				color = upper_color * t + lower_color * (1. - t);

				break;
			} else {
				upper_bound = boundary;
				upper_kind = tile_kind;
			}
		}

		let kind = kind.unwrap();
		let tint = (color / 255.).into();

		Self { height, kind, tint }
	}

	pub fn instance(&self, external: &External) -> Instance {
		Instance {
			color_tint: self.tint,
			..external.instance(Texture::Flat)
		}
		.scale(Tile::SIZE)
	}
}
