use crate::traits::{required as math_traits, required::SliceExt, extra::F32Compat};
use std::convert::TryInto;
use std::ops::{Deref, DerefMut};

const QUAT_IDENTITY: [f64; 4] = [0.0, 0.0, 0.0, 1.0];

#[repr(C)]
#[derive(PartialEq, Debug)]
pub struct Vec4([f64; 4]);

impl Vec4 {
    pub fn set_x(&mut self, value: f64) {
        self.0[0] = value;
    }
    pub fn set_y(&mut self, value: f64) {
        self.0[1] = value;
    }
    pub fn set_z(&mut self, value: f64) {
        self.0[2] = value;
    }
    pub fn set_w(&mut self, value: f64) {
        self.0[3] = value;
    }
    pub fn x(&self) -> f64 {
        self.0[0]
    }
    pub fn y(&self) -> f64 {
        self.0[1]
    }
    pub fn z(&self) -> f64 {
        self.0[2]
    }
    pub fn w(&self) -> f64 {
        self.0[3]
    }

    pub fn r(&self) -> f64 {
        self.x()
    }
    pub fn g(&self) -> f64 {
        self.y()
    }
    pub fn b(&self) -> f64 {
        self.z()
    }
    pub fn a(&self) -> f64 {
        self.w()
    }
    pub fn set_r(&mut self, value: f64) {
        self.0[0] = value;
    }
    pub fn set_g(&mut self, value: f64) {
        self.0[1] = value;
    }
    pub fn set_b(&mut self, value: f64) {
        self.0[2] = value;
    }
    pub fn set_a(&mut self, value: f64) {
        self.0[3] = value;
    }

    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Self {
        Self([x, y, z, w])
    }

    pub fn quat_identity() -> Self {
        QUAT_IDENTITY.as_ref().into()
    }
}

impl F32Compat for Vec4 {
    fn write_to_vf32(&self, target: &mut [f32]) {
        //can't memcpy since it needs a cast
        target[0] = self.x() as f32;
        target[1] = self.y() as f32;
        target[2] = self.z() as f32;
        target[3] = self.w() as f32;
    }
}
impl Deref for Vec4 {
    type Target = [f64];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Vec4 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<&[f64]> for Vec4 {
    fn from(values: &[f64]) -> Self {
        let data: [f64; 4] = values.try_into().unwrap();
        Self(data)
    }
}

impl SliceExt<f64> for Vec4 {
    fn as_slice(&self) -> &[f64] {
        &self.0
    }

    fn as_slice_mut(&mut self) -> &mut [f64] {
        &mut self.0
    }
}

impl Clone for Vec4 {
    fn clone(&self) -> Self {
        self.as_slice().into()
    }
}

impl math_traits::Quat<f64> for Vec4 {
    fn identity() -> Self {
        Self::quat_identity()
    }
}
