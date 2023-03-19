use super::{map::*, ship::*, Action, GameObject};

use crate::window::Context;
use std::time::Instant;

pub(super) struct World {
	map: Map,
	ship: Ship,
}

impl World {
	pub fn new(context: &Context) -> Self {
		Self {
			map: Map::new(context, 100),
			ship: Ship::new(context),
		}
	}
}

impl GameObject for World {
	fn update(&mut self, input: &crate::window::Input) -> super::Action {
		self.ship.update(input);
		Action::Nothing
	}

	fn render(&mut self, context: &mut Context, now: Instant) {
		self.map.render(context, now);
	}
}
