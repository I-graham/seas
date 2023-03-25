pub fn random() -> f32 {
	rand_in(0., 1.)
}

pub fn rand_in(lo: f32, hi: f32) -> f32 {
	lo + (hi - lo) * rand::random::<f32>()
}
