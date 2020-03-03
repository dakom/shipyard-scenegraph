use crate::math::traits::*;
use crate::errors::MatrixError;
use super::vec3::*;
use super::quat::*;

pub type Matrix4 = nalgebra::Matrix4<f64>;


impl <'a> FromSliceExt<'a> for Matrix4 {
    fn from_slice(values:&[f64]) -> Self {
        Self::from_row_slice(values)
    }
}

impl <'a> MathContainer<'a> for Matrix4 {
    fn len(&self) -> usize { 16 }
    fn write_to_vf32(self: &Self, target:&mut [f32]) {
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

impl AsSliceExt for Matrix4 {

    fn as_slice(&self) -> &[f64] {
        self.as_slice()
    }

    fn as_slice_mut(&mut self) -> &mut [f64] {
        self.as_mut_slice()
    }
}

impl MatrixExt for Matrix4 {
    fn identity() -> Self {
        Matrix4::identity()
    }

    //translation
    fn new_from_translation(translation: &Vec3) -> Self {
        Matrix4::new_translation(translation)
    }

    fn reset_from_translation(&mut self, translation:&Vec3) {
        self.fill_with_identity();
        self.translate(translation);
    }

    fn translate(&mut self, translation:&Vec3) {
        self.append_translation_mut(translation);
    }

    //rotation
    fn new_from_rotation(rotation: &Quat) -> Self {
        rotation.to_rotation_matrix().into()
    }
    fn reset_from_rotation(&mut self, rotation:&Quat) {
        self.fill_with_identity();
        self.rotate(rotation);
    }
    fn rotate(&mut self, rotation:&Quat) {
        let mat:Matrix4 = rotation.to_rotation_matrix().into();
        *self *= mat;
    }

    //scale
    fn new_from_scale(scale:&Vec3) -> Self {
        Matrix4::new_nonuniform_scaling(scale)
    }
    fn reset_from_scale(&mut self, scale:&Vec3) {
        self.fill_with_identity();
        self.scale(scale);
    }

    fn scale(&mut self, scale:&Vec3) {
        self.append_nonuniform_scaling_mut(scale);
    }

    //translation, rotation, scale
    fn new_from_trs(translation:&Vec3, rotation:&Quat, scale:&Vec3) -> Self {
        let mut mat = Matrix4::identity();
        mat.set_trs(translation, rotation, scale);
        mat
    }
    fn reset_from_trs(&mut self, translation:&Vec3, rotation:&Quat, scale:&Vec3) {
        self.fill_with_identity();
        self.set_trs(translation, rotation, scale);
    }
    fn set_trs(&mut self, translation:&Vec3, rotation:&Quat, scale:&Vec3) {
        self.translate(translation);
        self.scale(scale);
        self.rotate(rotation);
    }

    /// returns true if it was able to invert, false otherwise
    fn invert_mut(&mut self) -> Result<(), MatrixError> {
        if self.try_inverse_mut() {
            Ok(())
        } else {
            Err(MatrixError::Invert)
        }
    }
    fn invert(&self) -> Result<Self, MatrixError> {
        self.try_inverse().ok_or(MatrixError::Invert)
    }
}