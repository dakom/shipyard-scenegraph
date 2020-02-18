/*
    The math was taken and adapted from various places on the internet
    Specifically, from gl-matrix and the gltf-rs crate (which in turn took from cg_math)

    The idea is that we have a bespoke minimal math lib with no dependencies

    TODO: maybe impl From for nalgebra via a feature gate
    Though since as_ref() returns a slice maybe that's not necessary?
*/

use super::values::*;
use super::vec3::Vec3;
use super::quat::Quat;

#[derive(thiserror::Error, Debug)]
pub enum MatrixError {
    #[error("cannot invert the matrix")]
    Invert
}

#[repr(C)]
#[derive(Clone, PartialEq, Debug)]
pub struct Matrix4 (
    pub f64,
    pub f64,
    pub f64,
    pub f64,
    pub f64,
    pub f64,
    pub f64,
    pub f64,
    pub f64,
    pub f64,
    pub f64,
    pub f64,
    pub f64,
    pub f64,
    pub f64,
    pub f64,
);

const MATRIX_IDENTITY:[f64;16] = [
    1.0,0.0,0.0,0.0,
    0.0,1.0,0.0,0.0,
    0.0,0.0,1.0,0.0,
    0.0,0.0,0.0,1.0,
];

impl TransformValues for Matrix4 {
    fn len(&self) -> usize { 16 }
    fn static_default() -> &'static [f64] {
        &MATRIX_IDENTITY
    }
    fn write_to_vf32(self: &Self, target:&mut [f32]) {
        //can't memcpy since it needs a cast
        target[0] = self.0 as f32;
        target[1] = self.1 as f32;
        target[2] = self.2 as f32;
        target[3] = self.3 as f32;
        target[4] = self.4 as f32;
        target[5] = self.5 as f32;
        target[6] = self.6 as f32;
        target[7] = self.7 as f32;
        target[8] = self.8 as f32;
        target[9] = self.9 as f32;
        target[10] = self.10 as f32;
        target[11] = self.11 as f32;
        target[12] = self.12 as f32;
        target[13] = self.13 as f32;
        target[14] = self.14 as f32;
        target[15] = self.15 as f32;
    }
}

impl Default for Matrix4 {
    fn default() -> Self {
        Self(
            MATRIX_IDENTITY[0],
            MATRIX_IDENTITY[1],
            MATRIX_IDENTITY[2],
            MATRIX_IDENTITY[3],
            MATRIX_IDENTITY[4],
            MATRIX_IDENTITY[5],
            MATRIX_IDENTITY[6],
            MATRIX_IDENTITY[7],
            MATRIX_IDENTITY[8],
            MATRIX_IDENTITY[9],
            MATRIX_IDENTITY[10],
            MATRIX_IDENTITY[11],
            MATRIX_IDENTITY[12],
            MATRIX_IDENTITY[13],
            MATRIX_IDENTITY[14],
            MATRIX_IDENTITY[15],
        )
    }
}

impl Matrix4 {
    //translation
    pub fn new_from_translation(translation: &Vec3) -> Self {
        let mut m = Self::default();
        m.set_translation(translation);
        m
    }

    pub fn reset_from_translation(&mut self, translation:&Vec3) {
        self.reset();
        self.set_translation(translation);
    }

    pub fn set_translation(&mut self, translation:&Vec3) {
        self.12 = translation.x;
        self.13 = translation.y;
        self.14 = translation.z;
    }

    //rotation
    pub fn new_from_rotation(rotation: &Quat) -> Self {
        let mut m = Self::default();
        m.set_rotation(rotation);
        m
    }
    pub fn reset_from_rotation(&mut self, rotation:&Quat) {
        self.reset();
        self.set_rotation(rotation);
    }
    pub fn set_rotation(&mut self, rotation:&Quat) {
        let x = rotation.x;
        let y = rotation.y;
        let z = rotation.z;
        let w = rotation.w;
        let x2 = x + x;
        let y2 = y + y;
        let z2 = z + z;
        let xx = x * x2;
        let yx = y * x2;
        let yy = y * y2;
        let zx = z * x2;
        let zy = z * y2;
        let zz = z * z2;
        let wx = w * x2;
        let wy = w * y2;
        let wz = w * z2;
        self.0 = 1.0 - yy - zz;
        self.1 = yx + wz;
        self.2 = zx - wy;
        //self.3 = 0.0;
        self.4 = yx - wz;
        self.5 = 1.0 - xx - zz;
        self.6 = zy + wx;
        //self.7 = 0.0;
        self.8 = zx + wy;
        self.9 = zy - wx;
        self.10 = 1.0 - xx - yy;
        //self.11 = 0.0;
        //self.12 = 0.0;
        //self.13 = 0.0;
        //self.14 = 0.0;
        //self.15 = 1.0;
    }

