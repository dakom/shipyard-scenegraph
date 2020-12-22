use shipyard::*;
use crate::components::*;
use crate::math::traits::*;


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

    let (mut translations,mut rotations,mut scales, mut origins) 
        = world.borrow::<( 
            ViewMut<Translation<V, N>>, 
            ViewMut<Rotation<Q, N>>, 
            ViewMut<Scale<V, N>>, 
            ViewMut<Origin<V, N>>
        )>().unwrap();

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

#[cfg(test)]
mod tests {
    use shipyard::*;
    use crate::math::native::*;
    use std::borrow::Borrow;
    type Translation = crate::components::Translation<Vec3, f64>;

    #[test]
    fn quick_sanity_check() {
        let world = World::new();

        super::init_scenegraph::<Vec3, Vec4, Matrix4, f64>(&world);

        let translations = world.borrow::<View<Translation>>().unwrap(); 

        let value = translations.iter().next().unwrap();
        assert_eq!(&Vec3::new(0.0, 0.0, 0.0), value.borrow()); 
    }
}