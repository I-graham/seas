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

	pub fn contains(&self, pos: Vector2<f32>) -> bool {
		let GLvec2(x, y) = self.position;
		let center = vec2(x, y);

		let GLfloat(angle) = self.rotation;

		let s = (-angle).sin();
		let c = (-angle).cos();

		//rotated d
		let d = pos - center;
		let rx = d.x * c - d.y * s;
		let ry = d.x * s + d.y * c;

		let GLvec2(sx, sy) = self.scale;

		(x - sx..x + sx).contains(&(rx + x)) && (y - sy..y + sy).contains(&(ry + y))
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
