use super::components::{Vec3, Quat, Matrix4};
use super::super::{TransformValuesExt, AsSliceExt, IdentityExt, MatrixOpsExt, MatrixError};


const QUAT_IDENTITY:[f64;4] = [0.0, 0.0, 0.0, 1.0];
impl IdentityExt for Quat {
    fn identity() -> Self {
        Quat::identity()
    }
}
impl TransformValuesExt for Quat {
    fn len(&self) -> usize { 4 }
    fn static_default() -> &'static [f64] {
        &QUAT_IDENTITY
    }
    fn write_to_vf32(self: &Self, target:&mut [f32]) {
        //can't memcpy since it needs a cast
        target[0] = self.coords.x as f32;
        target[1] = self.coords.y as f32;
        target[2] = self.coords.z as f32;
        target[3] = self.coords.w as f32;
    }
    fn from_slice(values:&[f64]) -> Self {
        let mut _self = Self::identity();
        _self.copy_from_slice(values);
        _self
    }
}

impl AsSliceExt for Quat {
    fn as_slice(&self) -> &[f64] {
        self.as_slice()
    }

    fn as_slice_mut(&mut self) -> &mut [f64] {
        self.coords.as_mut_slice()
    }
}


const VECTOR_IDENTITY:[f64;3] = [0.0, 0.0, 0.0];

impl IdentityExt for Vec3 {
    fn identity() -> Self {
        Vec3::identity()
    }
}
impl TransformValuesExt for Vec3 {
    fn len(&self) -> usize { 3 }
    fn static_default() -> &'static [f64] {
        &VECTOR_IDENTITY
    }
    fn write_to_vf32(self: &Self, target:&mut [f32]) {
        //can't memcpy since it needs a cast
        target[0] = self.x as f32;
        target[1] = self.y as f32;
        target[2] = self.z as f32;
    }
    fn from_slice(values:&[f64]) -> Self {
        let mut _self = Self::identity();
        _self.copy_from_slice(values);
        _self
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

const MATRIX_IDENTITY:[f64;16] = [
    1.0,0.0,0.0,0.0,
    0.0,1.0,0.0,0.0,
    0.0,0.0,1.0,0.0,
    0.0,0.0,0.0,1.0,
];



impl IdentityExt for Matrix4 {
    fn identity() -> Self {
        Matrix4::identity()
    }
}

impl TransformValuesExt for Matrix4 {
    fn len(&self) -> usize { 16 }
    fn static_default() -> &'static [f64] {
        &MATRIX_IDENTITY
    }
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
    fn from_slice(values:&[f64]) -> Self {
        let mut _self = Self::identity();
        _self.copy_from_slice(values);
        _self
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

impl MatrixOpsExt for Matrix4 {
    //translation
    fn new_from_translation(translation: &Vec3) -> Self {
        Matrix4::new_translation(translation)
    }

    fn reset_from_translation(&mut self, translation:&Vec3) {
        self.fill_with_identity();
        self.set_translation(translation);
    }

    fn set_translation(&mut self, translation:&Vec3) {
        self.append_translation_mut(translation);
    }

    //rotation
    fn new_from_rotation(rotation: &Quat) -> Self {
        rotation.to_rotation_matrix().into()
    }
    fn reset_from_rotation(&mut self, rotation:&Quat) {
        self.fill_with_identity();
        self.set_rotation(rotation);
    }
    fn set_rotation(&mut self, rotation:&Quat) {
        //TODO - probably a faster way to do this
        let mat:Matrix4 = rotation.to_rotation_matrix().into();
        *self *= mat;
    }

    //scale
    fn new_from_scale(scale:&Vec3) -> Self {
        Matrix4::new_nonuniform_scaling(scale)
    }
    fn reset_from_scale(&mut self, scale:&Vec3) {
        self.fill_with_identity();
        self.set_scale(scale);
    }

    fn set_scale(&mut self, scale:&Vec3) {
        self.append_nonuniform_scaling_mut(scale);
    }

    //translation, rotation, scale
    fn new_from_trs(translation:&Vec3, rotation:&Quat, scale:&Vec3) -> Self {
    }
    fn reset_from_trs(&mut self, translation:&Vec3, rotation:&Quat, scale:&Vec3) {
    }
    fn set_trs(&mut self, translation:&Vec3, rotation:&Quat, scale:&Vec3) {
    }

    // arithmetic 
    fn mul_mut(&mut self, rhs: &Matrix4) {
    }

    /// returns true if it was able to invert, false otherwise
    fn invert_mut(&mut self) -> Result<(), MatrixError> {
    }
    fn invert(&self) -> Result<Self, MatrixError> {
    }
}