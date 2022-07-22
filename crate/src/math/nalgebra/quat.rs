use crate::traits::required as math_traits;

pub(super) type Quat = nalgebra::UnitQuaternion<f64>;

impl math_traits::QuatExt<f64> for Quat {
    fn identity() -> Self {
        //The storage order is [ i, j, k, w ] while the arguments for this functions are in the order (w, i, j, k).
        Self::new_unchecked(nalgebra::Quaternion::new(1.0, 0.0, 0.0, 0.0))
    }
}
