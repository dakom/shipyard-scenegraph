use shipyard::prelude::*;
use crate::transform::*;

pub fn init(world:&World) -> EntityId {
    let translation = Vec3::default();
    let rotation = Quat::default();
    let scale = Vec3::new(1.0, 1.0, 1.0);
    let local_matrix = Matrix4::default(); 
    let world_matrix = Matrix4::default();

    let id = {
        let (
            mut entities,
            mut translations,
            mut rotations,
            mut scales,
            mut local_transforms,
            mut world_transforms,
            mut dirty_transforms
        ) = world.borrow::<(
            EntitiesMut, 
            &mut Translation,
            &mut Rotation,
            &mut Scale,
            &mut LocalTransform,
            &mut WorldTransform,
            &mut DirtyTransform,
        )>();

        entities.add_entity( 
            (
                &mut translations,
                &mut rotations,
                &mut scales,
                &mut local_transforms,
                &mut world_transforms,
                &mut dirty_transforms
            ),
            (
                Translation(translation),
                Rotation(rotation),
                Scale(scale),
                LocalTransform(local_matrix),
                WorldTransform(world_matrix),
                DirtyTransform(false)
            )
        )
    };

    world.add_unique(TransformRoot(id));

    let (mut translations,mut rotations,mut scales) = world.borrow::<( &mut Translation, &mut Rotation, &mut Scale)>();
    translations.update_pack();
    rotations.update_pack();
    scales.update_pack();

    translations.clear_inserted_and_modified();
    rotations.clear_inserted_and_modified();
    scales.clear_inserted_and_modified();

    id
}