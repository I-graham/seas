use noise::*;

type WaterGen = BasicMulti<OpenSimplex>;
type IslandGen = BasicMulti<OpenSimplex>;

pub struct Generator {
	water_gen: WaterGen,
	island_gen: IslandGen,
}

impl Generator {
	const TAPER_SCALE: f32 = 2.5;
	const TAPER_POW: f32 = 3.;
	const WAVE_AMPLITUDE: f32 = 0.3;
	const WAVE_BIAS: f32 = 0.25;

	pub fn init(seed: u32) -> Self {
		let water_gen = WaterGen::default().set_seed(seed);
		let island_gen = IslandGen::default().set_seed(seed + 1);

		Self {
			water_gen,
			island_gen,
		}
	}

	pub fn read(&self, pos: [f32; 2]) -> f32 {
		let [x, y] = pos;

		let water_reading = self.water_gen.get([x as f64, y as f64]) as f32;
		let waves = Self::WAVE_AMPLITUDE * water_reading - Self::WAVE_BIAS;

		let island_reading = self.island_gen.get([x as f64, y as f64]) as f32;
		let unclamped_taper = Self::TAPER_SCALE / (1. + x.hypot(y));
		let taper = unclamped_taper.min(1.0).powf(Self::TAPER_POW);

		(1. - taper) * waves + taper * island_reading
	}
}
