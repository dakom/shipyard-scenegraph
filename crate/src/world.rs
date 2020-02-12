use shipyard::prelude::*;
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
    let local_matrix = Matrix4::from_trs(&translation, &rotation, &scale);
    let world_matrix = Matrix4::default();

    log::info!("{:#?}", translation.as_ref());
    log::info!("{:#?}", rotation.as_ref());
    log::info!("{:#?}", scale.as_ref());
    log::info!("{:#?}", local_matrix.as_ref());
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