/*
    The math was taken and adapted from various places on the internet
    Specifically, from gl-matrix and the gltf-rs crate (which in turn took from cg_math)

    The idea is that we have a bespoke minimal math lib with no dependencies

    TODO: maybe impl From for nalgebra via a feature gate
    Though since as_ref() returns a slice maybe that's not necessary?
*/

use std::ops::{Mul, MulAssign};
use std::convert::{AsRef, TryInto};
use super::vec3::Vec3;
use super::quat::Quat;
use crate::math::traits::{MathContainer, AsSliceExt, FromSliceExt, MatrixExt};
use crate::errors::{MatrixError};

#[repr(C)]
#[derive(Clone, PartialEq, Debug)]
//pub struct Matrix4 (pub [f64;16]);
pub struct Matrix4 (pub [f64;16]);

const MATRIX_IDENTITY:[f64;16] = [
    1.0,0.0,0.0,0.0,
    0.0,1.0,0.0,0.0,
    0.0,0.0,1.0,0.0,
    0.0,0.0,0.0,1.0,
];

impl Matrix4 {
    pub fn new( a:f64, b:f64, c:f64, d:f64,
                e:f64, f:f64, g:f64, h:f64,  
                i:f64, j:f64, k:f64, l:f64,
                m:f64, n:f64, o:f64, p:f64,
        ) -> Self {
            Self([
                a,b,c,d,
                e,f,g,h,
                i,j,k,l,
                m,n,o,p,
            ])
    }
    fn reset(&mut self) {
        self.copy_from_slice(&MATRIX_IDENTITY);
    }
}

impl From<&[f64]> for Matrix4 {
    fn from(values:&[f64]) -> Self {
        let data:[f64;16] = values.try_into().unwrap();
        Self(data)
    }
}

