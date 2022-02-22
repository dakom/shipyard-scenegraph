use shipyard_scenegraph::traits::required as math_traits;
use shipyard_scenegraph::traits::required::SliceExt;

const MATRIX_IDENTITY: [f64; 16] = [
    1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
];

const VECTOR_ZERO: [f64; 3] = [0.0, 0.0, 0.0];
const VECTOR_ONE: [f64; 3] = [1.0, 1.0, 1.0];

const QUAT_IDENTITY: [f64; 4] = [0.0, 0.0, 0.0, 1.0];

#[test]
fn test_nalgebra() {
    let m: nalgebra::Matrix4<f64> = math_traits::Matrix4Ext::identity();
    assert_eq!(m.as_slice(), MATRIX_IDENTITY);

    let v: nalgebra::Vector3<f64> = math_traits::Vec3Ext::zero();
    assert_eq!(v.as_slice(), VECTOR_ZERO);

    let v: nalgebra::Vector3<f64> = math_traits::Vec3Ext::one();
    assert_eq!(v.as_slice(), VECTOR_ONE);

    let q1: nalgebra::UnitQuaternion<f64> = math_traits::QuatExt::identity();
    //The storage order for nalgebra is [ i, j, k, w ] while the arguments for this functions are in the order (w, i, j, k).
    let q2 = nalgebra::Quaternion::<f64>::new(1.0, 0.0, 0.0, 0.0);
    assert_eq!(q1.as_slice(), q2.coords.as_slice());
    assert_eq!(q1.as_slice(), QUAT_IDENTITY);
}
