use crate::eng::*;
use crate::window::*;
use crate::world::Signal;
use crate::world::{Texture, World};
use cgmath::*;

pub struct Puffin {
	source: Vector2<i32>,
	heading: Vector2<i32>,
	flipped: bool,
	animation: Animation<Texture>,
	scared_of: Option<Vector2<f32>>,
}

#[derive(PartialEq, Copy, Clone)]
pub enum Action {
	Die,
}

use Texture::*;
impl Puffin {
	const SPOT_DIMS: (f32, f32) = (32., 16.);
	const DENSITY: f32 = 1. / 800_000.;
	const FLEE_DIST: f32 = 320.;
	const SPEED: f32 = 60.0;
	const SCARE_DIST: f32 = 60.0;

	pub fn maybe_spawn(external: &External) -> Option<Self> {
		let v = external.view_dims() / 2.;

		if probability(Self::DENSITY * external.delta * v.x * v.y) {
			let pos = external.camera.pos;

			let offset = v.map(|f| rand_in(-f, f));
			let heading = snap_to_grid(pos + offset, Self::SPOT_DIMS);

			let signum = offset.map(f32::signum);

			let source = snap_to_grid(
				heading.cast::<f32>().unwrap() + v.mul_element_wise(signum),
				Self::SPOT_DIMS,
			);

			Some(Self {
				source,
				heading,
				flipped: source.x < heading.x,
				animation: Animation::new(
					Texture::PuffinFlap,
					1.,
					curves::SIN_BOUNCE,
					f32::INFINITY,
				),
				scared_of: None,
			})
		} else {
			None
		}
	}

	fn position(&self, external: &External) -> Vector2<f32> {
		let fsource = self.source.cast::<f32>().unwrap();
		let fheading = self.heading.cast::<f32>().unwrap();

		if self.state() == PuffinFlap {
			let dist = fsource.distance(fheading);

			let total_time = dist / Self::SPEED;

			let t = (self.animation.age(external.now) / total_time).min(1.);

			(1. - t) * fsource + t * fheading
		} else {
			fsource
		}
	}
}

impl Automaton for Puffin {
	type FsmScene = World;
	type FsmAction = Action;
	type State = Texture;

	fn fsm_update(
		&mut self,
		external: &External,
		messenger: &Messenger<Signal>,
	) -> Option<Self::FsmAction> {
		use Signal::*;
		type SignalTy = <Signal as SignalType>::SignalKinds;

		let destination = self.heading.cast::<f32>().unwrap();
		for message in messenger.local_receive(
			destination.into(),
			Self::SCARE_DIST,
			&[SignalTy::BoatNearby],
		) {
			match message {
				(pos, BoatNearby) => {
					self.scared_of = Some(pos.into());
				}
				_ => unreachable!(),
			}
		}

		if !external.point_in_view(self.heading.cast::<f32>().unwrap())
			&& !external.visible(self.instance(external).unwrap())
		{
			Some(Action::Die)
		} else {
			None
		}
	}

	fn next_state(&self, external: &External) -> Self::State {
		let at_destination = self
			.position(external)
			.distance2(self.heading.cast::<f32>().unwrap())
			< f32::EPSILON;

		if self.state() == PuffinFlap && at_destination
			|| self.scared_of.is_some() && [Puffin, PuffinPeck, PuffinFlip].contains(&self.state())
		{
			PuffinFly
		} else if self.animation.finished(external.now) {
			if self.state() == PuffinFly {
				if at_destination {
					Puffin
				} else {
					PuffinFlap
				}
			} else {
				self.by_probability(&[
					(Puffin, 0.90),
					(PuffinFlip, 0.045),
					(PuffinPeck, 0.045),
					(PuffinFly, 0.01),
				])
			}
		} else {
			self.state()
		}
	}

	fn enter_from(&mut self, old: Self::State) {
		if probability(0.05) && ![PuffinFly, PuffinFlap].contains(&old) {
			self.flipped = !self.flipped;
		}

		use Texture::*;
		let mut reps = 1.;
		let (duration, curve) = match self.state() {
			Puffin => (rand_in(1., 6.), curves::FIRST),
			PuffinFlip => (rand_in(1., 6.), curves::FIRST),
			PuffinPeck => (0.65, curves::LINEAR),
			PuffinFly if old == PuffinFlap => {
				self.source = self.heading;
				(0.65, curves::REV_SIN_SQ)
			}
			PuffinFly => {
				//Find new home
				match self.scared_of {
					Some(pos) => {
						let current = self.source.cast::<f32>().unwrap();
						let dir = (current - pos).normalize();
						self.heading =
							snap_to_grid(current + Self::FLEE_DIST * dir, Self::SPOT_DIMS);
					}
					None =>
					//Different x values to avoid unrealistic movement.
					{
						while self.heading.x == self.source.x {
							self.heading = snap_to_grid(
								self.source.cast::<f32>().unwrap()
									+ rand_in2d(-Self::FLEE_DIST, Self::FLEE_DIST),
								Self::SPOT_DIMS,
							);
						}
					}
				}

				self.flipped = self.heading.x > self.source.x;

				(0.65, curves::SIN_SQ)
			}
			PuffinFlap => {
				reps = f32::INFINITY;
				(0.65, curves::SIN_BOUNCE)
			}
			_ => unreachable!(),
		};

		self.animation = Animation::new(self.state(), duration, curve, reps);
	}

	fn state(&self) -> Self::State {
		self.animation.texture
	}

	fn state_mut(&mut self) -> &mut Self::State {
		&mut self.animation.texture
	}

	fn fsm_instance(&self, external: &External) -> Option<Instance> {
		let instance = Instance {
			position: self.position(external).into(),
			..self.animation.frame(external)
		}
		.scale2(if self.flipped { -1. } else { 1. }, 1.);

		Some(instance)
	}
}
