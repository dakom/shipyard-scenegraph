use super::super::{TransformValuesExt, IdentityExt};

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

impl TransformValuesExt for Quat {
    fn len(&self) -> usize { 4 }
    fn static_default() -> &'static [f64] {
        &QUAT_IDENTITY
    }
    fn write_to_vf32(self: &Self, target:&mut [f32]) {
        //can't memcpy since it needs a cast
        target[0] = self.x as f32;
        target[1] = self.y as f32;
        target[2] = self.z as f32;
        target[3] = self.w as f32;
    }
    fn from_slice(values:&[f64]) -> Self {
        let mut _self = Self::identity();
        _self.copy_from_slice(values);
        _self
    }
}

impl IdentityExt for Quat {
    fn identity() -> Self {
        Self::new(QUAT_IDENTITY[0], QUAT_IDENTITY[1], QUAT_IDENTITY[2], QUAT_IDENTITY[3])
    }
}