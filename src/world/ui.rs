use std::cell::RefCell;

use super::*;
use crate::eng::*;
use crate::window::*;

pub enum UIAction {
	Routing(GridId, Route), //boat & target route
}

pub struct WorldUI {
	action: RefCell<Option<UIAction>>,
}

impl WorldUI {
	const SELECT_RADIUS: f32 = 50.;

	pub fn new() -> Self {
		Self {
			action: None.into(),
		}
	}
}

impl GameObject for WorldUI {
	type Scene = World;
	type Action = UIAction;

	fn plan(&self, world: &World, external: &External, _messenger: &Sender<Dispatch<Signal>>) {
		use winit::event::VirtualKeyCode::*;
		use UIAction::*;

		let mut action = self.action.borrow_mut();
		let mouse = external.camera.screen_to_world(external.mouse_pos);

		match &mut *action {
			Some(Routing(_, _)) if external.key(Escape).pressed() => *action = None,
			Some(Routing(target_id, route)) => {
				let target = world.env.boats.get(*target_id).unwrap();

				route.move_first(target.pos);
				route.move_last(mouse);

				if external.left_mouse.pressed() {
					route.add_waypoint(mouse);
				}
			}
			None => {
				if external.left_mouse.pressed() {
					let mouse_pos = external.mouse_pos;
					let pos = external.camera.screen_to_world(mouse_pos);
					let target = world.env.boats.nearest(pos, Self::SELECT_RADIUS);
					if let Some((id, boat)) = target {
						let mut route = Route::new(boat.pos);
						route.add_waypoint(pos);

						*action = Some(Routing(id, route));
					}
				}
			}
		}
	}

	fn update(
		&mut self,
		external: &External,
		_messenger: &Messenger<Signal>,
	) -> Option<Self::Action> {
		use winit::event::VirtualKeyCode::*;

		if external.key(Space).pressed() {
			self.action.take()
		} else {
			None
		}
	}

	fn render(&self, win: &mut Window) {
		use UIAction::*;

		match &*self.action.borrow() {
			Some(Routing(_, route)) => route.render(win),
			_ => (),
		}
	}
}
