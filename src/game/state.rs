use super::world::World;
use super::{GameObject};
use crate::window::WinApi;
use winit::event::VirtualKeyCode;
use winit::event_loop::EventLoop;

pub struct GameState {
	pub(super) api: WinApi,
	world: World,
}

impl GameState {
	pub(super) fn new(event_loop: &EventLoop<()>) -> Self {
		let api = WinApi::new(event_loop);
		Self {
			world: World::new(),
			api,
		}
	}

	pub(super) fn step(&mut self) {
		self.world
			.plan(&self.world, &self.api.external, &self.api.input);

		self.world.update(&self.api.external);
		self.api.external.refresh();

		self.api.input.update_mouse();

		self.api.external.camera.pos.0 += 0.05
			* (self.api.input.key(VirtualKeyCode::D) as i32
				- self.api.input.key(VirtualKeyCode::A) as i32) as f32;

		self.api.external.camera.pos.1 += 0.05
			* (self.api.input.key(VirtualKeyCode::W) as i32
				- self.api.input.key(VirtualKeyCode::S) as i32) as f32;

		self.api.external.camera.scale += 0.05
			* (self.api.input.key(VirtualKeyCode::Q) as i32
				- self.api.input.key(VirtualKeyCode::Z) as i32) as f32;
	}

	pub(super) fn draw(&mut self) {
		self.api.clear();

		self.world
			.render(&self.api.external, &mut self.api.output);

		self.api.draw();
	}
}
