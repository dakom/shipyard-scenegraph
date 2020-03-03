use crate::math::traits::*;

pub type Vec3 = nalgebra::Vector3<f64>;

const VECTOR_DEFAULT:[f64;3] = [0.0, 0.0, 0.0];

impl VectorExt for Vec3 {
    fn zero() -> Self {
        Vec3::from_slice(&VECTOR_DEFAULT)
    }
}

impl <'a> FromSliceExt<'a> for Vec3 {
    fn from_slice(values:&[f64]) -> Self {
        Self::from_row_slice(values)
    }
}

impl <'a> MathContainer<'a> for Vec3 {
    fn len(&self) -> usize { 3 }
    fn write_to_vf32(self: &Self, target:&mut [f32]) {
        //can't memcpy since it needs a cast
        target[0] = self.x as f32;
        target[1] = self.y as f32;
        target[2] = self.z as f32;
    }
}

impl AsSliceExt for Vec3 {
    fn as_slice(&self) -> &[f64] {
        self.as_slice()
    }

    fn as_slice_mut(&mut self) -> &mut [f64] {
        self.as_mut_slice()
    }
}