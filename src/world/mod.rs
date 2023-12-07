mod boats;
mod map;
mod signal;
mod texture;

use boats::*;
use map::*;

pub use super::eng::*;
use crate::window::*;
pub use signal::Signal;
pub use texture::Texture;

pub struct World {
	pub map: Map,
	pub raft: Raft,
}

const MAP_SIZE: u32 = 500 * 32;
impl Root for World {
	type Texture = Texture;
	type Signal = Signal;

	fn init(_external: &External) -> Self {
		Self {
			map: Map::new(MAP_SIZE),
			raft: Raft::new(),
		}
	}

	fn camera(&self, inputs: &External) -> Camera {
		use winit::event::VirtualKeyCode;
		const CAM_MOVE_SPEED: f32 = 50.;

		let mut camera = inputs.camera;

		camera.pos.x += CAM_MOVE_SPEED
			* inputs.delta
			* (inputs.key(VirtualKeyCode::D).is_down() as i32
				- inputs.key(VirtualKeyCode::A).is_down() as i32) as f32;

		camera.pos.y += CAM_MOVE_SPEED
			* inputs.delta
			* (inputs.key(VirtualKeyCode::W).is_down() as i32
				- inputs.key(VirtualKeyCode::S).is_down() as i32) as f32;

		const CAM_SCALE_SPEED: f32 = 50.;

		camera.scale += CAM_SCALE_SPEED
			* inputs.delta
			* (inputs.key(VirtualKeyCode::Q).is_down() as i32
				- inputs.key(VirtualKeyCode::Z).is_down() as i32) as f32;

		camera
	}

	fn plan(&self, external: &External, messenger: &Sender<Dispatch<Signal>>) {
		self.map.plan(self, external, messenger);
		self.raft.plan(self, external, messenger);
	}

	fn update(&mut self, external: &External, messenger: &Messenger<Signal>) {
		self.map.update(external, messenger);
		self.raft.update(external, messenger);
	}

	fn render(&self, win: &mut Window) {
		self.map.render(win);
		self.raft.render(win);
	}
}
