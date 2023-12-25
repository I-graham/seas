use noise::*;

pub struct NoiseGenerator(Inner);

type Inner = BasicMulti<OpenSimplex>;

impl NoiseGenerator {
	const TAPER_OFF: f32 = 1. / 20.;

	pub fn init(seed: u32) -> Self {
		Self(Inner::default().set_seed(seed))
	}

	pub fn read(&self, pos: [f32; 2]) -> f32 {
		let [x, y] = pos;
		let reading = self.0.get([x as f64, y as f64]) as f32;

		let unclamped_taper = Self::TAPER_OFF * x.hypot(y).powi(3);
		let taper = unclamped_taper.clamp(0.0, 0.5);

		reading - taper
	}
}
