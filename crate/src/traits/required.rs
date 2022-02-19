pub trait SliceExt<T: Copy> {
    //Gotta define these
    fn as_slice(&self) -> &[T];
    fn as_slice_mut(&mut self) -> &mut [T];

    //these can be left out of the impls, default will work
    fn copy_from_slice(&mut self, values: &[T]) {
        let curr: &mut [T] = self.as_slice_mut();
        curr.copy_from_slice(values);
    }

    fn copy_from(&mut self, other: &Self) {
        self.copy_from_slice(other.as_slice());
    }
}

pub trait Vec3<T: Copy>: SliceExt<T> {
    fn zero() -> Self;
    fn one() -> Self;
}

pub trait Quat<T: Copy>: SliceExt<T> {
    fn identity() -> Self;
}

pub trait Matrix4<T: Copy>: SliceExt<T> {
    fn identity() -> Self;
    fn reset_from_trs_origin(
        &mut self,
        translation: &[T],
        rotation: &[T],
        scale: &[T],
        origin: &[T],
    );
    fn mul_assign(&mut self, other: &Self);
}
