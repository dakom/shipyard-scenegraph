use nalgebra::{UnitQuaternion, Quaternion};
use crate::traits::{required::{SliceExt, QuatExt}, extra::F32Compat};

////////// F32 ///////////////////////////
impl F32Compat for Quaternion<f32> {
    fn write_to_vf32(&self, target: &mut [f32]) {
        target.copy_from_slice(self.as_slice());
    }
}

impl SliceExt<f32> for Quaternion<f32>{
    fn as_slice(&self) -> &[f32] {
        self.coords.as_slice()
    }

    fn as_slice_mut(&mut self) -> &mut [f32] {
        self.coords.as_mut_slice()
    }
}

impl QuatExt<f32> for Quaternion<f32> {
    fn identity() -> Self {
        Quaternion::identity()
    }
}

impl F32Compat for UnitQuaternion<f32> {
    fn write_to_vf32(&self, target: &mut [f32]) {
        target.copy_from_slice(self.as_slice());
    }
}

impl SliceExt<f32> for UnitQuaternion<f32>{
    fn as_slice(&self) -> &[f32] {
        self.coords.as_slice()
    }

    fn as_slice_mut(&mut self) -> &mut [f32] {
        self.as_mut_unchecked().coords.as_mut_slice()
        //self.coords.as_mut_slice()
    }
}

impl QuatExt<f32> for UnitQuaternion<f32> {
    fn identity() -> Self {
        UnitQuaternion::identity()
    }
}

////////// F64 ///////////////////////////
impl F32Compat for Quaternion<f64>{
    fn write_to_vf32(&self, target: &mut [f32]) {
        //can't memcpy since it needs a cast
        target[0] = self.coords.x as f32;
        target[1] = self.coords.y as f32;
        target[2] = self.coords.z as f32;
        target[3] = self.coords.w as f32;
    }
}

impl SliceExt<f64> for Quaternion<f64>{
    fn as_slice(&self) -> &[f64] {
        self.coords.as_slice()
    }

    fn as_slice_mut(&mut self) -> &mut [f64] {
        self.coords.as_mut_slice()
    }
}
impl QuatExt<f64> for Quaternion<f64>{
    fn identity() -> Self {
        //The storage order is [ i, j, k, w ] while the arguments for this functions are in the order (w, i, j, k).
        nalgebra::Quaternion::new(1.0, 0.0, 0.0, 0.0)
    }
}

impl F32Compat for UnitQuaternion<f64>{
    fn write_to_vf32(&self, target: &mut [f32]) {
        //can't memcpy since it needs a cast
        target[0] = self.coords.x as f32;
        target[1] = self.coords.y as f32;
        target[2] = self.coords.z as f32;
        target[3] = self.coords.w as f32;
    }
}


impl SliceExt<f64> for UnitQuaternion<f64>{
    fn as_slice(&self) -> &[f64] {
        self.coords.as_slice()
    }

    fn as_slice_mut(&mut self) -> &mut [f64] {
        self.as_mut_unchecked().coords.as_mut_slice()
    }
}

impl QuatExt<f64> for UnitQuaternion<f64>{
    fn identity() -> Self {
        //The storage order is [ i, j, k, w ] while the arguments for this functions are in the order (w, i, j, k).
        Self::new_unchecked(Quaternion::new(1.0, 0.0, 0.0, 0.0))
    }
}
