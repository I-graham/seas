use super::*;

use crate::window::Window;
use winit::event_loop::EventLoop;

pub struct GameState<World: Root> {
	pub(super) win: Window,
	messenger: Messenger<World::Signal>,
	world: World,
}

impl<World: Root> GameState<World> {
	pub fn new(event_loop: &EventLoop<()>) -> Self {
		let api = Window::new::<World::Texture>(event_loop);
		Self {
			world: World::init(api.inputs()),
			messenger: Messenger::new(),
			win: api,
		}
	}

	pub fn step(&mut self) {
		self.world.plan(self.win.inputs(), &self.messenger.sender());
		self.world.update(self.win.inputs(), &self.messenger);

		let now = std::time::Instant::now();
		self.win.inputs_mut().update(now);
		self.messenger.update(now);
		self.win.inputs_mut().camera = self.world.camera(self.win.inputs());
	}

	pub fn draw(&mut self) {
		self.win.clear();
		self.world.render(&mut self.win);
		self.win.draw();
	}

	pub fn cleanup(&mut self) {
		self.world.cleanup()
	}
}
