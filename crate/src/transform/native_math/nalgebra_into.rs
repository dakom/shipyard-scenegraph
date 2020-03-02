use nalgebra;
use super::*;
use super::super::*;

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
        Self::new(quat.x, quat.y, quat.z, quat.w)
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
        Self::new_unchecked(nalgebra::Quaternion::new(quat.x, quat.y, quat.z, quat.w))
    }
}