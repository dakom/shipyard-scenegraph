use super::super::{TransformValuesExt, AsSliceExt, IdentityExt};


macro_rules! impl_as_slice_repr {
    ( $( $x:ty ),* ) => {
        $(
            impl AsSliceExt for $x {
                //this is fast - no copy
                fn as_slice(&self) -> &[f64] {
                    let pointer = self as *const Self as *const f64;
                    let slice: &[f64] = unsafe { std::slice::from_raw_parts(pointer, self.len()) };
                    slice
                }

                fn as_slice_mut(&mut self) -> &mut [f64] {
                    let pointer = self as *const Self as *mut f64;
                    let slice: &mut [f64] = unsafe { std::slice::from_raw_parts_mut(pointer, self.len()) };
                    slice
                }
            }
        )*
    };
}

macro_rules! impl_as_slice_backing_data {
    ( $( $x:ty ),* ) => {
        $(
            impl AsSliceExt for $x {
                fn as_slice(&self) -> &[f64] {
                    &self.0
                }

                fn as_slice_mut(&mut self) -> &mut [f64] {
                    &mut self.0
                }
            }
        )*
    };
}
impl_as_slice_repr!{super::vec3::Vec3, super::quat::Quat}
impl_as_slice_backing_data!{super::matrix::Matrix4}