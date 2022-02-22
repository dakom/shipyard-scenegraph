use crate::traits::{required as math_traits, required::SliceExt, extra::F32Compat};
use std::convert::TryInto;
use std::ops::{Deref, DerefMut};

const VECTOR_ZERO: [f64; 3] = [0.0, 0.0, 0.0];
const VECTOR_ONE: [f64; 3] = [1.0, 1.0, 1.0];

#[repr(C)]
#[derive(PartialEq, Debug)]
pub struct Vec3([f64; 3]);

impl Vec3 {
    pub fn set_x(&mut self, value: f64) {
        self.0[0] = value;
    }
    pub fn set_y(&mut self, value: f64) {
        self.0[1] = value;
    }
    pub fn set_z(&mut self, value: f64) {
        self.0[2] = value;
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

    pub fn set_r(&mut self, value: f64) {
        self.0[0] = value;
    }
    pub fn set_g(&mut self, value: f64) {
        self.0[1] = value;
    }
    pub fn set_b(&mut self, value: f64) {
        self.0[2] = value;
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

    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self([x, y, z])
    }

    pub fn zero() -> Self {
        VECTOR_ZERO.as_ref().into()
    }

    pub fn one() -> Self {
        VECTOR_ONE.as_ref().into()
    }
}

impl F32Compat for Vec3 {
    fn write_to_vf32(&self, target: &mut [f32]) {
        //can't memcpy since it needs a cast
        target[0] = self.x() as f32;
        target[1] = self.y() as f32;
        target[2] = self.z() as f32;
    }
}

impl Deref for Vec3 {
    type Target = [f64];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Vec3 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<&[f64]> for Vec3 {
    fn from(values: &[f64]) -> Self {
        let data: [f64; 3] = values.try_into().unwrap();
        Self(data)
    }
}

impl SliceExt<f64> for Vec3 {
    fn as_slice(&self) -> &[f64] {
        &self.0
    }

    fn as_slice_mut(&mut self) -> &mut [f64] {
        &mut self.0
    }
}

impl Clone for Vec3 {
    fn clone(&self) -> Self {
        self.as_slice().into()
    }
}

impl math_traits::Vec3Ext<f64> for Vec3 {
    fn zero() -> Self {
        Self::zero()
    }
    fn one() -> Self {
        Self::one()
    }
}
