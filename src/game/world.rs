use crate::window::{Animation, Instance, Renderable, Texture, TextureMap};
use std::time::Instant;

pub(super) struct World {
	animation: Animation,
}

impl World {
	pub fn new() -> Self {
		Self {
			animation: Animation::new(Texture::ShipSheet, 5.0),
		}
	}
}

impl Renderable for World {
	fn render(&mut self, text_map: &TextureMap, out: &mut Vec<Instance>, now: Instant) {
		self.animation.render(text_map, out, now);
	}
}
