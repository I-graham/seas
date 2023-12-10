use super::*;
use crate::window::TextureType;
use std::time::Instant;

pub type Curve = fn(f32) -> f32;

pub mod curves {
	use super::Curve;
	use std::f32::consts::*;

	pub const LINEAR: Curve = |f| f;
	pub const FIRST: Curve = |_| 0.;
	pub const LAST: Curve = |_| 1.;
	pub const REVERSE: Curve = |f| 1. - f;
	pub const SIN: Curve = |f| (1. - (f * PI).cos()) / 2.;
	pub const SIN_SQ: Curve = |f| SIN(f).powf(2.);
	pub const REV_SIN_SQ: Curve = |f| SIN(1.0 - f).powf(2.);
	pub const SIN_BOUNCE: Curve = |f| SIN(2. * f);
}

#[derive(Clone)]
pub struct Animation<Texture: TextureType> {
	pub start: Instant,
	pub texture: Texture,
	pub duration: f32,
	pub curve: Curve,
	pub repeat: f32, //Use f32::INFINITY to repeat forever
}

impl<Texture: TextureType> Animation<Texture> {
	pub fn new(texture: Texture, duration: f32, curve: fn(f32) -> f32, repeat: f32) -> Self {
		Self {
			start: Instant::now(),
			texture,
			duration,
			curve,
			repeat,
		}
	}

	pub fn frame(&self, external: &External) -> Instance {
		let elapsed = self.age(external.now);
		let frames = self.texture.frame_count();

		let reps_elapsed = elapsed / self.duration;

		let proportion = reps_elapsed.min(self.repeat) - f32::EPSILON;

		let frame = (frames as f32 * (self.curve)(proportion.fract())) as u32;

		external
			.instance(self.texture)
			.nth_frame(frame.clamp(0, frames - 1), frames)
	}

	pub fn finished(&self, now: Instant) -> bool {
		self.age(now) > self.repeat * self.duration
	}

	pub fn age(&self, now: Instant) -> f32 {
		now.duration_since(self.start).as_secs_f32()
	}

	pub fn restart(&mut self) {
		self.start = Instant::now()
	}
}
