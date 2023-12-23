#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TileKind {
	Land,
	Shore,
	Sea,
	DeepSea,
	Wood,
}

impl TileKind {
	pub fn color(&self) -> (f32, f32, f32, f32) {
		use TileKind::*;
		match self {
			Land => (10., 109., 70., 255.),
			Shore => (230., 210., 75., 255.),
			Sea => (57., 120., 168., 255.),
			DeepSea => (15., 50., 70., 255.),
			_ => (255., 255., 255., 255.), //Otherwise untinted
		}
	}
}
