use crate::components::*;
use crate::traits::required::*;
use shipyard::*;

pub fn init_scenegraph<V, Q, M, N>(world: &World) -> EntityId
where
    V: Vec3Ext<N> + Send + Sync + 'static,
    Q: QuatExt<N> + Send + Sync + 'static,
    M: Matrix4Ext<N> + Send + Sync + 'static,
    N: Copy + Send + Sync + 'static,
{
    let id = world.borrow::<AllStoragesViewMut>().unwrap().add_entity((
        Translation::new(V::zero()),
        Rotation::new(Q::identity()),
        Scale::new(V::one()),
        Origin::new(V::zero()),
        LocalTransform::new(M::identity()),
        WorldTransform::new(M::identity()),
        DirtyTransform(false),
    ));

    world.add_unique(TransformRoot(id));

    id
}
