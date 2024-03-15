use super::*;
use crate::eng::*;
use crate::window::*;
use cgmath::*;

pub struct Raft {
	pub pos: Vector2<f32>,
	pub vel: Vector2<f32>,
}

impl Raft {
	const ACCELERATION: f32 = 20.;
	const TOP_SPEED: f32 = 500.0;

	pub fn new() -> Self {
		Self {
			pos: vec2(0., 0.),
			vel: vec2(0., 0.),
		}
	}
}

impl GameObject for Raft {
	type Scene = World;
	type Action = ();

	fn plan(&self, _world: &World, _external: &External, messenger: &Sender<Dispatch<Signal>>) {
		messenger
			.send(Dispatch::local(self.pos.into(), Signal::BoatNearby, 0.))
			.expect("???");
	}

	fn update(
		&mut self,
		external: &External,
		_messenger: &Messenger<Signal>,
	) -> Option<Self::Action> {
		let [w, a, s, d] = {
			use winit::event::VirtualKeyCode::*;
			[W, A, S, D].map(|k| {
				if external.key(k).is_down() {
					1f32
				} else {
					-1f32
				}
			})
		};

		let acc = Self::ACCELERATION * vec2(d - a, w - s);

		self.pos += self.vel * external.delta;
		self.vel += acc * external.delta;

		if self.vel.magnitude2() > Self::TOP_SPEED.powi(2) {
			self.vel.normalize_to(Self::TOP_SPEED);
		}

		None
	}

	fn instance(&self, external: &External) -> Option<Instance> {
		Some(Instance {
			position: self.pos.into(),
			..external.instance(Texture::Raft)
		})
	}
}

impl Griddable for Raft {
	fn pos(&self) -> (f32, f32) {
		self.pos.into()
	}
}
