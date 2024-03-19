use super::*;
use crate::eng::*;
use crate::window::*;
use cgmath::*;

pub struct Raft {
	pub pos: Vector2<f32>,
	pub dir: Vector2<f32>,
	pub path: Option<(usize, Path)>,
}

impl Raft {
	const SPEED: f32 = 200.0;
	const TURN_SPEED: f32 = 60. * std::f32::consts::TAU / 360.;

	const WAYPOINT_TOLERANCE: f32 = 150.;
	const DESTINATION_TOLERANCE: f32 = 10.;

	pub fn new(pos: Vector2<f32>) -> Self {
		Self {
			pos,
			dir: vec2(0., 1.),
			path: None,
		}
	}

	pub fn follow(&mut self, path: Path) {
		self.path = Some((0, path))
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
		let (wpi, path) = self.path.as_ref()?;

		let i = *wpi;
		let n = path.nodes.len();

		if i >= path.nodes.len() {
			self.path.take();
			self.pos += Self::SPEED * external.delta * self.dir;
			return None;
		}

		let destination = path.nodes[i];
		let desired_dir = destination - self.pos;

		let distance = desired_dir.magnitude();

		//Less tolerance for last waypoint
		if (distance < Self::WAYPOINT_TOLERANCE && i < n - 1)
			|| (distance < Self::DESTINATION_TOLERANCE && i == n - 1)
		{
			//Move on to next waypoint
			self.path.as_mut().unwrap().0 += 1;
		}

		//gradual turning
		let ang = angle(self.dir);
		let Rad(diff) = desired_dir.angle(self.dir);
		let max_turn = Self::TURN_SPEED * external.delta;
		let capped_diff = diff.signum() * diff.abs().min(max_turn);

		self.dir = unit_in_dir(ang + capped_diff);
		self.pos += Self::SPEED * external.delta * self.dir;

		None
	}

	fn render(&self, win: &mut Window) {
		if let Some((_, path)) = self.path.as_ref() {
			path.render(win);
		}

		win.queue(Instance {
			position: self.pos.into(),
			rotation: GLfloat(angle(self.dir).to_degrees()),
			..win.external().instance(Texture::Raft)
		});
	}
}

impl Griddable for Raft {
	fn pos(&self) -> (f32, f32) {
		self.pos.into()
	}
}
