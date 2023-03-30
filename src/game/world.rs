use super::{map::*, GameObject, Instance};

use crate::window::External;

pub struct World {
	map: Map,
}

const MAP_SIZE: u32 = 500 * 32;
impl World {
	pub fn new() -> Self {
		Self {
			map: Map::new(MAP_SIZE),
		}
	}
}

impl GameObject for World {
	fn update(&mut self, external: &External) -> Option<super::Action> {
		self.map.update(external);
		None
	}

	fn render(&self, external: &External, out: &mut Vec<Instance>) {
		self.map.render(external, out);
	}
}
