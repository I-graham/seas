use crate::window::{Animation, Context, Instance, Texture};
use std::time::Instant;

use super::GameObject;

pub struct Ship {
	tile: Animation,
}

impl Ship {
	pub fn new(context: &Context) -> Self {
		Self {
			tile: Animation::new(context, Texture::Flat, 3.0, Animation::LINEAR, Some(1.0)),
		}
	}
}

impl GameObject for Ship {
	fn render(&self, context: &Context, out: &mut Vec<Instance>, now: Instant) {
		context.emit(out, self.tile.frame(now));
	}
}
