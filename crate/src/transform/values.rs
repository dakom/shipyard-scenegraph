pub trait TransformValues: AsRef<[f64]> + AsMut<[f64]> + Default {
    //these need to be impl'd
    fn static_default() -> &'static [f64];
    fn len(self: &Self) -> usize;
    fn write_to_vf32(self: &Self, target:&mut [f32]); 

    //these can be left out of the impls
    fn copy_from_slice(&mut self, values:&[f64]) {
        let curr:&mut [f64] = self.as_mut(); 
        curr.copy_from_slice(values);
    }

    fn reset(&mut self) {
        self.copy_from_slice(Self::static_default());
    }
    fn new_from_slice(values:&[f64]) -> Self {
        let mut _self = Self::default();
        _self.copy_from_slice(values);
        _self
    }

    fn copy_from(&mut self, other:&Self) {
        self.copy_from_slice(other.as_ref());
    }
}

macro_rules! impl_asref {
    ( $( $x:ty ),* ) => {
        $(

            impl AsRef<[f64]> for $x {
                //this is fast - no copy
                fn as_ref(&self) -> &[f64] {
                    let pointer = self as *const Self as *const f64;
                    let slice: &[f64] = unsafe { std::slice::from_raw_parts(pointer, self.len()) };
                    slice
                }
            }
            impl AsMut<[f64]> for $x {
                //this is fast - no copy
                fn as_mut(&mut self) -> &mut [f64] {
                    let pointer = self as *const Self as *mut f64;
                    let slice: &mut [f64] = unsafe { std::slice::from_raw_parts_mut(pointer, self.len()) };
                    slice
                }
            }
        )*
    };
}

impl_asref!{super::vec3::Vec3, super::quat::Quat, super::matrix::Matrix4}