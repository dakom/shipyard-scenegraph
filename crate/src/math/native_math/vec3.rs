use crate::math::traits::{MathContainer, FromSliceExt, VectorExt};

#[repr(C)]
#[derive(Clone, PartialEq, Debug)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self{x, y, z}
    }
}

const VECTOR_ZERO:[f64;3] = [0.0, 0.0, 0.0];

impl From<&[f64]> for Vec3 {
    fn from(values:&[f64]) -> Self {
        Self::new(values[0], values[1], values[2])
    }
}

impl <'a> FromSliceExt<'a> for Vec3 {
    fn from_slice(values:&'a [f64]) -> Self {
        values.into()
    }
}

impl <'a> MathContainer<'a> for Vec3 {
    fn len(&self) -> usize { 3 }
    fn write_to_vf32(self: &Self, target:&mut [f32]) {
        //can't memcpy since it needs a cast
        target[0] = self.x as f32;
        target[1] = self.y as f32;
        target[2] = self.z as f32;
    }
}

impl VectorExt for Vec3 {
    fn zero() -> Self {
        Vec3::from_slice(&VECTOR_ZERO)
    }
}