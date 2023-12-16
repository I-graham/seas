mod boats;
mod map;
mod signal;
mod texture;

use crate::window::*;
use boats::*;
use map::*;
#[cfg(feature = "profile")]
use tracing::instrument;

pub use super::eng::*;
pub use signal::Signal;
pub use texture::Texture;

pub struct World {
	pub env: Environment,
	pub raft: Raft,
}

impl Root for World {
	type Texture = Texture;
	type Signal = Signal;

	fn init(_external: &External) -> Self {
		Self {
			env: Environment::new(),
			raft: Raft::new(),
		}
	}

	fn camera(&self, inputs: &External) -> Camera {
		let mut camera = Camera {
			pos: self.raft.pos,
			..inputs.camera
		};

		use winit::event::VirtualKeyCode;
		const CAM_SCALE_SPEED: f32 = 500.;
		camera.scale += CAM_SCALE_SPEED
			* inputs.delta
			* (inputs.key(VirtualKeyCode::Q).is_down() as i32
				- inputs.key(VirtualKeyCode::Z).is_down() as i32) as f32;

		camera
	}

	#[cfg_attr(feature = "profile", instrument(skip_all, name = "Planning World"))]
	fn plan(&self, external: &External, messenger: &Sender<Dispatch<Signal>>) {
		self.raft.plan(self, external, messenger);
		self.env.plan(self, external, messenger);
	}

	#[cfg_attr(feature = "profile", instrument(skip_all, name = "Updating World"))]
	fn update(&mut self, external: &External, messenger: &Messenger<Signal>) {
		self.raft.update(external, messenger);
		self.env.update(external, messenger);
	}

	#[cfg_attr(feature = "profile", instrument(skip_all, name = "World Rendering"))]
	fn render(&self, win: &mut Window) {
		self.env.render(win);
		self.raft.render(win);
	}

	fn cleanup(&mut self) {
		self.env.cleanup();
	}
}
