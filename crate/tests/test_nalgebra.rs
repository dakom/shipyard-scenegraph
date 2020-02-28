use shipyard::prelude::*;
use shipyard_scenegraph::*;
use std::collections::HashMap;
#[cfg(feature = "nalgebra")]
use nalgebra;

#[test]
#[cfg(feature = "nalgebra")]
fn test_nalgebra_owned() {
    let m1:nalgebra::Matrix4<f64> = Matrix4::default().into(); 
    let m2:Matrix4 = nalgebra::Matrix4::<f64>::identity().into(); 

    let v1:nalgebra::Vector3<f64> = Vec3::default().into(); 
    let v2:Vec3 = nalgebra::Vector3::<f64>::new(0.0, 0.0, 0.0).into();
    assert_eq!(v1.as_slice(), v2.as_slice());
}


#[test]
#[cfg(feature = "nalgebra")]
fn test_nalgebra_refs() {
    let m1:nalgebra::Matrix4<f64> = (&Matrix4::default()).into(); 
    //let m2:Matrix4 = (nalgebra::Matrix4::<f64>::identity()).into(); 

    //let v1:nalgebra::Vector3<f64> = Vec3::default().into(); 
    //let v2:Vec3 = nalgebra::Vector3::<f64>::new(0.0, 0.0, 0.0).into();
    //assert_eq!(v1.as_slice(), v2.as_slice());
}