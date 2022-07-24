use nalgebra::Vector3;
use crate::traits::{required::{SliceExt, Vec3Ext}, extra::F32Compat};


////////// F32 ///////////////////////////
impl F32Compat for Vector3<f32> {
    fn write_to_vf32(&self, target: &mut [f32]) {
        target.copy_from_slice(self.as_slice());
    }
}

impl SliceExt<f32> for Vector3<f32> {
    fn as_slice(&self) -> &[f32] {
        self.as_slice()
    }

    fn as_slice_mut(&mut self) -> &mut [f32] {
        self.as_mut_slice()
    }
}

const VECTOR_ZERO_F32: [f32; 3] = [0.0, 0.0, 0.0];
const VECTOR_ONE_F32: [f32; 3] = [1.0, 1.0, 1.0];

impl Vec3Ext<f32> for Vector3<f32> {
    fn zero() -> Self {
        Self::from_row_slice(&VECTOR_ZERO_F32)
    }
    fn one() -> Self {
        Self::from_row_slice(&VECTOR_ONE_F32)
    }
}


////////// F64 ///////////////////////////
impl F32Compat for Vector3<f64> {
    fn write_to_vf32(&self, target: &mut [f32]) {
        //can't memcpy since it needs a cast
        target[0] = self.x as f32;
        target[1] = self.y as f32;
        target[2] = self.z as f32;
    }
}

impl SliceExt<f64> for Vector3<f64> {
    fn as_slice(&self) -> &[f64] {
        self.as_slice()
    }

    fn as_slice_mut(&mut self) -> &mut [f64] {
        self.as_mut_slice()
    }
}

const VECTOR_ZERO_F64: [f64; 3] = [0.0, 0.0, 0.0];
const VECTOR_ONE_F64: [f64; 3] = [1.0, 1.0, 1.0];

impl Vec3Ext<f64> for Vector3<f64> {
    fn zero() -> Self {
        Self::from_row_slice(&VECTOR_ZERO_F64)
    }
    fn one() -> Self {
        Self::from_row_slice(&VECTOR_ONE_F64)
    }
}



