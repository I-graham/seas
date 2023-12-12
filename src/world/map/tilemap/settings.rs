#[derive(Clone, Copy)]
pub struct TileMapSettings {
	pub seed: u32,
	pub sea_level: f64,
	pub height_pow: f64,
	pub scale: f64, 
}

impl Default for TileMapSettings {
	fn default() -> Self {
		Self {
			seed: rand::random(),
			sea_level: 0.35,
			height_pow: 0.75,
			scale: 1.,
		}
	}
}
