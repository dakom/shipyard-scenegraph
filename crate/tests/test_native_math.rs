#![allow(dead_code, unused_imports)]
use shipyard_scenegraph::prelude::*;
mod helpers;
use helpers::*;

#[test]
fn test_math_immutable() {
    let mut foo = Matrix4::identity();
    foo.translate(&Vec3::new(10.0, 0.0, 0.0));

    let mut bar = Matrix4::identity();
    bar.translate(&Vec3::new(10.0, 0.0, 0.0));

    let baz = foo * bar;

    let result = get_translation(&baz);

    assert_eq!([20.0, 0.0, 0.0], result.as_slice());
}

#[test]
fn test_math_mutable() {
    let mut foo = Matrix4::identity();
    foo.translate(&Vec3::new(10.0, 0.0, 0.0));

    let mut bar = Matrix4::identity();
    bar.translate(&Vec3::new(10.0, 0.0, 0.0));

    foo *= bar;

    let result = get_translation(&foo);

    assert_eq!([20.0, 0.0, 0.0], result.as_slice());
}

fn get_translation(mat:&Matrix4) -> Vec3 {
    let values = mat.as_slice();
    Vec3::new(values[12], values[13], values[14])
}