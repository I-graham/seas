#[derive(Clone, Copy)]
pub struct TileMapSettings {
	pub seed: u32,
	pub height_pow: f32,
	pub scale: f32,
	pub land_lvl: f32,
	pub shore_lvl: f32,
	pub sea_lvl: f32,
	pub deep_sea_lvl: f32,
	pub sea_floor_lvl: f32,
}

impl Default for TileMapSettings {
	fn default() -> Self {
		Self {
			seed: rand::random(),
			height_pow: 0.8,
			scale: 1024.,
			land_lvl: 0.50,
			shore_lvl: 0.40,
			sea_lvl: 0.35,
			deep_sea_lvl: 0.30,
			sea_floor_lvl: -0.5,
		}
	}
}
