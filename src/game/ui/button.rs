use crate::game::*;
use crate::window::{glsl::*, Context, Instance, MouseState, Texture};

pub struct Button<T> {
	pub state: T,
	pub texture: Texture,
	pub pos: (f32, f32),
	pub size: (f32, f32), //Relative to screen
	pub on_click: Option<fn(&mut T)>,
	pub on_release: Option<fn(&mut T)>,
}

impl<T> Button<T> {
	pub fn new(
		_context: &Context,
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
		}
	}

	pub fn includes(&self, point: (f32, f32)) -> bool {
		point_in_rect(point, self.pos, self.size)
	}
}

impl<T> GameObject for Button<T> {
	fn update(&mut self, _context: &Context, input: &Input) -> Action {
		match input.left_mouse {
			MouseState::Click if self.includes(input.mouse_pos) => {
				if let Some(listener) = self.on_click {
					listener(&mut self.state)
				}
			}
			MouseState::Release if self.includes(input.mouse_pos) => {
				if let Some(listener) = self.on_release {
					listener(&mut self.state)
				}
			}
			_ => {}
		}

		Action::Nothing
	}

	fn render(&self, context: &Context, out: &mut Vec<Instance>, _now: std::time::Instant) {
		context.emit(
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
