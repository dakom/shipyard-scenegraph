use shipyard_scenegraph::math::native::*;
use nalgebra;

#[test]
fn test_nalgebra_vs_native() {

    let m1:nalgebra::Matrix4<f64> = nalgebra::Matrix4::<f64>::identity();
    let m2:Matrix4 = Matrix4::identity();
    assert_eq!(m1.as_slice(), m2.as_slice());

    let v1:nalgebra::Vector3<f64> = nalgebra::Vector3::<f64>::new(0.0, 0.0, 0.0);
    let v2:Vec3 = Vec3::zero(); 
    assert_eq!(v1.as_slice(), v2.as_slice());

    let q1:Vec4 = Vec4::quat_identity();
    //The storage order for nalgebra is [ i, j, k, w ] while the arguments for this functions are in the order (w, i, j, k).
    let q2:nalgebra::Quaternion<f64> = nalgebra::Quaternion::<f64>::new(1.0, 0.0, 0.0, 0.0);
    println!("{:?}", q1.as_slice());
    assert_eq!(q1.as_slice(), q2.coords.as_slice());
}