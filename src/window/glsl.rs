#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Uniform {
    pub ortho: cgmath::Matrix4<f32>,
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
