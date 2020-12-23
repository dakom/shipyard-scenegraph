use super::slice::SliceExt;
pub trait Vec3<T: Copy>: SliceExt<T> 
{
    fn zero() -> Self; 
    fn one() -> Self; 
}

pub trait Quat<T: Copy>: SliceExt<T> 
{
    fn identity() -> Self; 
}

pub trait Matrix4<T: Copy>: SliceExt<T>
{
    fn identity() -> Self;
    fn reset_from_trs_origin(&mut self, translation: &[T], rotation: &[T], scale: &[T], origin: &[T]);
    fn mul_assign(&mut self, other:&Self);
}