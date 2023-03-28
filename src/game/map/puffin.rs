use super::*;
use crate::{game::*, window::Animation};

pub struct Puffin {
	pos: (f32, f32),
	animation: Animation,
}

impl StateMachine for Puffin {
	type State = Texture;

	fn next_state(&self, external: &External) -> Self::State {
		if self.animation.finished(external.now) {
			use Texture::*;
			self.by_table(&[(Puffin, 0.90), (PuffinFlip, 0.05), (PuffinPeck, 0.05)])
		} else {
			self.state()
		}
	}

	fn enter(&mut self, mut new: Self::State) {
		use Texture::*;
		let duration = match new {
			Puffin => 3.,
			PuffinFlip => 5.,
			PuffinPeck => 0.5,
			_ => unreachable!(),
		};
		self.animation = Animation::new(new, duration, Animation::LINEAR, Some(1.));
	}

	fn state(&self) -> Self::State {
		self.animation.texture
	}

	fn state_mut(&mut self) -> &mut Self::State {
		&mut self.animation.texture
	}

	fn render(&self, external: &External, out: &mut Vec<Instance>) {
		external.clip(
			out,
			Instance {
				position: self.pos.into(),
				..self.animation.frame(external)
			}
			.scale(2.),
		)
	}
}

impl Puffin {
	pub fn maybe_spawn(external: &External) -> Option<Self> {
		let (px, py) = external.camera.pos;

		const PUFFIN_DENSITY: f32 = 1. / 100_000.;
		let (dw, dh) = external.view_dims();
		let (vw, vh) = (dw / 2., dh / 2.);
		if random() < PUFFIN_DENSITY * external.delta * vw * vh {
			Some(Self {
				pos: (px + rand_in(-vw, vw), py + rand_in(-vh, vh)),
				animation: Animation::new(Texture::Puffin, 0., Animation::SIN, Some(1.0)),
			})
		} else {
			None
		}
	}
}
