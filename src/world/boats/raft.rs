use super::*;
use crate::eng::*;
use crate::window::*;
use cgmath::*;

pub struct Raft {
	pub pos: Vector2<f32>,
	pub dir: Vector2<f32>,
	pub route: Option<(usize, Route)>,
}

impl Raft {
	const SPEED: f32 = 300.0;
	const TURN_SPEED: f32 = 60. * std::f32::consts::TAU / 360.;

	const WAYPOINT_TOLERANCE: f32 = 100.;

	pub fn new() -> Self {
		Self {
			pos: vec2(0., 0.),
			dir: vec2(0., 1.),
			route: None,
		}
	}

	pub fn route(&mut self, route: Route) {
		self.route = Some((0, route))
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
		let (wpi, route) = self.route.as_ref()?;

		if *wpi >= route.nodes.len() {
			self.route.take();
			self.pos += Self::SPEED * external.delta * self.dir;

			return None;
		}

		let destination = route.nodes[*wpi];
		let desired_dir = destination - self.pos;

		if desired_dir.magnitude() < Self::WAYPOINT_TOLERANCE {
			//Move on to next waypoint
			self.route.as_mut().unwrap().0 += 1;
		}

		//gradual turning
		let curr_ang = angle(self.dir);
		let Rad(diff) = desired_dir.angle(self.dir);
		let new_ang = curr_ang + Self::TURN_SPEED * diff * external.delta;

		self.dir = unit_in_dir(new_ang);

		self.pos += Self::SPEED * external.delta * self.dir;

		None
	}

	fn render(&self, win: &mut Window) {
		if let Some((_, route)) = self.route.as_ref() {
			route.render(win);
		}

		win.queue(Instance {
			position: self.pos.into(),
			rotation: angle(self.dir).to_degrees().into(),
			..win.external().instance(Texture::Raft)
		});
	}
}

impl Griddable for Raft {
	fn pos(&self) -> (f32, f32) {
		self.pos.into()
	}
}
