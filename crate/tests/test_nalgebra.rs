cfg_if::cfg_if! {
    if #[cfg(all(not(feature = "nalgebra_transforms"), feature = "nalgebra"))] {
        #[test]
        fn test_nalgebra_owned() {
            use shipyard_scenegraph::*;
            use nalgebra;

            let m1:nalgebra::Matrix4<f64> = Matrix4::identity().into(); 
            let m2:Matrix4 = nalgebra::Matrix4::<f64>::identity().into(); 
            assert_eq!(m1.as_slice(), m2.as_slice());

            let v1:nalgebra::Vector3<f64> = Vec3::zero().into(); 
            let v2:Vec3 = nalgebra::Vector3::<f64>::new(0.0, 0.0, 0.0).into();
            assert_eq!(v1.as_slice(), v2.as_slice());

            let q1:nalgebra::Quaternion<f64> = Quat::identity().into(); 
            //The storage order is [ i, j, k, w ] while the arguments for this functions are in the order (w, i, j, k).
            let q2:Quat = nalgebra::Quaternion::<f64>::new(1.0, 0.0, 0.0, 0.0).into();
            let q3:Quat = nalgebra::Quaternion::<f64>::identity().into();
            println!("{:?}", q1.coords.as_slice());

            assert_eq!(q1.coords.as_slice(), q2.as_slice());
            assert_eq!(q2.as_slice(), q3.as_slice());
        }

        #[test]
        fn test_nalgebra_refs() {
            use shipyard_scenegraph::*;
            use nalgebra;

            let m1:nalgebra::Matrix4<f64> = (&Matrix4::identity()).into(); 
            let m2:Matrix4 = (&nalgebra::Matrix4::<f64>::identity()).into(); 
            assert_eq!(m1.as_slice(), m2.as_slice());

            let v1:nalgebra::Vector3<f64> = (&Vec3::zero()).into(); 
            let v2:Vec3 = (&nalgebra::Vector3::<f64>::new(0.0, 0.0, 0.0)).into();
            assert_eq!(v1.as_slice(), v2.as_slice());

            let q1:nalgebra::Quaternion<f64> = (&Quat::identity()).into(); 
            //The storage order is [ i, j, k, w ] while the arguments for this functions are in the order (w, i, j, k).
            let q2:Quat = (&nalgebra::Quaternion::<f64>::new(1.0, 0.0, 0.0, 0.0)).into();
            let q3:Quat = (&nalgebra::Quaternion::<f64>::identity()).into();
            assert_eq!(q1.coords.as_slice(), q2.as_slice());
            assert_eq!(q2.as_slice(), q3.as_slice());
        }
    } else if #[cfg(feature = "nalgebra")] {
    }
}