use shipyard::prelude::*;
use shipyard_scenegraph::*;
use std::collections::HashMap;
#[cfg(feature = "nalgebra")]
use nalgebra;

use cfg_if::cfg_if;
cfg_if! {
    if #[cfg(all(not(feature = "nalgebra_transforms"), feature = "nalgebra"))] {
        #[test]
        fn test_nalgebra_owned() {
            let m1:nalgebra::Matrix4<f64> = Matrix4::identity().into(); 
            let m2:Matrix4 = nalgebra::Matrix4::<f64>::identity().into(); 
            assert_eq!(m1.as_slice(), m2.as_slice());

            let v1:nalgebra::Vector3<f64> = Vec3::identity().into(); 
            let v2:Vec3 = nalgebra::Vector3::<f64>::new(0.0, 0.0, 0.0).into();
            assert_eq!(v1.as_slice(), v2.as_slice());

            let q1:nalgebra::Quaternion<f64> = Quat::identity().into(); 
            let q2:Quat = nalgebra::Quaternion::<f64>::new(0.0, 0.0, 0.0, 1.0).into();
            assert_eq!(q1.coords.as_slice(), q2.as_slice());
        }


        #[test]
        fn test_nalgebra_refs() {
            let m1:nalgebra::Matrix4<f64> = (&Matrix4::identity()).into(); 
            let m2:Matrix4 = (&nalgebra::Matrix4::<f64>::identity()).into(); 
            assert_eq!(m1.as_slice(), m2.as_slice());

            let v1:nalgebra::Vector3<f64> = (&Vec3::identity()).into(); 
            let v2:Vec3 = (&nalgebra::Vector3::<f64>::new(0.0, 0.0, 0.0)).into();
            assert_eq!(v1.as_slice(), v2.as_slice());

            let q1:nalgebra::Quaternion<f64> = (&Quat::identity()).into(); 
            let q2:Quat = (&nalgebra::Quaternion::<f64>::new(0.0, 0.0, 0.0, 1.0)).into();
            assert_eq!(q1.coords.as_slice(), q2.as_slice());
        }
    }
}