use shipyard::prelude::*;

pub type Vec3 = nalgebra::Vector3<f64>;
pub type Quat = nalgebra::UnitQuaternion<f64>;
pub type Matrix4 = nalgebra::Matrix4<f64>;

pub struct TransformRoot(pub EntityId);

#[derive(Debug)]
pub struct Translation(pub Vec3);
#[derive(Debug)]
pub struct Rotation(pub Quat);
#[derive(Debug)]
pub struct Scale(pub Vec3);
#[derive(Debug)]
pub struct LocalTransform(pub Matrix4);
#[derive(Debug)]
pub struct WorldTransform(pub Matrix4);
#[derive(Debug)]
pub struct DirtyTransform(pub bool);