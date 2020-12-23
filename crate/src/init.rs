use shipyard::*;
use crate::components::*;
use crate::traits::math::*;

pub fn init_scenegraph<V, Q, M, N>(world:&World) -> EntityId
where
    V: Vec3<N> + Send + Sync + 'static,
    Q: Quat<N> + Send + Sync + 'static,
    M: Matrix4<N> + Send + Sync + 'static,
    N: Copy + Send + Sync + 'static,
{
    let translation = V::zero();
    let rotation = Q::identity();
    let scale = V::one();
    let origin = V::zero();
    let local_matrix = M::identity(); 
    let world_matrix = M::identity();

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
            ViewMut<Translation<V, N>>,
            ViewMut<Rotation<Q, N>>,
            ViewMut<Scale<V, N>>,
            ViewMut<Origin<V, N>>,
            ViewMut<LocalTransform<M, N>>,
            ViewMut<WorldTransform<M, N>>,
            ViewMut<DirtyTransform>,
        )>().unwrap();

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
                Translation::new(translation),
                Rotation::new(rotation),
                Scale::new(scale),
                Origin::new(origin),
                LocalTransform::new(local_matrix),
                WorldTransform::new(world_matrix),
                DirtyTransform(false)
            )
        )
    };

    world.add_unique(TransformRoot(id)).unwrap();

    let (mut local_transforms, mut translations,mut rotations,mut scales, mut origins) 
        = world.borrow::<( 
            ViewMut<LocalTransform<M, N>>, 
            ViewMut<Translation<V, N>>, 
            ViewMut<Rotation<Q, N>>, 
            ViewMut<Scale<V, N>>, 
            ViewMut<Origin<V, N>>
        )>().unwrap();

    local_transforms.update_pack();
    translations.update_pack();
    rotations.update_pack();
    scales.update_pack();
    origins.update_pack();

    local_transforms.clear_inserted_and_modified();
    translations.clear_inserted_and_modified();
    rotations.clear_inserted_and_modified();
    scales.clear_inserted_and_modified();
    origins.clear_inserted_and_modified();

    id
}