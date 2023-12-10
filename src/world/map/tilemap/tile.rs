use crate::window::GLvec4;

pub struct Tile {
	pub kind: TileKind,
}

#[derive(Clone, Copy, PartialEq)]
pub enum TileKind {
	Land,
	Sea,
}

impl Tile {
	pub const SIZE: f32 = 32f32;

	pub fn color(&self) -> GLvec4 {
		use TileKind::*;
		match &self.kind {
			Sea => GLvec4(57., 120., 168., 255.),
			Land => GLvec4(33., 200., 132., 255.),
		}.rgba()
	}
}
