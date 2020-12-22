pub trait SliceExt<T: Copy> {
    //Gotta define these
    fn as_slice(&self) -> &[T];
    fn as_slice_mut(&mut self) -> &mut [T];

    //these can be left out of the impls, default will work
    fn copy_from_slice(&mut self, values:&[T]) {
        let curr:&mut [T] = self.as_slice_mut(); 
        curr.copy_from_slice(values);
    }

    fn copy_from(&mut self, other:&Self) {
        self.copy_from_slice(other.as_slice());
    }
}