use crate::traits::{required as math_traits, required::SliceExt, extra::F32Compat};

pub(super) type Quat = nalgebra_glm::Quat;

impl math_traits::QuatExt<f32> for Quat {
    fn identity() -> Self {
        Quat::identity()
    }
}

impl SliceExt<f32> for Quat {
    fn as_slice(&self) -> &[f32] {
        self.coords.as_slice()
    }

    fn as_slice_mut(&mut self) -> &mut [f32] {
        self.coords.as_mut_slice()
    }
}

impl F32Compat for Quat {
    fn write_to_vf32(&self, target: &mut [f32]) {
        target.copy_from_slice(self.as_slice());
    }
}
