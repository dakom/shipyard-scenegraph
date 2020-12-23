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
    let id = world.borrow::<AllStoragesViewMut>().unwrap().add_entity((
        Translation::new(V::zero()),
        Rotation::new(Q::identity()),
        Scale::new(V::one()),
        Origin::new(V::zero()),
        LocalTransform::new(M::identity()),
        WorldTransform::new(M::identity()),
        DirtyTransform(false)
    ));

    world.borrow::<ViewMut<LocalTransform<M, N>>>().unwrap().update_pack();
    world.borrow::<ViewMut<Translation<V, N>>>().unwrap().update_pack();
    world.borrow::<ViewMut<Rotation<Q, N>>>().unwrap().update_pack();
    world.borrow::<ViewMut<Scale<V, N>>>().unwrap().update_pack();
    world.borrow::<ViewMut<Origin<V, N>>>().unwrap().update_pack();

    world.add_unique(TransformRoot(id)).unwrap();

    id
}