use super::*;

pub trait TransformValuesExt: AsSliceExt + IdentityExt {
    //these need to be impl'd
    fn static_default() -> &'static [f64];
    fn len(self: &Self) -> usize;
    fn write_to_vf32(self: &Self, target:&mut [f32]); 
    fn from_slice(values:&[f64]) -> Self;
    //these can be left out of the impls, default will work
    fn copy_from_slice(&mut self, values:&[f64]) {
        let curr:&mut [f64] = self.as_slice_mut(); 
        curr.copy_from_slice(values);
    }

    fn reset(&mut self) {
        self.copy_from_slice(Self::static_default());
    }

    fn copy_from(&mut self, other:&Self) {
        self.copy_from_slice(other.as_slice());
    }

}

pub trait AsSliceExt {
    fn as_slice(&self) -> &[f64];
    fn as_slice_mut(&mut self) -> &mut [f64];
}

pub trait MatrixOpsExt: IdentityExt {
    fn mul_mut(&mut self, rhs: &Matrix4);
    fn new_from_translation(translation: &Vec3) -> Matrix4;
    fn reset_from_translation(&mut self, translation:&Vec3);
    fn set_translation(&mut self, translation:&Vec3);
    fn new_from_rotation(rotation: &Quat) -> Matrix4;
    fn reset_from_rotation(&mut self, rotation:&Quat);
    fn set_rotation(&mut self, rotation:&Quat);
    fn new_from_scale(scale:&Vec3) -> Matrix4;
    fn reset_from_scale(&mut self, scale:&Vec3);
    fn set_scale(&mut self, scale:&Vec3);
    fn new_from_trs(translation:&Vec3, rotation:&Quat, scale:&Vec3) -> Matrix4;
    fn reset_from_trs(&mut self, translation:&Vec3, rotation:&Quat, scale:&Vec3);
    fn set_trs(&mut self, translation:&Vec3, rotation:&Quat, scale:&Vec3);
    fn invert_mut(&mut self) -> Result<(), MatrixError>;
    fn invert(&self) -> Result<Matrix4, MatrixError>;
}

pub trait IdentityExt {
    fn identity() -> Self;
}