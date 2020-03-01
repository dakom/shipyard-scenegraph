use super::components::{Vec3, Quat, Matrix4};

pub trait MatrixOps {
    fn mul_mut(&mut self, rhs: &Matrix4);
}

impl MatrixOps for Matrix4 {
    fn mul_mut(&mut self, rhs: &Matrix4) {
    }
}