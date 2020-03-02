use super::super::{TransformValuesExt, IdentityExt};

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

const VECTOR_IDENTITY:[f64;3] = [0.0, 0.0, 0.0];

impl TransformValuesExt for Vec3 {
    fn len(&self) -> usize { 3 }
    fn static_default() -> &'static [f64] {
        &VECTOR_IDENTITY
    }
    fn write_to_vf32(self: &Self, target:&mut [f32]) {
        //can't memcpy since it needs a cast
        target[0] = self.x as f32;
        target[1] = self.y as f32;
        target[2] = self.z as f32;
    }
    fn from_slice(values:&[f64]) -> Self {
        Self::new(values[0], values[1], values[2])
    }
}

impl IdentityExt for Vec3 {
    fn identity() -> Self {
        Self::new( VECTOR_IDENTITY[0], VECTOR_IDENTITY[1], VECTOR_IDENTITY[2])
    }
}