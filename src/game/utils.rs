pub fn rand_dir() -> f32 {
	(random() - 0.5).signum()
}

pub fn probability(p: f32) -> bool {
	random() < p
}

pub fn random() -> f32 {
	rand_in(0., 1.)
}

pub fn rand_in(lo: f32, hi: f32) -> f32 {
	lo + (hi - lo) * rand::random::<f32>()
}

pub fn snap_to_grid((px, py): (f32, f32), (cellx, celly): (f32, f32)) -> (f32, f32) {
	(px - px % cellx, py - py % celly)
}
