use crate::components::*;
use crate::hierarchy::SceneGraph;
use crate::traits::math::*;
use shipyard::*;
use shipyard_hierarchy::*;

/// Custom view for all scene graph + hierarchy stuff
/// Especially useful for adding/removing items from the tree
#[derive(Borrow, AllStoragesBorrow, BorrowInfo)]
pub struct SceneGraphStoragesMut<'a, V, Q, M, N>
where
    V: Vec3<N> + Send + Sync + 'static,
    Q: Quat<N> + Send + Sync + 'static,
    M: Matrix4<N> + Send + Sync + 'static,
    N: Copy + Send + Sync + 'static,
{
    pub entities: EntitiesViewMut<'a>,
    pub transform_root: UniqueView<'a, TransformRoot>,
    pub parents: ViewMut<'a, Parent<SceneGraph>>,
    pub children: ViewMut<'a, Child<SceneGraph>>,
    pub translations: ViewMut<'a, Translation<V, N>>,
    pub rotations: ViewMut<'a, Rotation<Q, N>>,
    pub scales: ViewMut<'a, Scale<V, N>>,
    pub origins: ViewMut<'a, Origin<V, N>>,
    pub local_transforms: ViewMut<'a, LocalTransform<M, N>>,
    pub world_transforms: ViewMut<'a, WorldTransform<M, N>>,
    pub dirty_transforms: ViewMut<'a, DirtyTransform>,
}

/// Custom view for local transforms without the hierarchy
/// Useful for easily setting translation, rotation, and scale all at once
#[derive(Borrow, AllStoragesBorrow, BorrowInfo)]
pub struct LocalTransformStoragesMut<'a, V, Q, M, N>
where
    V: Vec3<N> + Send + Sync + 'static,
    Q: Quat<N> + Send + Sync + 'static,
    M: Matrix4<N> + Send + Sync + 'static,
    N: Copy + Send + Sync + 'static,
{
    pub translations: ViewMut<'a, Translation<V, N>>,
    pub rotations: ViewMut<'a, Rotation<Q, N>>,
    pub scales: ViewMut<'a, Scale<V, N>>,
    pub origins: ViewMut<'a, Origin<V, N>>,
    pub local_transforms: ViewMut<'a, LocalTransform<M, N>>,
}

/// Helper for easily getting local transform data for a given entity id
impl<'a: 'b, 'b, V, Q, M, N> Get for &'b mut LocalTransformStoragesMut<'a, V, Q, M, N>
where
    V: Vec3<N> + Send + Sync + 'static,
    Q: Quat<N> + Send + Sync + 'static,
    M: Matrix4<N> + Send + Sync + 'static,
    N: Copy + Send + Sync + 'static,
{
    type Out = LocalTransformDataMut<'b, V, Q, M, N>;

    fn get(self, entity: EntityId) -> Result<Self::Out, shipyard::error::MissingComponent> {
        Ok(LocalTransformDataMut {
            translation: (&mut self.translations).get(entity)?,
            rotation: (&mut self.rotations).get(entity)?,
            scale: (&mut self.scales).get(entity)?,
            origin: (&mut self.origins).get(entity)?,
            local_transform: (&mut self.local_transforms).get(entity)?,
        })
    }
}

pub struct LocalTransformDataMut<'a, V, Q, M, N>
where
    V: Vec3<N> + Send + Sync + 'static,
    Q: Quat<N> + Send + Sync + 'static,
    M: Matrix4<N> + Send + Sync + 'static,
    N: Copy + Send + Sync + 'static,
{
    pub translation: Mut<'a, Translation<V, N>>,
    pub rotation: Mut<'a, Rotation<Q, N>>,
    pub scale: Mut<'a, Scale<V, N>>,
    pub origin: Mut<'a, Origin<V, N>>,
    pub local_transform: Mut<'a, LocalTransform<M, N>>,
}
