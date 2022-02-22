use crate::traits::{required as math_traits, required::SliceExt, extra::F32Compat};

pub(super) type Vec3 = nalgebra_glm::Vec3;

const VECTOR_ZERO: [f32; 3] = [0.0, 0.0, 0.0];
const VECTOR_ONE: [f32; 3] = [1.0, 1.0, 1.0];

impl math_traits::Vec3Ext<f32> for Vec3 {
    fn zero() -> Self {
        Self::from_row_slice(&VECTOR_ZERO)
    }
    fn one() -> Self {
        Self::from_row_slice(&VECTOR_ONE)
    }
}

impl SliceExt<f32> for Vec3 {
    fn as_slice(&self) -> &[f32] {
        self.as_slice()
    }

    fn as_slice_mut(&mut self) -> &mut [f32] {
        self.as_mut_slice()
    }
}

impl F32Compat for Vec3 {
    fn write_to_vf32(&self, target: &mut [f32]) {
        target.copy_from_slice(self.as_slice());
    }
}
