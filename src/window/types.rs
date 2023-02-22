#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Uniform {
    pub ortho: cgmath::Matrix4<f32>,
}

#[repr(C, align(16))]
#[derive(Copy, Clone, Debug)]
pub struct Instance2D {
    pub color_tint: GLvec4,
    pub texture: GLvec4,
    pub scale: GLvec2,
    pub translate: GLvec2,
    pub rotation: GLfloat,
    pub z_coord: GLfloat,
}

impl Instance2D {
    pub fn scale(&self, r: f32) -> Self {
        Self {
            scale: GLvec2(r * self.scale.0, r * self.scale.1),
            ..*self
        }
    }
}

impl Default for Instance2D {
    fn default() -> Self {
        Instance2D {
            color_tint: GLvec4(1.0, 1.0, 1.0, 1.0),
            texture: GLvec4(0.0, 0.0, 1.0, 1.0),
            scale: GLvec2(1.0, 1.0),
            translate: GLvec2(0.0, 0.0),
            rotation: GLfloat(0.0),
            z_coord: GLfloat(0.0),
        }
    }
}

pub struct Camera {
    pub pos: (f32, f32),
    pub scale: f32,
}

impl Camera {
    pub fn proj(&self, aspect: f32) -> cgmath::Matrix4<f32> {
        cgmath::ortho(
            -aspect * self.scale + self.pos.0,
            aspect * self.scale + self.pos.0,
            -self.scale + self.pos.1,
            self.scale + self.pos.1,
            -100.,
            100.,
        )
    }

    pub fn to_world_coords(&self, (x, y): (f32, f32)) -> (f32, f32) {
        (self.scale * x + self.pos.0, self.scale * y + self.pos.1)
    }
}

#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GLint(pub i32);

#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GLfloat(pub f32);

impl From<f32> for GLfloat {
    fn from(f: f32) -> Self {
        GLfloat(f)
    }
}

#[repr(C, align(8))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GLvec2(pub f32, pub f32);

impl From<(f32, f32)> for GLvec2 {
    fn from((f1, f2): (f32, f32)) -> Self {
        GLvec2(f1, f2)
    }
}

#[repr(C, align(16))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GLvec3(pub f32, pub f32, pub f32);

impl From<(f32, f32, f32)> for GLvec3 {
    fn from((f1, f2, f3): (f32, f32, f32)) -> Self {
        GLvec3(f1, f2, f3)
    }
}

#[repr(C, align(16))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GLvec4(pub f32, pub f32, pub f32, pub f32);

impl From<(f32, f32, f32, f32)> for GLvec4 {
    fn from((f1, f2, f3, f4): (f32, f32, f32, f32)) -> Self {
        GLvec4(f1, f2, f3, f4)
    }
}

#[repr(C, align(16))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GLdvec4(pub f64, pub f64, pub f64, pub f64);

impl From<(f64, f64, f64, f64)> for GLdvec4 {
    fn from((f1, f2, f3, f4): (f64, f64, f64, f64)) -> Self {
        GLdvec4(f1, f2, f3, f4)
    }
}
