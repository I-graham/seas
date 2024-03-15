use super::*;
use crate::eng::*;
use crate::window::*;

enum UIAction {
	Routing(Route),
}

pub struct WorldUI {
	action: Option<UIAction>,
}

impl WorldUI {
	pub fn new() -> Self {
		Self { action: None }
	}
}

impl GameObject for WorldUI {
	type Scene = World;
	type Action = ();

	fn update(
		&mut self,
		external: &External,
		_messenger: &Messenger<Signal>,
	) -> Option<Self::Action> {
		use UIAction::*;
		match &mut self.action {
			Some(Routing(route)) => {
				if external.left_mouse.pressed() {
					let mouse_pos = external.mouse_pos;
					let pos = external.camera.screen_to_world(mouse_pos);

					route.add_waypoint(pos);
				}

				use winit::event::VirtualKeyCode::*;
				if external.key(R).pressed() {
					self.action = None;
				}
			}
			None => {
				use winit::event::VirtualKeyCode::*;
				if external.key(R).pressed() {
					self.action = Some(Routing(Route::new()));
				}
			}
		}
		None
	}

	fn render(&self, win: &mut Window) {
		use UIAction::*;

		match &self.action {
			Some(Routing(route)) => route.render(win),
			_ => (),
		}
	}
}
