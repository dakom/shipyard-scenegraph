/*
    These are just convenience functions to operate one-off on Worlds
    All they do is take a borrow of the required storages and
    Call the same-named method on that tuple 
*/
use shipyard::prelude::*;
use crate::transform::*;
use crate::hierarchy::*;

pub fn spawn_child(world:&World, parent: EntityId, translation: Option<Vec3>, rotation: Option<Quat>, scale: Option<Vec3>) -> EntityId {

    let mut storages = world.borrow::<(EntitiesMut, &mut Parent, &mut Child, &mut Translation, &mut Rotation, &mut Scale, &mut LocalTransform, &mut WorldTransform)>();
    let mut storages:TransformHierarchyStoragesMut = (&mut storages.0, &mut storages.1, &mut storages.2, &mut storages.3, &mut storages.4,&mut storages.5,&mut storages.6,&mut storages.7);

    storages.spawn_child(parent, translation, rotation, scale)
}


pub fn set_trs(world:&World, entity:EntityId, translation: Option<Vec3>, rotation: Option<Quat>, scale: Option<Vec3>) {
    let mut storages = world.borrow::<(EntitiesMut, &mut Translation, &mut Rotation, &mut Scale, &mut LocalTransform, &mut WorldTransform)>();
    let mut storages:TransformStoragesMut = (&mut storages.0, &mut storages.1, &mut storages.2, &mut storages.3, &mut storages.4,&mut storages.5);

    storages.set_trs(entity, translation, rotation, scale);
}

