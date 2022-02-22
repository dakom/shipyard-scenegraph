use shipyard_scenegraph::traits::required as math_traits;
use shipyard_scenegraph::traits::required::SliceExt;

const MATRIX_IDENTITY: [f32; 16] = [
    1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
];

const VECTOR_ZERO: [f32; 3] = [0.0, 0.0, 0.0];
const VECTOR_ONE: [f32; 3] = [1.0, 1.0, 1.0];

const QUAT_IDENTITY: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

#[test]
fn test_nalgebra_glm() {
    let m: nalgebra_glm::Mat4 = math_traits::Matrix4Ext::identity();
    assert_eq!(m.as_slice(), MATRIX_IDENTITY);

    let v: nalgebra_glm::Vec3 = math_traits::Vec3Ext::zero();
    assert_eq!(v.as_slice(), VECTOR_ZERO);

    let v: nalgebra_glm::Vec3 = math_traits::Vec3Ext::one();
    assert_eq!(v.as_slice(), VECTOR_ONE);

    let q1: nalgebra_glm::Quat = math_traits::QuatExt::identity();
    //The storage order for nalgebra is [ i, j, k, w ] while the arguments for this functions are in the order (w, i, j, k).
    let q2 = nalgebra_glm::Quat::new(1.0, 0.0, 0.0, 0.0);
    assert_eq!(q1.as_slice(), q2.coords.as_slice());
    assert_eq!(q1.as_slice(), QUAT_IDENTITY);
}
