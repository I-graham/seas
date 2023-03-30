use super::*;
use crate::{game::*, window::Animation};

pub struct Puffin {
	source: (f32, f32),
	heading: (f32, f32),
	flipped: bool,
	animation: Animation,
}

use Texture::*;
impl Automaton for Puffin {
	type State = Texture;

	fn update(&mut self, external: &External) -> Option<Action> {
		if !external.visible(self.instance(external)) && !external.point_in_view(self.heading) {
			Some(Action::Die)
		} else {
			None
		}
	}

	fn next_state(&self, external: &External) -> Self::State {
		if self.animation.finished(external.now) {
			if self.state() == PuffinFly {
				if self.heading != self.source {
					PuffinFlap
				} else {
					Puffin
				}
			} else {
				self.by_probability(&[
					(Puffin, 0.90),
					(PuffinFlip, 0.045),
					(PuffinPeck, 0.045),
					(PuffinFly, 0.01),
				])
			}
		} else if self.state() == PuffinFlap && self.position(external) == self.heading {
			PuffinFly
		} else {
			self.state()
		}
	}

	fn enter_from(&mut self, old: Self::State) {
		if probability(0.05) && ![PuffinFly, PuffinFlap].contains(&self.state()) {
			self.flipped = !self.flipped;
		}

		use Texture::*;
		let mut reps = Some(1.);
		let (duration, curve) = match self.state() {
			Puffin => (3., Animation::LINEAR),
			PuffinFlip => (rand_in(1., 6.), Animation::LAST),
			PuffinPeck => (0.5, Animation::LINEAR),
			PuffinFly if old == PuffinFlap => {
				self.source = self.heading;
				(0.65, Animation::REV_SIN_SQ)
			}
			PuffinFly => {
				let (sx, sy) = self.source;
				const FLEE: f32 = 80.0;
				self.heading = snap_to_grid(
					(sx + rand_in(-FLEE, FLEE), sy + rand_in(-FLEE, FLEE)),
					Self::SPRITE_SIZE,
				);

				self.flipped = self.heading.0 > self.source.0;

				(0.65, Animation::SIN_SQ)
			}
			PuffinFlap => {
				reps = None;
				(1., Animation::SIN_BOUNCE)
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

	fn render(&self, external: &External, out: &mut Vec<Instance>) {
		external.clip(out, self.instance(external))
	}
}

impl Puffin {
	const SPRITE_SIZE: (f32, f32) = (32., 16.);

	pub fn maybe_spawn(external: &External) -> Option<Self> {
		const PUFFIN_DENSITY: f32 = 1. / 80_000.;

		let (px, py) = external.camera.pos;
		let (dw, dh) = external.view_dims();
		let (vw, vh) = (dw / 2., dh / 2.);

		if probability(PUFFIN_DENSITY * external.delta * vw * vh) {
			let (hx, hy) = snap_to_grid(
				(px + rand_in(-vw, vw), py + rand_in(-vh, vh)),
				Self::SPRITE_SIZE,
			);

			let (sx, sy) = (hx + dw * rand_dir(), hy + dh * rand_dir());

			Some(Self {
				source: (sx, sy),
				heading: (hx, hy),
				flipped: sx < hx,
				animation: Animation::new(Texture::PuffinFlap, 1., Animation::SIN_BOUNCE, None),
			})
		} else {
			None
		}
	}

	fn position(&self, external: &External) -> (f32, f32) {
		if self.state() == PuffinFlap {
			let (sx, sy) = self.source;
			let (hx, hy) = self.heading;

			let dist = (sx - hx).hypot(sy - hy);

			const PUFFIN_SPEED: f32 = 60.0;
			let total_time = dist / PUFFIN_SPEED;

			let t = (self.animation.age(external.now) / total_time).min(1.);

			let lerp = |a, b| (1. - t) * a + t * b;

			(lerp(sx, hx), lerp(sy, hy))
		} else {
			self.source
		}
	}

	fn instance(&self, external: &External) -> Instance {
		Instance {
			position: self.position(external).into(),
			..self.animation.frame(external)
		}
		.scale(if self.flipped { -1. } else { 1. }, 1.)
	}
}
