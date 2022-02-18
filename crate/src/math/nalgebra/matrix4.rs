use crate::traits::math as math_traits;
use crate::traits::slice::*;

pub type Matrix4 = nalgebra::Matrix4<f64>;

impl math_traits::Matrix4<f64> for Matrix4 {
    fn identity() -> Self {
        Matrix4::identity()
    }

    fn reset_from_trs_origin(
        &mut self,
        translation: &[f64],
        rotation: &[f64],
        scale: &[f64],
        origin: &[f64],
    ) {
        let values = &mut self.as_slice_mut();
        let x = rotation[0];
        let y = rotation[1];
        let z = rotation[2];
        let w = rotation[3];
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
        let sx = scale[0];
        let sy = scale[1];
        let sz = scale[2];
        let ox = origin[0];
        let oy = origin[1];
        let oz = origin[2];
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
        values[12] = translation[0] + ox - (out0 * ox + out4 * oy + out8 * oz);
        values[13] = translation[1] + oy - (out1 * ox + out5 * oy + out9 * oz);
        values[14] = translation[2] + oz - (out2 * ox + out6 * oy + out10 * oz);
        values[15] = 1.0;
    }

    fn mul_assign(&mut self, other: &Self) {
        *self *= other;
    }
}

impl SliceExt<f64> for Matrix4 {
    fn as_slice(&self) -> &[f64] {
        self.as_slice()
    }

    fn as_slice_mut(&mut self) -> &mut [f64] {
        self.as_mut_slice()
    }
}

impl F32Compat for Matrix4 {
    fn write_to_vf32(self: &Self, target: &mut [f32]) {
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
