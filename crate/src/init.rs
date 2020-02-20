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
    };

    world.add_unique(TransformRoot(id));

    let (mut translations,mut rotations,mut scales) = world.borrow::<( &mut Translation, &mut Rotation, &mut Scale)>();
    translations.update_pack();
    rotations.update_pack();
    scales.update_pack();

    id
}