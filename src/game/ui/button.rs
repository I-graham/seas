use crate::game::*;
use crate::window::{glsl::*, External, Instance, MouseState, Texture};
use std::cell::Cell;

enum ButtonPlan {
	Click,
	Release,
}

pub struct Button<T> {
	pub state: T,
	pub texture: Texture,
	pub pos: (f32, f32),
	pub size: (f32, f32), //Relative to screen
	pub on_click: Option<fn(&mut T)>,
	pub on_release: Option<fn(&mut T)>,
	plan: Cell<Option<ButtonPlan>>,
}

impl<T> Button<T> {
	pub fn new(
		_context: &External,
		state: T,
		texture: Texture,
		pos: (f32, f32),
		size: (f32, f32),
		on_click: Option<fn(&mut T)>,
		on_release: Option<fn(&mut T)>,
	) -> Self {
		Self {
			state,
			texture,
			pos,
			size,
			on_click,
			on_release,
			plan: Cell::new(None),
		}
	}

	pub fn includes(&self, point: (f32, f32)) -> bool {
		point_in_rect(point, self.pos, self.size)
	}
}

impl<T> GameObject for Button<T> {
	fn plan(&self, _world: &World, _context: &External, input: &Input) {
		self.plan.set(match input.left_mouse {
			MouseState::Click if self.includes(input.mouse_pos) => Some(ButtonPlan::Click),
			MouseState::Release if self.includes(input.mouse_pos) => Some(ButtonPlan::Release),
			_ => None,
		})
	}

	fn update(&mut self, _external: &External) -> Option<Action> {
		if let Some(plan) = self.plan.take() {
			match plan {
				ButtonPlan::Click => {
					if let Some(listener) = self.on_click {
						listener(&mut self.state)
					}
				}
				ButtonPlan::Release => {
					if let Some(listener) = self.on_release {
						listener(&mut self.state)
					}
				}
			}
		}

		None
	}

	fn render(&self, context: &External, out: &mut Vec<Instance>) {
		context.clip(
			out,
			Instance {
				position: self.pos.into(),
				scale: self.size.into(),
				screen_relative: GLbool::True,
				..context.instance(self.texture)
			},
		);
	}
}

fn point_in_rect(point: (f32, f32), pos: (f32, f32), scale: (f32, f32)) -> bool {
	let (x1, x2) = (pos.0 - scale.0, pos.0 + scale.0);
	let (y1, y2) = (pos.1 - scale.1, pos.1 + scale.1);

	(x1..x2).contains(&point.0) && (y1..y2).contains(&point.1)
}
