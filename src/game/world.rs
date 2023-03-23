use super::{map::*, Action, GameObject, Instance};

use crate::window::Context;
use std::time::Instant;

pub(super) struct World {
	map: Map,
}

impl World {
	pub fn new(context: &Context) -> Self {
		Self {
			map: Map::new(context, 50),
		}
	}
}

impl GameObject for World {
	fn update(&mut self, _context: &Context, _input: &crate::window::Input) -> super::Action {
		Action::Nothing
	}

	fn render(&self, context: &Context, out: &mut Vec<Instance>, now: Instant) {
		self.map.render(context, out, now);
	}
}
