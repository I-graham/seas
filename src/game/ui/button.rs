use crate::game::*;
use crate::window::{Context, Instance, MouseState, Texture};

type Nop<T> = fn(&mut T);

pub struct Button<T, F1: FnMut(&mut T) = Nop<T>, F2: FnMut(&mut T) = Nop<T>> {
	pub state: T,
	pub texture: Texture,
	pub rel_pos: (f32, f32),  //Relative to corner
	pub size: (f32, f32), //Relative to screen
	pub on_click: Option<F1>,
	pub on_release: Option<F2>,
	hitbox_center: (f32, f32),
}

impl<T, F1: FnMut(&mut T), F2: FnMut(&mut T)> Button<T, F1, F2> {
	pub fn new(
		_context: &Context,
		state: T,
		texture: Texture,
		pos: (f32, f32),
		size: (f32, f32),
		on_click: Option<F1>,
		on_release: Option<F2>,
	) -> Self {
		Self {
			state,
			texture,
			rel_pos: pos,
			size,
			on_click,
			on_release,
			hitbox_center: pos,
		}
	}

	pub fn includes(&self, point: (f32, f32)) -> bool {
		point_in_rect(point, self.hitbox_center, self.size)
	}
}

impl<T> GameObject for Button<T> {
	fn update(&mut self, input: &Input) -> Action {
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

	fn render(&mut self, context: &Context, out: &mut Vec<Instance>, _now: std::time::Instant) {
		self.hitbox_center = context.corner_relative(self.rel_pos);
		out.push(Instance {
			translate: context.screen_to_world_pos(self.hitbox_center).into(),
			scale: self.size.into(),
			..context.get_inst(self.texture)
		});
	}
}

fn point_in_rect(point: (f32, f32), pos: (f32, f32), scale: (f32, f32)) -> bool {
	let (x1, x2) = (pos.0 - scale.0, pos.0 + scale.0);
	let (y1, y2) = (pos.1 - scale.1, pos.1 + scale.1);

	(x1..x2).contains(&point.0) && (y1..y2).contains(&point.1)
}
