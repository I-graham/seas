use super::*;
use cgmath::*;

pub enum UIAction {
	Route(GridId, Path), //boat id & path
	Place(Vector2<f32>),
}

impl UIAction {
	pub fn finish(mut self) -> Self {
		use UIAction::*;
		match &mut self {
			Route(_, path) => path.finish(),
			Place(_) => (),
		};
		self
	}
}
