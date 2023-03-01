use super::world::World;
use super::GameObject;
use crate::window::{Instance, WinApi};
use winit::event_loop::EventLoop;

pub struct GameState {
	pub(super) api: WinApi,
	world: World,
	instances: Vec<Instance>,
}

impl GameState {
	pub(super) fn update(&mut self) {
		self.world.update(&self.api.input);

		self.api.input.update_mouse();
	}

	pub(super) fn draw(&mut self) {
		let now = std::time::Instant::now();

		self.instances.clear();
		self.api.clear();

		self.world
			.render(&self.api.context,&mut self.instances, now);

		self.api.draw(self.api.context.camera, &self.instances);
	}

	pub(super) fn new(event_loop: &EventLoop<()>) -> Self {
		let api = WinApi::new(event_loop);
		Self {
			instances: vec![],
			world: World::new(&api.context),
			api,
		}
	}
}