impl <'a> FromSliceExt<'a> for Matrix4 {
    fn from_slice(values:&'a [f64]) -> Self {
        values.into()
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

impl MatrixExt for Matrix4 {
    fn identity() -> Self {
        Self(MATRIX_IDENTITY.clone())
    }
    //translation
    fn new_from_translation(translation: &Vec3) -> Self {
        let mut m = Self::identity();
        m.translate(translation);
        m
    }

    fn reset_from_translation(&mut self, translation:&Vec3) {
        self.reset();
        self.translate(translation);
    }

    fn translate(&mut self, translation:&Vec3) {
        let values = &mut self.0;
        values[12] = translation.x;
        values[13] = translation.y;
        values[14] = translation.z;
    }

    //rotation
    fn new_from_rotation(rotation: &Quat) -> Self {
        let mut m = Self::identity();
        m.rotate(rotation);
        m
    }
    fn reset_from_rotation(&mut self, rotation:&Quat) {
        self.reset();
        self.rotate(rotation);
    }
    fn rotate(&mut self, rotation:&Quat) {
        let values = &mut self.0;
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
        values[0] = 1.0 - yy - zz;
        values[1] = yx + wz;
        values[2] = zx - wy;
        //values[3 = 0.0;
        values[4] = yx - wz;
        values[5] = 1.0 - xx - zz;
        values[6] = zy + wx;
        //values[7 = 0.0;
        values[8] = zx + wy;
        values[9] = zy - wx;
        values[10] = 1.0 - xx - yy;
        //values[11 = 0.0;
        //values[12 = 0.0;
        //values[13 = 0.0;
        //values[14 = 0.0;
        //values[15 = 1.0;
    }

    //scale
    fn new_from_scale(scale:&Vec3) -> Self {
        let mut m = Self::identity();
        m.scale(scale);
        m
    }
    fn reset_from_scale(&mut self, scale:&Vec3) {
        self.reset();
        self.scale(scale);
    }

    fn scale(&mut self, scale:&Vec3) {
        let values = &mut self.0;
        values[0] = scale.x;
        values[5] = scale.y;
        values[10] = scale.z;
    }

    //translation, rotation, scale
    fn new_from_trs(translation:&Vec3, rotation:&Quat, scale:&Vec3) -> Self {
        let mut m = Self::identity();
        m.set_trs(translation, rotation, scale);
        m
    }
    fn new_from_trs_origin(translation:&Vec3, rotation:&Quat, scale:&Vec3, origin:&Vec3) -> Self {
        let mut m = Self::identity();
        m.set_trs_origin(translation, rotation, scale, origin);
        m
    }
    fn reset_from_trs(&mut self, translation:&Vec3, rotation:&Quat, scale:&Vec3) {
        self.reset();
        self.set_trs(translation, rotation, scale);
    }
    fn reset_from_trs_origin(&mut self, translation:&Vec3, rotation:&Quat, scale:&Vec3, origin:&Vec3) {
        self.reset();
        self.set_trs_origin(translation, rotation, scale, origin);
    }
    fn set_trs(&mut self, translation:&Vec3, rotation:&Quat, scale:&Vec3) {
        let values = &mut self.0;
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
        values[0] = (1.0 - (yy + zz)) * sx;
        values[1] = (xy + wz) * sx;
        values[2] = (xz - wy) * sx;
        //values[3 = 0.0;
        values[4] = (xy - wz) * sy;
        values[5] = (1.0 - (xx + zz)) * sy;
        values[6] = (yz + wx) * sy;
        //values[7 = 0.0;
        values[8] = (xz + wy) * sz;
        values[9] = (yz - wx) * sz;
        values[10] = (1.0 - (xx + yy)) * sz;
        //values[11 = 0.0;
        values[12] = translation.x;
        values[13] = translation.y;
        values[14] = translation.z;
        //values[15 = 1.0;
    }
    fn set_trs_origin(&mut self, translation:&Vec3, rotation:&Quat, scale:&Vec3, origin:&Vec3) {
        let values = &mut self.0;
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
        let ox = origin.x;
        let oy = origin.y;
        let oz = origin.z;
        let out0 = (1.0 - (yy + zz)) * sx;
        let out1 = (xy + wz) * sx;
        let out2 = (xz - wy) * sx;
        let out4 = (xy - wz) * sy;
        let out5 = (1.0 - (xx + zz)) * sy;
        let out6 = (yz + wx) * sy;
        let out8 = (xz + wy) * sz;
        let out9 = (yz - wx) * sz;
        let out10 = (1.0 - (xx + yy)) * sz;
        values[0] = out0;
        values[1] = out1;
        values[2] = out2;
        values[3] = 0.0;
        values[4] = out4;
        values[5] = out5;
        values[6] = out6;
        values[7] = 0.0;
        values[8] = out8;
        values[9] = out9;
        values[10] = out10;
        values[11] = 0.0;
        values[12] = translation.x + ox - (out0 * ox + out4 * oy + out8 * oz);
        values[13] = translation.y + oy - (out1 * ox + out5 * oy + out9 * oz);
        values[14] = translation.z + oz - (out2 * ox + out6 * oy + out10 * oz);
        values[15] = 1.0;
    }

    // arithmetic 

    /// returns true if it was able to invert, false otherwise
    fn invert_mut(&mut self) -> Result<(), MatrixError> {
        let values = &mut self.0;
        let a:&[f64] = values; 
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
            values[0] = (a11 * b11 - a12 * b10 + a13 * b09) * det;
            values[1] = (a02 * b10 - a01 * b11 - a03 * b09) * det;
            values[2] = (a31 * b05 - a32 * b04 + a33 * b03) * det;
            values[3] = (a22 * b04 - a21 * b05 - a23 * b03) * det;
            values[4] = (a12 * b08 - a10 * b11 - a13 * b07) * det;
            values[5] = (a00 * b11 - a02 * b08 + a03 * b07) * det;
            values[6] = (a32 * b02 - a30 * b05 - a33 * b01) * det;
            values[7] = (a20 * b05 - a22 * b02 + a23 * b01) * det;
            values[8] = (a10 * b10 - a11 * b08 + a13 * b06) * det;
            values[9] = (a01 * b08 - a00 * b10 - a03 * b06) * det;
            values[10] = (a30 * b04 - a31 * b02 + a33 * b00) * det;
            values[11] = (a21 * b02 - a20 * b04 - a23 * b00) * det;
            values[12] = (a11 * b07 - a10 * b09 - a12 * b06) * det;
            values[13] = (a00 * b09 - a01 * b07 + a02 * b06) * det;
            values[14] = (a31 * b01 - a30 * b03 - a32 * b00) * det;
            values[15] = (a20 * b03 - a21 * b01 + a22 * b00) * det;
            Ok(())
        }
    }
    fn invert(&self) -> Result<Self, MatrixError> {
        let mut clone = self.clone();
        clone.invert_mut()?;
        Ok(clone)
    }
}


impl AsRef<Matrix4> for Matrix4 {
    fn as_ref(&self) -> &Self {
        self
    }
}
impl <T: AsRef<Matrix4>> Mul<T> for Matrix4 {
    type Output = Matrix4;
    fn mul(self, rhs: T) -> Self::Output {
        let mut clone = self.clone();
        clone *= rhs.as_ref();
        clone
    }
}

impl <T: AsRef<Matrix4>> Mul<T> for &Matrix4 {
    type Output = Matrix4;
    fn mul(self, rhs: T) -> Self::Output {
        let mut clone = self.clone();
        clone *= rhs.as_ref();
        clone
    }
}

impl <T: AsRef<Matrix4>> MulAssign<T> for Matrix4 {
    fn mul_assign(&mut self, rhs: T) {
        let values = &mut self.0;
        let a:&[f64] = values; 
        let b:&[f64] = rhs.as_ref().as_slice();
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

        values[0] = b0*a00 + b1*a10 + b2*a20 + b3*a30;
        values[1] = b0*a01 + b1*a11 + b2*a21 + b3*a31;
        values[2] = b0*a02 + b1*a12 + b2*a22 + b3*a32;
        values[3] = b0*a03 + b1*a13 + b2*a23 + b3*a33;
        b0 = b[4]; b1 = b[5]; b2 = b[6]; b3 = b[7];
        values[4] = b0*a00 + b1*a10 + b2*a20 + b3*a30;
        values[5] = b0*a01 + b1*a11 + b2*a21 + b3*a31;
        values[6] = b0*a02 + b1*a12 + b2*a22 + b3*a32;
        values[7] = b0*a03 + b1*a13 + b2*a23 + b3*a33;
        b0 = b[8]; b1 = b[9]; b2 = b[10]; b3 = b[11];
        values[8] = b0*a00 + b1*a10 + b2*a20 + b3*a30;
        values[9] = b0*a01 + b1*a11 + b2*a21 + b3*a31;
        values[10] = b0*a02 + b1*a12 + b2*a22 + b3*a32;
        values[11] = b0*a03 + b1*a13 + b2*a23 + b3*a33;
        b0 = b[12]; b1 = b[13]; b2 = b[14]; b3 = b[15];
        values[12] = b0*a00 + b1*a10 + b2*a20 + b3*a30;
        values[13] = b0*a01 + b1*a11 + b2*a21 + b3*a31;
        values[14] = b0*a02 + b1*a12 + b2*a22 + b3*a32;
        values[15] = b0*a03 + b1*a13 + b2*a23 + b3*a33;
    }
}