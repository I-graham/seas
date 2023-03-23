use super::world::World;
use super::GameObject;
use crate::window::WinApi;
use winit::event::VirtualKeyCode;
use winit::event_loop::EventLoop;

pub struct GameState {
	pub(super) api: WinApi,
	world: World,
}

impl GameState {
	pub(super) fn update(&mut self) {
		self.world.update(&self.api.context, &self.api.input);

		self.api.input.update_mouse();

		self.api.context.camera.pos.0 += 0.05
			* (self.api.input.key(VirtualKeyCode::D) as i32
				- self.api.input.key(VirtualKeyCode::A) as i32) as f32;

		self.api.context.camera.pos.1 += 0.05
			* (self.api.input.key(VirtualKeyCode::W) as i32
				- self.api.input.key(VirtualKeyCode::S) as i32) as f32;

		self.api.context.camera.scale += 0.05
			* (self.api.input.key(VirtualKeyCode::Q) as i32
				- self.api.input.key(VirtualKeyCode::Z) as i32) as f32;
	}

	pub(super) fn draw(&mut self) {
		let now = std::time::Instant::now();

		self.api.clear();

		self.world
			.render(&self.api.context, &mut self.api.output, now);

		self.api.draw();
	}

	pub(super) fn new(event_loop: &EventLoop<()>) -> Self {
		let api = WinApi::new(event_loop);
		Self {
			world: World::new(&api.context),
			api,
		}
	}
}
