use super::*;

use crate::window::Window;
#[cfg(feature = "profile")]
use tracing::instrument;
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
			world: World::init(api.external()),
			messenger: Messenger::new(),
			win: api,
		}
	}

	#[cfg_attr(feature = "profile", instrument(skip_all, name = "Frame"))]
	pub fn frame(&mut self) {
		self.step();
		self.draw();
		self.win.submit();
	}

	#[cfg_attr(feature = "profile", instrument(skip_all, name = "Game Step"))]
	fn step(&mut self) {
		self.world
			.plan(self.win.external(), &self.messenger.sender());
		self.world.update(self.win.external(), &self.messenger);
		self.win.external_mut().camera = self.world.camera(self.win.external());

		let now = std::time::Instant::now();
		self.win.external_mut().update(now);
		self.messenger.update(now);
	}

	#[cfg_attr(feature = "profile", instrument(skip_all, name = "Drawing"))]
	fn draw(&mut self) {
		self.win.clear();
		self.world.render(&mut self.win);
		self.win.draw();
	}

	#[cfg_attr(feature = "profile", instrument(skip_all, name = "Cleanup"))]
	pub fn cleanup(&mut self) {
		self.world.cleanup();
		self.win.clean_cache();
	}
}
