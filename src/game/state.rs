use super::world::World;
use crate::window::{Instance, WinApi};
use std::time::Instant;
use winit::event_loop::EventLoop;

pub struct GameState {
	pub(super) api: WinApi,
	now: Instant,
	world: World,
	instances: Vec<Instance>,
}

impl GameState {
	pub(super) fn update(&mut self) {
		self.api.update_mouse();
		let _delta_time = Instant::now().duration_since(self.now);
	}

	pub(super) fn draw(&mut self) {
		use crate::window::Renderable;

		let now = std::time::Instant::now();

		self.instances.clear();
		self.api.clear();

		let map = &self.api.texture_map;
		self.world.render(map, &mut self.instances, now);

		self.api.draw(&self.instances);
	}

	pub(super) fn new(event_loop: &EventLoop<()>) -> Self {
		Self {
			now: Instant::now(),
			api: WinApi::new(event_loop),
			instances: vec![],
			world: World::new(),
		}
	}
}
