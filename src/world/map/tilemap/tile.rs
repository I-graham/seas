use crate::window::GLvec4;

pub struct Tile {
	pub kind: TileKind,
	pub height: f32,
}

#[derive(Clone, Copy, PartialEq)]
pub enum TileKind {
	Land,
	Sea,
	DeepSea,
}

impl Tile {
	pub const SIZE: f32 = 16f32;

	pub fn color(&self) -> GLvec4 {
		use TileKind::*;
		match &self.kind {
			Land => GLvec4(33., 200., 132., 255.),
			Sea => GLvec4(57., 120., 168., 255.),
			DeepSea => GLvec4(15., 50., 70., 255.)
		}.rgba()
	}
}
