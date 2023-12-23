use noise::*;

pub struct NoiseFn(Inner);

type Inner = BasicMulti<OpenSimplex>;

impl NoiseFn {
	pub fn init(seed: u32) -> Self {
		Self(Inner::default().set_seed(seed))
	}

	pub fn get(&self, pos: [f64; 2]) -> f64 {
		use noise::NoiseFn;
		self.0.get(pos)
	}
}
