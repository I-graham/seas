mod action;

pub use action::*;

use super::*;
use crate::eng::*;
use crate::window::*;
use std::cell::RefCell;

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
			Some(_) if external.key(Escape).pressed() => *action = None,
			Some(Route(target_id, path)) => {
				let target = world.env.boats.get(*target_id).unwrap();

				path.move_first(target.pos);
				path.move_last(mouse);

				if external.left_mouse.pressed() {
					path.add_waypoint(mouse);
				}
			}
			Some(Place(pos)) => *pos = mouse,
			None => {
				//If left click on raft, begin routing
				if external.left_mouse.pressed() {
					let target = world.env.boats.nearest(mouse, Self::SELECT_RADIUS);
					if let Some((id, boat)) = target {
						let mut path = Path::new(boat.pos);
						path.add_waypoint(mouse);

						*action = Some(Route(id, path));
					}
				}

				//If right click, spawn raft
				if external.right_mouse.pressed() {
					*action = Some(Place(mouse));
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
		use UIAction::*;

		match self.action.get_mut() {
			Some(Place(_)) => self.action.take(),
			_ => {
				if external.key(Space).pressed() {
					self.action.take().map(|action| action.finish())
				} else {
					None
				}
			}
		}
	}

	fn render(&self, win: &mut Window) {
		use UIAction::*;

		match &*self.action.borrow() {
			Some(Route(_, path)) => path.render(win),
			_ => (),
		}
	}
}
