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
	pub dock_prob: f32,
	pub dock_depth: f32,
}

impl Default for TileMapSettings {
	fn default() -> Self {
		Self {
			seed: rand::random(),
			height_pow: 1.,
			scale: 3000.,
			land_lvl: 0.20,
			shore_lvl: 0.15,
			sea_lvl: 0.10,
			deep_sea_lvl: 0.,
			sea_floor_lvl: -0.3,
			dock_prob: 0.10,
			dock_depth: 0.025,
		}
	}
}
