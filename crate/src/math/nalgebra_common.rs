use nalgebra::{Matrix4, UnitQuaternion, Quaternion, Vector3};
use crate::traits::{required::SliceExt, extra::F32Compat};

impl F32Compat for Matrix4<f32> {
    fn write_to_vf32(&self, target: &mut [f32]) {
        target.copy_from_slice(self.as_slice());
    }
}

impl F32Compat for Matrix4<f64> {
    fn write_to_vf32(&self, target: &mut [f32]) {
        let values = self.as_slice();

        //can't memcpy since it needs a cast
        target[0] = values[0] as f32;
        target[1] = values[1] as f32;
        target[2] = values[2] as f32;
        target[3] = values[3] as f32;
        target[4] = values[4] as f32;
        target[5] = values[5] as f32;
        target[6] = values[6] as f32;
        target[7] = values[7] as f32;
        target[8] = values[8] as f32;
        target[9] = values[9] as f32;
        target[10] = values[10] as f32;
        target[11] = values[11] as f32;
        target[12] = values[12] as f32;
        target[13] = values[13] as f32;
        target[14] = values[14] as f32;
        target[15] = values[15] as f32;
    }
}

impl SliceExt<f32> for Matrix4<f32> {
    fn as_slice(&self) -> &[f32] {
        self.as_slice()
    }

    fn as_slice_mut(&mut self) -> &mut [f32] {
        self.as_mut_slice()
    }
}

impl SliceExt<f64> for Matrix4<f64> {
    fn as_slice(&self) -> &[f64] {
        self.as_slice()
    }

    fn as_slice_mut(&mut self) -> &mut [f64] {
        self.as_mut_slice()
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
impl SliceExt<f64> for Quaternion<f64>{
    fn as_slice(&self) -> &[f64] {
        self.coords.as_slice()
    }

    fn as_slice_mut(&mut self) -> &mut [f64] {
        self.coords.as_mut_slice()
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

impl F32Compat for Quaternion<f64>{
    fn write_to_vf32(&self, target: &mut [f32]) {
        //can't memcpy since it needs a cast
        target[0] = self.coords.x as f32;
        target[1] = self.coords.y as f32;
        target[2] = self.coords.z as f32;
        target[3] = self.coords.w as f32;
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
impl SliceExt<f32> for Quaternion<f32>{
    fn as_slice(&self) -> &[f32] {
        self.coords.as_slice()
    }

    fn as_slice_mut(&mut self) -> &mut [f32] {
        self.coords.as_mut_slice()
    }
}

impl F32Compat for UnitQuaternion<f32> {
    fn write_to_vf32(&self, target: &mut [f32]) {
        target.copy_from_slice(self.as_slice());
    }
}
impl F32Compat for Quaternion<f32> {
    fn write_to_vf32(&self, target: &mut [f32]) {
        target.copy_from_slice(self.as_slice());
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

impl F32Compat for Vector3<f64> {
    fn write_to_vf32(&self, target: &mut [f32]) {
        //can't memcpy since it needs a cast
        target[0] = self.x as f32;
        target[1] = self.y as f32;
        target[2] = self.z as f32;
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

impl F32Compat for Vector3<f32> {
    fn write_to_vf32(&self, target: &mut [f32]) {
        target.copy_from_slice(self.as_slice());
    }
}
