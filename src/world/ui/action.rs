use super::*;

pub enum UIAction {
	Routing(GridId, Route), //boat & target route
}

impl UIAction {
	pub fn finish(mut self) -> Self {
		use UIAction::*;
		match &mut self {
			Routing(_, route) => route.finish(),
		};
		self
	}
}
