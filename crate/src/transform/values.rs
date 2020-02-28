pub trait TransformValues: AsSlice + Default {
    //these need to be impl'd
    fn static_default() -> &'static [f64];
    fn len(self: &Self) -> usize;
    fn write_to_vf32(self: &Self, target:&mut [f32]); 

    //these can be left out of the impls, default will work
    fn from_slice(values:&[f64]) -> Self {
        let mut _self = Self::default();
        _self.copy_from_slice(values);
        _self
    }
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

pub trait AsSlice {
    fn as_slice(&self) -> &[f64];
    fn as_slice_mut(&mut self) -> &mut [f64];
}

macro_rules! impl_as_slice_repr {
    ( $( $x:ty ),* ) => {
        $(
            impl AsSlice for $x {
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
            impl AsSlice for $x {
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