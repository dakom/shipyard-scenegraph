use shipyard::prelude::*;
use crate::transform::*;
use std::cmp::Ordering;
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
