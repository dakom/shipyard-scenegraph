use shipyard::EntityId;
use crate::math::*;
#[cfg(feature = "easy_deref")]
use derive_deref::{Deref, DerefMut};

#[cfg_attr(feature = "easy_deref", derive(Deref, DerefMut))]
pub struct TransformRoot(pub EntityId);

#[cfg_attr(feature = "easy_deref", derive(Deref, DerefMut))]
#[derive(Debug)]
pub struct Translation(pub Vec3);

#[cfg_attr(feature = "easy_deref", derive(Deref, DerefMut))]
#[derive(Debug)]
pub struct Rotation(pub Quat);

#[cfg_attr(feature = "easy_deref", derive(Deref, DerefMut))]
#[derive(Debug)]
pub struct Scale(pub Vec3);

#[cfg_attr(feature = "easy_deref", derive(Deref, DerefMut))]
#[derive(Debug)]
pub struct LocalTransform(pub Matrix4);

#[cfg_attr(feature = "easy_deref", derive(Deref, DerefMut))]
#[derive(Debug)]
pub struct WorldTransform(pub Matrix4);

#[cfg_attr(feature = "easy_deref", derive(Deref, DerefMut))]
#[derive(Debug)]
pub struct DirtyTransform(pub bool);