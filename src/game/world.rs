use super::{ship::*, Action, Camera, GameObject};

use crate::window::{Context, Instance};
use std::time::Instant;

pub(super) struct World {
	ship: Ship,
}

impl World {
	pub fn new(context: &Context) -> Self {
		Self {
			ship: Ship::new(context),
		}
	}
}

impl GameObject for World {
	fn update(&mut self, input: &crate::window::Input) -> super::Action {
		self.ship.update(input);
		Action::Nothing
	}

	fn render(&mut self, context: &Context, view: &Camera, out: &mut Vec<Instance>, now: Instant) {
		self.ship.render(context, view, out, now);
	}
}
