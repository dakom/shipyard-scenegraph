use shipyard::prelude::*;
use std::cmp::Ordering;
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

//TODO - this probably isn't really needed - and packing might be best left to consumer too

pub fn pack_storages(world:&World) {
    let (
        mut translations,
        mut rotations,
        mut scales,
        mut local_transforms,
        mut world_transforms
    ) = world.borrow::<(
        &mut Translation,
        &mut Rotation,
        &mut Scale,
        &mut LocalTransform,
        &mut WorldTransform,
    )>();

    (
        &mut translations,
        &mut rotations,
        &mut scales,
        &mut local_transforms,
        &mut world_transforms
    ).tight_pack();
}
pub fn sort_pack_by_translation<F>(world:&World, cmp_fn:F) 
    where F: Fn(&Vec3, &Vec3) -> Ordering
{

    let (
        mut translations,
        mut rotations,
        mut scales,
        mut local_transforms,
        mut world_transforms
    ) = world.borrow::<(
        &mut Translation,
        &mut Rotation,
        &mut Scale,
        &mut LocalTransform,
        &mut WorldTransform,
    )>();

    (
        &mut translations,
        &mut rotations,
        &mut scales,
        &mut local_transforms,
        &mut world_transforms
    ).sort()
    .unstable(|a, b| cmp_fn(&(a.0).0, &(b.0).0));
}

pub fn sort_pack_by_depth_back_to_front(world:&World) {
    sort_pack_by_translation(world, |a, b| a.z.partial_cmp(&b.z).unwrap());
}

pub fn sort_pack_by_depth_front_to_back(world:&World) {
    sort_pack_by_translation(world, |a, b| b.z.partial_cmp(&a.z).unwrap());
}