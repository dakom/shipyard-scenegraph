use shipyard::*;
use crate::components::*;
use crate::math::*;

pub fn init(world:&World) -> EntityId {
    let translation = Vec3::zero();
    let rotation = Quat::identity();
    let scale = Vec3::new(1.0, 1.0, 1.0);
    let origin = Vec3::zero();
    let local_matrix = Matrix4::identity(); 
    let world_matrix = Matrix4::identity();

    let id = {
        let (
            mut entities,
            mut translations,
            mut rotations,
            mut scales,
            mut origins,
            mut local_transforms,
            mut world_transforms,
            mut dirty_transforms
        ) = world.borrow::<(
            EntitiesViewMut, 
            ViewMut<Translation>,
            ViewMut<Rotation>,
            ViewMut<Scale>,
            ViewMut<Origin>,
            ViewMut<LocalTransform>,
            ViewMut<WorldTransform>,
            ViewMut<DirtyTransform>,
        )>();

        entities.add_entity( 
            (
                &mut translations,
                &mut rotations,
                &mut scales,
                &mut origins,
                &mut local_transforms,
                &mut world_transforms,
                &mut dirty_transforms
            ),
            (
                Translation(translation),
                Rotation(rotation),
                Scale(scale),
                Origin(origin),
                LocalTransform(local_matrix),
                WorldTransform(world_matrix),
                DirtyTransform(false)
            )
        )
    };

    world.add_unique(TransformRoot(id));

    let (mut translations,mut rotations,mut scales, mut origins) = world.borrow::<( ViewMut<Translation>, ViewMut<Rotation>, ViewMut<Scale>, ViewMut<Origin>)>();
    translations.update_pack();
    rotations.update_pack();
    scales.update_pack();
    origins.update_pack();

    translations.clear_inserted_and_modified();
    rotations.clear_inserted_and_modified();
    scales.clear_inserted_and_modified();
    origins.clear_inserted_and_modified();

    id
}