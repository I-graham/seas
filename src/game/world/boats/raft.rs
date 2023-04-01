use crate::game::*;
use crate::window::Texture;
use cgmath::*;
use std::cell::Cell;

pub struct Raft {
	pub pos: Vector2<f32>,
	pub vel: Vector2<f32>,
	pub acc: Cell<Vector2<f32>>,
}

impl Raft {
	const SIZE: (f32, f32) = (32., 32.);
	const ACCELERATION: f32 = 0.05;
	const TOP_SPEED: f32 = 100.0;

	pub fn new() -> Self {
		Self {
			pos: vec2(0., 0.),
			vel: vec2(0., 0.),
			acc: Cell::new(vec2(0., 0.)),
		}
	}
}

impl GameObject for Raft {
	fn plan(&self, _world: &World, _external: &External, input: &Input) {
		let [w, a, s, d] = {
			use winit::event::VirtualKeyCode::*;
			[W, A, S, D].map(|k| if input.key(k) { 1f32 } else { -1f32 })
		};

		let acc = vec2(d - a, w - s);

		self.acc.set(acc);
	}

	fn update(&mut self, external: &External) -> Option<Action> {
		self.pos += self.vel * external.delta;
		self.vel += self.acc.get() * external.delta;
		None
	}

	fn instance(&self, external: &External) -> Option<Instance> {
		Some(Instance {
			scale: Self::SIZE.into(),
			position: self.pos.into(),
			..external.instance(Texture::Raft)
		})
	}
}
