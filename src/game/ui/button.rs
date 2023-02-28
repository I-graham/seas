use crate::game::*;
use crate::window::{Camera, Context, Instance, MouseState, Texture};

type Nop<T> = fn(&mut T);

pub struct Button<T, F1: FnMut(&mut T) = Nop<T>, F2: FnMut(&mut T) = Nop<T>> {
	pub state: T,
	pub texture: Texture,
	pub pos: (f32, f32),  //Relative to screen
	pub size: (f32, f32), //Relative to screen	pub on_click: Option<F1>,
	pub on_click: Option<F1>,
	pub on_release: Option<F2>,
}

impl<T, F1: FnMut(&mut T), F2: FnMut(&mut T)> Button<T, F1, F2> {
	pub fn new(
		_context: &Context,
		state: T,
		texture: Texture,
		pos: (f32, f32),  //Relative to screen
		size: (f32, f32), //Relative to screen
		on_click: Option<F1>,
		on_release: Option<F2>,
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
	fn update(&mut self, input: &Input) -> Action {
		match input.left_mouse {
			MouseState::Click if self.includes(input.mouse_pos) => {
				self.on_click.map(|listener| listener(&mut self.state));
			}
			MouseState::Release if self.includes(input.mouse_pos) => {
				self.on_release.map(|listener| listener(&mut self.state));
			}
			_ => {}
		}

		Action::Nothing
	}

	fn render(
		&mut self,
		context: &Context,
		view: &Camera,
		out: &mut Vec<Instance>,
		_now: std::time::Instant,
	) {
		out.push(Instance {
			translate: view.screen_to_world_pos(self.pos, context.aspect).into(),
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