    //scale
    pub fn new_from_scale(scale:&Vec3) -> Self {
        let mut m = Self::default();
        m.set_scale(scale);
        m
    }
    pub fn reset_from_scale(&mut self, scale:&Vec3) {
        self.reset();
        self.set_scale(scale);
    }

    pub fn set_scale(&mut self, scale:&Vec3) {
        self.0 = scale.x;
        self.5 = scale.y;
        self.10 = scale.z;
    }

    //translation, rotation, scale
    pub fn new_from_trs(translation:&Vec3, rotation:&Quat, scale:&Vec3) -> Self {
        let mut m = Self::default();
        m.set_trs(translation, rotation, scale);
        m
    }
    pub fn reset_from_trs(&mut self, translation:&Vec3, rotation:&Quat, scale:&Vec3) {
        self.reset();
        self.set_trs(translation, rotation, scale);
    }
    pub fn set_trs(&mut self, translation:&Vec3, rotation:&Quat, scale:&Vec3) {
        let x = rotation.x;
        let y = rotation.y; 
        let z = rotation.z;
        let w = rotation.w;
        let x2 = x + x;
        let y2 = y + y;
        let z2 = z + z;
        let xx = x * x2;
        let xy = x * y2;
        let xz = x * z2;
        let yy = y * y2;
        let yz = y * z2;
        let zz = z * z2;
        let wx = w * x2;
        let wy = w * y2;
        let wz = w * z2;
        let sx = scale.x;
        let sy = scale.y;
        let sz = scale.z;
        self.0 = (1.0 - (yy + zz)) * sx;
        self.1 = (xy + wz) * sx;
        self.2 = (xz - wy) * sx;
        //self.3 = 0.0;
        self.4 = (xy - wz) * sy;
        self.5 = (1.0 - (xx + zz)) * sy;
        self.6 = (yz + wx) * sy;
        //self.7 = 0.0;
        self.8 = (xz + wy) * sz;
        self.9 = (yz - wx) * sz;
        self.10 = (1.0 - (xx + yy)) * sz;
        //self.11 = 0.0;
        self.12 = translation.x;
        self.13 = translation.y;
        self.14 = translation.z;
        //self.15 = 1.0;

        /* alternatively, but slower:
        self.set_from_translation(translation);
        self.mul_mut(&Self::from_rotation(rotation));
        self.mul_mut(&Self::from_scale(scale));
        */
    }

    // arithmetic 
    pub fn mul_mut(&mut self, rhs: &Matrix4) {
        let a:&[f64] = self.as_ref();
        let b:&[f64] = rhs.as_ref();
        let a00 = a[0]; 
        let a01 = a[1]; 
        let a02 = a[2];
        let a03 = a[3];
        let a10 = a[4]; 
        let a11 = a[5];
        let a12 = a[6]; 
        let a13 = a[7];
        let a20 = a[8];
        let a21 = a[9];
        let a22 = a[10];
        let a23 = a[11];
        let a30 = a[12];
        let a31 = a[13];
        let a32 = a[14];
        let a33 = a[15];
        let mut b0  = b[0];
        let mut b1 = b[1];
        let mut b2 = b[2];
        let mut b3 = b[3];

        self.0 = b0*a00 + b1*a10 + b2*a20 + b3*a30;
        self.1 = b0*a01 + b1*a11 + b2*a21 + b3*a31;
        self.2 = b0*a02 + b1*a12 + b2*a22 + b3*a32;
        self.3 = b0*a03 + b1*a13 + b2*a23 + b3*a33;
        b0 = b[4]; b1 = b[5]; b2 = b[6]; b3 = b[7];
        self.4 = b0*a00 + b1*a10 + b2*a20 + b3*a30;
        self.5 = b0*a01 + b1*a11 + b2*a21 + b3*a31;
        self.6 = b0*a02 + b1*a12 + b2*a22 + b3*a32;
        self.7 = b0*a03 + b1*a13 + b2*a23 + b3*a33;
        b0 = b[8]; b1 = b[9]; b2 = b[10]; b3 = b[11];
        self.8 = b0*a00 + b1*a10 + b2*a20 + b3*a30;
        self.9 = b0*a01 + b1*a11 + b2*a21 + b3*a31;
        self.10 = b0*a02 + b1*a12 + b2*a22 + b3*a32;
        self.11 = b0*a03 + b1*a13 + b2*a23 + b3*a33;
        b0 = b[12]; b1 = b[13]; b2 = b[14]; b3 = b[15];
        self.12 = b0*a00 + b1*a10 + b2*a20 + b3*a30;
        self.13 = b0*a01 + b1*a11 + b2*a21 + b3*a31;
        self.14 = b0*a02 + b1*a12 + b2*a22 + b3*a32;
        self.15 = b0*a03 + b1*a13 + b2*a23 + b3*a33;
    }

