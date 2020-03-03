use nalgebra;
use crate::math::traits::{AsSliceExt, FromSliceExt};
use super::*;

impl From<nalgebra::Matrix4<f64>> for Matrix4 {
    fn from(mat:nalgebra::Matrix4<f64>) -> Self {
        Self::from(&mat)
    }
}
impl From<&nalgebra::Matrix4<f64>> for Matrix4 {
    fn from(mat:&nalgebra::Matrix4<f64>) -> Self {
        Self::from_slice(mat.as_slice())
    }
}

impl From<Matrix4> for nalgebra::Matrix4<f64> {
    fn from(mat:Matrix4) -> Self {
        Self::from(&mat)
    }
}
impl From<&Matrix4> for nalgebra::Matrix4<f64> {
    fn from(mat:&Matrix4) -> Self {
        Self::from_row_slice(mat.as_slice())
    }
}

impl From<nalgebra::Vector3<f64>> for Vec3 {
    fn from(vector:nalgebra::Vector3<f64>) -> Self {
        Self::from(&vector)
    }
}
impl From<&nalgebra::Vector3<f64>> for Vec3 {
    fn from(vector:&nalgebra::Vector3<f64>) -> Self {
        Self::from_slice(vector.as_slice())
    }
}

impl From<Vec3> for nalgebra::Vector3<f64> {
    fn from(vector:Vec3) -> Self {
        Self::from(&vector)
    }
}
impl From<&Vec3> for nalgebra::Vector3<f64> {
    fn from(vector:&Vec3) -> Self {
        Self::from_row_slice(vector.as_slice())
    }
}

impl From<nalgebra::Point3<f64>> for Vec3 {
    fn from(point:nalgebra::Point3<f64>) -> Self {
        Self::from(&point)
    }
}
impl From<&nalgebra::Point3<f64>> for Vec3 {
    fn from(point:&nalgebra::Point3<f64>) -> Self {
        Self::from_slice(point.coords.as_slice())
    }
}

impl From<Vec3> for nalgebra::Point3<f64> {
    fn from(vector:Vec3) -> Self {
        Self::from(&vector)
    }
}
impl From<&Vec3> for nalgebra::Point3<f64> {
    fn from(vector:&Vec3) -> Self {
        Self::from_slice(vector.as_slice())
    }
}

impl From<nalgebra::Quaternion<f64>> for Quat {
    fn from(quat:nalgebra::Quaternion<f64>) -> Self {
        Self::from(&quat)
    }
}
impl From<&nalgebra::Quaternion<f64>> for Quat {
    fn from(quat:&nalgebra::Quaternion<f64>) -> Self {
        Self::from_slice(quat.coords.as_slice())
    }
}

impl From<Quat> for nalgebra::Quaternion<f64> {
    fn from(quat:Quat) -> Self {
        Self::from(&quat)
    }
}
impl From<&Quat> for nalgebra::Quaternion<f64> {
    fn from(quat:&Quat) -> Self {
        //The storage order is [ i, j, k, w ] while the arguments for this functions are in the order (w, i, j, k).
        Self::new(quat.w, quat.x, quat.y, quat.z)
    }
}


impl From<nalgebra::UnitQuaternion<f64>> for Quat {
    fn from(quat:nalgebra::UnitQuaternion<f64>) -> Self {
        Self::from(&quat)
    }
}
impl From<&nalgebra::UnitQuaternion<f64>> for Quat {
    fn from(quat:&nalgebra::UnitQuaternion<f64>) -> Self {
        Self::from_slice(quat.coords.as_slice())
    }
}

impl From<Quat> for nalgebra::UnitQuaternion<f64> {
    fn from(quat:Quat) -> Self {
        Self::from(&quat)
    }
}
impl From<&Quat> for nalgebra::UnitQuaternion<f64> {
    fn from(quat:&Quat) -> Self {
        //The storage order is [ i, j, k, w ] while the arguments for this functions are in the order (w, i, j, k).
        Self::new_unchecked(nalgebra::Quaternion::new(quat.w, quat.x, quat.y, quat.z))
    }
}