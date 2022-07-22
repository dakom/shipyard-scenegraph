use crate::traits::{required as math_traits};

pub(super) type Vec3 = nalgebra::Vector3<f64>;

const VECTOR_ZERO: [f64; 3] = [0.0, 0.0, 0.0];
const VECTOR_ONE: [f64; 3] = [1.0, 1.0, 1.0];

impl math_traits::Vec3Ext<f64> for Vec3 {
    fn zero() -> Self {
        Self::from_row_slice(&VECTOR_ZERO)
    }
    fn one() -> Self {
        Self::from_row_slice(&VECTOR_ONE)
    }
}