    /// returns true if it was able to invert, false otherwise
    pub fn invert_mut(&mut self) -> Result<(), MatrixError> {
        let a:&[f64] = self.as_ref();
        let a00 = a[0]; 
        let a01 = a[1]; 
        let a02 = a[2]; 
        let a03 = a[3];
        let a10 = a[4]; 
        let a11 = a[5]; 
        let a12 = a[6]; 
        let a13 = a[7];
        let a20 = a[8]; 
        let a21 = a[9]; 
        let a22 = a[10]; 
        let a23 = a[11];
        let a30 = a[12]; 
        let a31 = a[13]; 
        let a32 = a[14]; 
        let a33 = a[15];
        let b00 = a00 * a11 - a01 * a10;
        let b01 = a00 * a12 - a02 * a10;
        let b02 = a00 * a13 - a03 * a10;
        let b03 = a01 * a12 - a02 * a11;
        let b04 = a01 * a13 - a03 * a11;
        let b05 = a02 * a13 - a03 * a12;
        let b06 = a20 * a31 - a21 * a30;
        let b07 = a20 * a32 - a22 * a30;
        let b08 = a20 * a33 - a23 * a30;
        let b09 = a21 * a32 - a22 * a31;
        let b10 = a21 * a33 - a23 * a31;
        let b11 = a22 * a33 - a23 * a32;
        // Calculate the determinant
        let mut det = b00 * b11 - b01 * b10 + b02 * b09 + b03 * b08 - b04 * b07 + b05 * b06;
        if det == 0.0 {
            Err(MatrixError::Invert)
        } else {
            det = 1.0 / det;
            self.0 = (a11 * b11 - a12 * b10 + a13 * b09) * det;
            self.1 = (a02 * b10 - a01 * b11 - a03 * b09) * det;
            self.2 = (a31 * b05 - a32 * b04 + a33 * b03) * det;
            self.3 = (a22 * b04 - a21 * b05 - a23 * b03) * det;
            self.4 = (a12 * b08 - a10 * b11 - a13 * b07) * det;
            self.5 = (a00 * b11 - a02 * b08 + a03 * b07) * det;
            self.6 = (a32 * b02 - a30 * b05 - a33 * b01) * det;
            self.7 = (a20 * b05 - a22 * b02 + a23 * b01) * det;
            self.8 = (a10 * b10 - a11 * b08 + a13 * b06) * det;
            self.9 = (a01 * b08 - a00 * b10 - a03 * b06) * det;
            self.10 = (a30 * b04 - a31 * b02 + a33 * b00) * det;
            self.11 = (a21 * b02 - a20 * b04 - a23 * b00) * det;
            self.12 = (a11 * b07 - a10 * b09 - a12 * b06) * det;
            self.13 = (a00 * b09 - a01 * b07 + a02 * b06) * det;
            self.14 = (a31 * b01 - a30 * b03 - a32 * b00) * det;
            self.15 = (a20 * b03 - a21 * b01 + a22 * b00) * det;
            Ok(())
        }
    }
    pub fn invert_clone(orig:&Self) -> Result<Self, MatrixError> {
        let mut clone = orig.clone();
        clone.invert_mut()?;
        Ok(clone)
    }
}

impl std::ops::Mul<Matrix4> for Matrix4 {
    type Output = Matrix4;
    fn mul(self, rhs: Matrix4) -> Self::Output {
        let mut clone = self.clone();
        clone.mul_mut(&rhs);
        clone
    }
}