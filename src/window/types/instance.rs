use super::*;

#[repr(C, align(16))]
#[derive(Copy, Clone, Debug)]
pub struct Instance {
	pub color_tint: GLvec4,
	pub texture: GLvec4,
	pub scale: GLvec2,
	pub position: GLvec2,
	pub rotation: GLfloat,
	pub screen_relative: GLbool,
}

impl Instance {
	pub fn scale_rgba(self) -> Self {
		let GLvec4(ulx, uly, lrx, lry) = self.color_tint;
		Self {
			color_tint: GLvec4(ulx / 255., uly / 255., lrx / 255., lry / 255.),
			..self
		}
	}

	pub fn scale(self, r: f32) -> Self {
		self.scale2(r, r)
	}

	pub fn scale2(self, x: f32, y: f32) -> Self {
		Self {
			scale: GLvec2(x * self.scale.0, y * self.scale.1),
			..self
		}
	}

	pub fn nth_frame(self, n: u32, out_of: u32) -> Self {
		let GLvec4(ulx, uly, lrx, lry) = self.texture;
		let shift = (lry - uly) / out_of as f32;
		let starty = uly + n as f32 * shift;

		Self {
			texture: GLvec4(ulx, starty, lrx, starty + shift),
			..self
		}
	}
}

impl Default for Instance {
	fn default() -> Self {
		Instance {
			color_tint: GLvec4(1.0, 1.0, 1.0, 1.0),
			texture: GLvec4(0.0, 0.0, 1.0, 1.0),
			scale: GLvec2(1.0, 1.0),
			position: GLvec2(0.0, 0.0),
			rotation: GLfloat(0.0),
			screen_relative: GLbool::False,
		}
	}
}
