use crate::traits::{required as math_traits, required::SliceExt, extra::F32Compat};

pub type Quat = nalgebra::UnitQuaternion<f64>;

impl math_traits::Quat<f64> for Quat {
    fn identity() -> Self {
        //The storage order is [ i, j, k, w ] while the arguments for this functions are in the order (w, i, j, k).
        Self::new_unchecked(nalgebra::Quaternion::new(1.0, 0.0, 0.0, 0.0))
    }
}

impl SliceExt<f64> for Quat {
    fn as_slice(&self) -> &[f64] {
        self.coords.as_slice()
    }

    fn as_slice_mut(&mut self) -> &mut [f64] {
        self.as_mut_unchecked().coords.as_mut_slice()
    }
}

impl F32Compat for Quat {
    fn write_to_vf32(self: &Self, target: &mut [f32]) {
        //can't memcpy since it needs a cast
        target[0] = self.coords.x as f32;
        target[1] = self.coords.y as f32;
        target[2] = self.coords.z as f32;
        target[3] = self.coords.w as f32;
    }
}
