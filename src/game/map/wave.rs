use super::*;
use crate::{game::rand_in, window::Animation};

pub struct Wave {
	pos: (f32, f32),
	animation: Animation,
}

impl GameObject for Wave {
	fn update(&mut self, external: &External) -> Option<Action> {
		if self.animation.finished(external.now) {
			Some(Action::Die)
		} else {
			None
		}
	}

	fn render(&self, external: &External, out: &mut Vec<Instance>) {
		external.clip(
			out,
			Instance {
				position: self.pos.into(),
				..self.animation.frame(external)
			},
		)
	}
}

impl Wave {
	pub fn maybe_spawn(external: &External) -> Option<Self> {
		let (px, py) = external.camera.pos;

		const WAVE_DENSITY: f32 = 1. / 500.;
		let (dw, dh) = external.view_dims();
		let (vw, vh) = (dw / 2., dh / 2.);
		if random() < WAVE_DENSITY * external.delta * vw * vh {
			Some(Wave {
				pos: (px + rand_in(-vw, vw), py + rand_in(-vh, vh)),
				animation: Animation::new(Texture::Wave, 3., Animation::SIN, Some(1.0)),
			})
		} else {
			None
		}
	}
}
