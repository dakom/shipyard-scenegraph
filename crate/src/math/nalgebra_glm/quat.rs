use crate::traits::required as math_traits;

pub(super) type Quat = nalgebra_glm::Quat;

impl math_traits::QuatExt<f32> for Quat {
    fn identity() -> Self {
        Quat::identity()
    }
}

