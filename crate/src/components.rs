use shipyard::EntityId;
use crate::math::*;

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