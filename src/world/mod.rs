mod boats;
mod env;
mod signal;
mod texture;
mod ui;

use crate::window::*;
use boats::*;
use env::*;

#[cfg(feature = "profile")]
use tracing::instrument;

pub use super::eng::*;
pub use signal::Signal;
pub use texture::Texture;
pub use ui::*;

pub struct World {
	pub env: Environment,
	pub ui: WorldUI,
}

impl Root for World {
	const TITLE: &'static str = "Boat Game";

	type Texture = Texture;
	type Signal = Signal;

	fn init(_external: &External) -> Self {
		Self {
			env: Environment::new(),
			ui: WorldUI::new(),
		}
	}

	fn camera(&self, inputs: &External) -> Camera {
		const CAM_SCALE_SPEED: f32 = 500.;
		const CAM_MOVE_SPEED: f32 = 500.;

		let [q, z, w, a, s, d] = {
			use winit::event::VirtualKeyCode::*;
			[Q, Z, W, A, S, D].map(|k| if inputs.key(k).is_down() { 1 } else { -1 })
		};

		let mut camera = inputs.camera;
		camera.scale += CAM_SCALE_SPEED * inputs.delta * (q - z) as f32;
		camera.pos.x += CAM_MOVE_SPEED * inputs.delta * (d - a) as f32;
		camera.pos.y += CAM_MOVE_SPEED * inputs.delta * (w - s) as f32;

		camera
	}

	#[cfg_attr(feature = "profile", instrument(skip_all, name = "Planning World"))]
	fn plan(&self, external: &External, messenger: &Sender<Dispatch<Signal>>) {
		self.env.plan(self, external, messenger);
		self.ui.plan(self, external, messenger);
	}

	#[cfg_attr(feature = "profile", instrument(skip_all, name = "Updating World"))]
	fn update(&mut self, external: &External, messenger: &Messenger<Signal>) {
		self.env.update(external, messenger);
		if let Some(action) = self.ui.update(external, messenger) {
			self.env.act(action);
		};
	}

	#[cfg_attr(feature = "profile", instrument(skip_all, name = "World Rendering"))]
	fn render(&self, win: &mut Window) {
		self.env.render(win);
		self.ui.render(win);
	}

	fn cleanup(&mut self) {
		self.env.cleanup();
	}
}
