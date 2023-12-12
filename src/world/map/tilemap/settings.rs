#[derive(Clone, Copy)]
pub struct TileMapSettings {
	pub seed: u32,
	pub height_pow: f32,
	pub scale: f32,
	pub sea_level: f32,
	pub deep_sea_level: f32,
}

impl Default for TileMapSettings {
	fn default() -> Self {
		Self {
			seed: rand::random(),
			height_pow: 0.8,
			scale: 1024.,
			sea_level: 0.35,
			deep_sea_level: 0.15,
		}
	}
}
