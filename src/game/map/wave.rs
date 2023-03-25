use super::*;
use crate::{game::rand_in, window::Animation};

pub struct Wave {
	pos: (f32, f32),
	animation: Animation,
}

impl Wave {
	pub fn spawn_into(external: &External, out: &mut Vec<Wave>) {
		let (px, py) = external.camera.pos;

		const WAVE_PROB: f32 = 1. / 200.;
		let (vw, vh) = external.view_dims();
		if random() < WAVE_PROB * external.delta * vw * vh {
			out.push(Wave {
				pos: (px + rand_in(-vw, vw), py + rand_in(-vh, vh)),
				animation: Animation::new(Texture::Wave, 3., Animation::SIN_BOUNCE, Some(1.0)),
			});
		}
	}
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
		external.emit(
			out,
			Instance {
				position: self.pos.into(),
				..self.animation.frame(external)
			},
		)
	}
}
