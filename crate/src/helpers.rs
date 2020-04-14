/*
    These are just convenience functions to operate one-off on Worlds
    All they do is take a borrow of the required storages and
    Call the same-named method on that tuple 
*/
use shipyard::prelude::*;
use shipyard_hierarchy::*;
use crate::math::*;
use crate::components::*;
use crate::hierarchy::*;

pub fn spawn_child(world:&World, parent: Option<EntityId>, translation: Option<Vec3>, rotation: Option<Quat>, scale: Option<Vec3>, origin:Option<Vec3>) -> EntityId {

    let mut entities = world.borrow::<EntitiesMut>();
    let mut hierarchy_storages = world.borrow::<(Unique<&TransformRoot>, &mut Parent, &mut Child)>();
    let mut transform_storages = world.borrow::<(&mut Translation, &mut Rotation, &mut Scale, &mut Origin, &mut LocalTransform, &mut WorldTransform, &mut DirtyTransform)>();
    
    //let mut storages = world.borrow::<(EntitiesMut, Unique<&TransformRoot>, &mut Parent, &mut Child, &mut Translation, &mut Rotation, &mut Scale, &mut Origin, &mut LocalTransform, &mut WorldTransform, &mut DirtyTransform)>();
    //(&mut storages.0, &storages.1, &mut storages.2, &mut storages.3, &mut storages.4,&mut storages.5,&mut storages.6,&mut storages.7, &mut storages.8, &mut storages.9, &mut storages.10);
    let mut storages:TransformHierarchyStoragesMut = (
        &mut entities,
        &mut hierarchy_storages.0, 
        &mut hierarchy_storages.1, 
        &mut hierarchy_storages.2,
        &mut transform_storages.0, 
        &mut transform_storages.1, 
        &mut transform_storages.2, 
        &mut transform_storages.3, 
        &mut transform_storages.4, 
        &mut transform_storages.5, 
        &mut transform_storages.6
    );

    storages.spawn_child(parent, translation, rotation, scale, origin)
}


pub fn set_trs_origin(world:&World, entity:EntityId, translation: Option<Vec3>, rotation: Option<Quat>, scale: Option<Vec3>, origin:Option<Vec3>) {
    let mut storages = world.borrow::<(&mut Translation, &mut Rotation, &mut Scale, &mut Origin)>();
    let mut storages:TransformStoragesMut = (&mut storages.0, &mut storages.1, &mut storages.2, &mut storages.3);

    storages.set_trs_origin(entity, translation, rotation, scale, origin);
}

