use shipyard::prelude::*;
use std::cmp::Ordering;
use crate::transform::*;

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

pub fn create_entity(world:&World, translation: Option<Vec3>, rotation: Option<Quat>, scale: Option<Vec3>) -> EntityId {
    let translation = translation.unwrap_or_default();
    let rotation = rotation.unwrap_or_default();
    let scale = scale.unwrap_or(Vec3::new(1.0, 1.0, 1.0));
    let local_matrix = Matrix4::new_from_trs(&translation, &rotation, &scale);
    let world_matrix = Matrix4::default();

    let (
        mut entities, 
        mut translations,
        mut rotations,
        mut scales,
        mut local_transforms,
        mut world_transforms
    ) = world.borrow::<(
        EntitiesMut, 
        &mut Translation,
        &mut Rotation,
        &mut Scale,
        &mut LocalTransform,
        &mut WorldTransform,
    )>();


    entities.add_entity( 
        (
            &mut translations,
            &mut rotations,
            &mut scales,
            &mut local_transforms,
            &mut world_transforms
        ),
        (
            Translation(translation),
            Rotation(rotation),
            Scale(scale),
            LocalTransform(local_matrix),
            WorldTransform(world_matrix)
        )
    )
}


pub fn set_entity_trs(world:&World, entity:EntityId, translation: Option<Vec3>, rotation: Option<Quat>, scale: Option<Vec3>) {
    let (
        mut translations,
        mut rotations,
        mut scales,
    ) = world.borrow::<(
        &mut Translation,
        &mut Rotation,
        &mut Scale,
    )>();

    if let Some((t,r,s)) = (&mut translations, &mut rotations, &mut scales).get(entity).iter_mut().next() {
        if let Some(translation) = translation {
            t.0.copy_from(&translation);
        }
        if let Some(rotation) = rotation {
            r.0.copy_from(&rotation);
        }
        if let Some(scale) = scale {
            s.0.copy_from(&scale);
        }
    }
}

//TODO - this probably isn't really needed - and packing might be best left to consumer too
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