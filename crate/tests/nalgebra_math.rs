use shipyard_scenegraph::prelude::*;
use shipyard_scenegraph::traits::math as math_traits;

const MATRIX_IDENTITY: [f64; 16] = [
    1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
];

const VECTOR_ZERO: [f64; 3] = [0.0, 0.0, 0.0];
const VECTOR_ONE: [f64; 3] = [1.0, 1.0, 1.0];

const QUAT_IDENTITY: [f64; 4] = [0.0, 0.0, 0.0, 1.0];

#[test]
fn test_nalgebra() {
    let m: Matrix4 = math_traits::Matrix4::identity();
    assert_eq!(m.as_slice(), MATRIX_IDENTITY);

    let v: Vec3 = math_traits::Vec3::zero();
    assert_eq!(v.as_slice(), VECTOR_ZERO);

    let v: Vec3 = math_traits::Vec3::one();
    assert_eq!(v.as_slice(), VECTOR_ONE);

    let q1: Quat = math_traits::Quat::identity();
    //The storage order for nalgebra is [ i, j, k, w ] while the arguments for this functions are in the order (w, i, j, k).
    let q2 = nalgebra::Quaternion::<f64>::new(1.0, 0.0, 0.0, 0.0);
    assert_eq!(q1.as_slice(), q2.coords.as_slice());
    assert_eq!(q1.as_slice(), QUAT_IDENTITY);
}
