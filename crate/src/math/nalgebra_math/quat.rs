use crate::math::traits::*;

pub type Quat = nalgebra::UnitQuaternion<f64>;

impl QuatExt for Quat {
    fn identity() -> Self {
        Self::identity()
    }
}

impl <'a> FromSliceExt<'a> for Quat {
    fn from_slice(values:&[f64]) -> Self {
        //The storage order is [ i, j, k, w ] while the arguments for this functions are in the order (w, i, j, k).
        Self::new_unchecked(nalgebra::Quaternion::new(values[3], values[0], values[1], values[2]))
    }
}

impl <'a> MathContainer<'a> for Quat {
    fn len(&self) -> usize { 4 }
    fn write_to_vf32(self: &Self, target:&mut [f32]) {
        //can't memcpy since it needs a cast
        target[0] = self.coords.x as f32;
        target[1] = self.coords.y as f32;
        target[2] = self.coords.z as f32;
        target[3] = self.coords.w as f32;
    }
}

impl AsSliceExt for Quat {
    fn as_slice(&self) -> &[f64] {
        self.coords.as_slice()
    }

    fn as_slice_mut(&mut self) -> &mut [f64] {
        self.as_mut_unchecked().coords.as_mut_slice()
    }
}
