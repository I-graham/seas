use super::*;
use crate::window::GLvec4;

#[derive(Debug)]
pub struct Tile {
	pub kind: TileKind,
	pub height: f32,
	pub color: GLvec4,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TileKind {
	Land,
	Shore,
	Sea,
	DeepSea,
}

impl Tile {
	pub const SIZE: f32 = 32.;

	pub fn generate(settings: &TileMapSettings, reading: f32) -> Self {
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
		let color = color.into();

		Self {
			height,
			kind,
			color,
		}
	}
}

impl TileKind {
	pub fn color(&self) -> (f32, f32, f32, f32) {
		use TileKind::*;
		match self {
			Land => (10., 109., 70., 255.),
			Shore => (230., 210., 75., 255.),
			Sea => (57., 120., 168., 255.),
			DeepSea => (15., 50., 70., 255.),
		}
	}
}
