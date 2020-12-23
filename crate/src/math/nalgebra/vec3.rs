use std::ops::{Deref, DerefMut};
use std::convert::TryInto;

use crate::traits::slice::*;
use crate::traits::math as math_traits;

pub type Vec3 = nalgebra::Vector3<f64>;

const VECTOR_ZERO:[f64;3] = [0.0, 0.0, 0.0];
const VECTOR_ONE:[f64;3] = [1.0, 1.0, 1.0];

impl math_traits::Vec3<f64> for Vec3 {
    fn zero() -> Self {
        Self::from_row_slice(&VECTOR_ZERO)
    }
    fn one() -> Self {
        Self::from_row_slice(&VECTOR_ONE)
    }
}

impl SliceExt<f64> for Vec3 {
    fn as_slice(&self) -> &[f64] {
        self.as_slice()
    }

    fn as_slice_mut(&mut self) -> &mut [f64] {
        self.as_mut_slice()
    }

}

impl F32Compat for Vec3 {
    fn write_to_vf32(self: &Self, target:&mut [f32]) {
        //can't memcpy since it needs a cast
        target[0] = self.x as f32;
        target[1] = self.y as f32;
        target[2] = self.z as f32;
    }
}