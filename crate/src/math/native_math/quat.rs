use crate::math::traits::{MathContainer, FromSliceExt, QuatExt};

#[repr(C)]
#[derive(Clone, PartialEq, Debug)]
pub struct Quat {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}
impl Quat {
    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Self {
        Self{x, y, z, w}
    }
}

const QUAT_IDENTITY:[f64;4] = [0.0, 0.0, 0.0, 1.0];

impl From<&[f64]> for Quat {
    fn from(values:&[f64]) -> Self {
        Self::new(values[0], values[1], values[2], values[3])
    }
}

impl <'a> FromSliceExt<'a> for Quat {
    fn from_slice(values:&'a [f64]) -> Self {
        values.into()
    }
}

impl <'a> MathContainer<'a> for Quat {
    fn len(&self) -> usize { 4 }
    fn write_to_vf32(self: &Self, target:&mut [f32]) {
        //can't memcpy since it needs a cast
        target[0] = self.x as f32;
        target[1] = self.y as f32;
        target[2] = self.z as f32;
        target[3] = self.w as f32;
    }
}

impl QuatExt for Quat {
    fn identity() -> Self {
        Quat::from_slice(&QUAT_IDENTITY)
    }
}