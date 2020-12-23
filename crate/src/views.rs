use shipyard::*;
use shipyard::Borrow;
use shipyard_hierarchy::*;
use crate::components::*;
use crate::traits::math::*;
use crate::hierarchy::SceneGraph;

/// Custom view for all scene graph + hierarchy stuff
/// Especially useful for adding/removing items from the tree
pub struct SceneGraphStoragesMut<'a, V, Q, M, N> 
where
    V: Vec3<N> + Send + Sync,
    Q: Quat<N> + Send + Sync,
    M: Matrix4<N> + Send + Sync,
    N: Copy + Send + Sync
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

// this impl lets you use it with `World::borrow`, `World::run` and in workloads
impl<'a, V, Q, M, N> Borrow<'a> for SceneGraphStoragesMut<'a, V, Q, M, N> 
where
    V: Vec3<N> + Send + Sync + 'static,
    Q: Quat<N> + Send + Sync + 'static,
    M: Matrix4<N> + Send + Sync + 'static,
    N: Copy + Send + Sync + 'static

{
    fn try_borrow(world: &'a World) -> Result<Self, error::GetStorage>
    where
        Self: Sized,
    {
        Ok(SceneGraphStoragesMut {
            entities: Borrow::try_borrow(world)?,
            transform_root: Borrow::try_borrow(world)?,
            parents: Borrow::try_borrow(world)?,
            children: Borrow::try_borrow(world)?,
            translations: Borrow::try_borrow(world)?,
            rotations: Borrow::try_borrow(world)?,
            scales: Borrow::try_borrow(world)?,
            origins: Borrow::try_borrow(world)?,
            local_transforms: Borrow::try_borrow(world)?,
            world_transforms: Borrow::try_borrow(world)?,
            dirty_transforms: Borrow::try_borrow(world)?,
        })
    }

    fn borrow_info(infos: &mut Vec<info::TypeInfo>) {
        EntitiesViewMut::borrow_info(infos);
        UniqueView::<TransformRoot>::borrow_info(infos);
        ViewMut::<Parent<SceneGraph>>::borrow_info(infos);
        ViewMut::<Child<SceneGraph>>::borrow_info(infos);
        ViewMut::<Translation<V, N>>::borrow_info(infos);
        ViewMut::<Rotation<Q, N>>::borrow_info(infos);
        ViewMut::<Scale<V, N>>::borrow_info(infos);
        ViewMut::<Origin<V, N>>::borrow_info(infos);
        ViewMut::<LocalTransform<M, N>>::borrow_info(infos);
        ViewMut::<WorldTransform<M, N>>::borrow_info(infos);
        ViewMut::<DirtyTransform>::borrow_info(infos);
    }
}

// this impl lets you use it with `AllStorages::borrow`, `AllStorages::run`
impl<'a, V, Q, M, N> AllStoragesBorrow<'a> for SceneGraphStoragesMut<'a, V, Q, M, N> 

where
    V: Vec3<N> + Send + Sync + 'static,
    Q: Quat<N> + Send + Sync + 'static,
    M: Matrix4<N> + Send + Sync + 'static,
    N: Copy + Send + Sync + 'static
{
    fn try_borrow(all_storages: &'a AllStorages) -> Result<Self, error::GetStorage>
    where
        Self: Sized,
    {
        Ok(SceneGraphStoragesMut {
            entities: AllStoragesBorrow::try_borrow(all_storages)?,
            transform_root: AllStoragesBorrow::try_borrow(all_storages)?,
            parents: AllStoragesBorrow::try_borrow(all_storages)?,
            children: AllStoragesBorrow::try_borrow(all_storages)?,
            translations: AllStoragesBorrow::try_borrow(all_storages)?,
            rotations: AllStoragesBorrow::try_borrow(all_storages)?,
            scales: AllStoragesBorrow::try_borrow(all_storages)?,
            origins: AllStoragesBorrow::try_borrow(all_storages)?,
            local_transforms: AllStoragesBorrow::try_borrow(all_storages)?,
            world_transforms: AllStoragesBorrow::try_borrow(all_storages)?,
            dirty_transforms: AllStoragesBorrow::try_borrow(all_storages)?,
        })
    }
}


/// Custom view for local transforms without the hierarchy 
/// Useful for easily setting translation, rotation, and scale all at once
pub struct TrsStoragesMut<'a, V, Q, N> 
where
    V: Vec3<N> + Send + Sync,
    Q: Quat<N> + Send + Sync,
    N: Copy + Send + Sync
{
    pub translations: ViewMut<'a, Translation<V, N>>,
    pub rotations: ViewMut<'a, Rotation<Q, N>>,
    pub scales: ViewMut<'a, Scale<V, N>>,
    pub origins: ViewMut<'a, Origin<V, N>>,
}


// this impl lets you use it with `World::borrow`, `World::run` and in workloads
impl<'a, V, Q, N> Borrow<'a> for TrsStoragesMut<'a, V, Q, N> 
where
    V: Vec3<N> + Send + Sync + 'static,
    Q: Quat<N> + Send + Sync + 'static,
    N: Copy + Send + Sync + 'static

{
    fn try_borrow(world: &'a World) -> Result<Self, error::GetStorage>
    where
        Self: Sized,
    {
        Ok(TrsStoragesMut{
            translations: Borrow::try_borrow(world)?,
            rotations: Borrow::try_borrow(world)?,
            scales: Borrow::try_borrow(world)?,
            origins: Borrow::try_borrow(world)?,
        })
    }

    fn borrow_info(infos: &mut Vec<info::TypeInfo>) {
        EntitiesViewMut::borrow_info(infos);
        ViewMut::<Translation<V, N>>::borrow_info(infos);
        ViewMut::<Rotation<Q, N>>::borrow_info(infos);
        ViewMut::<Scale<V, N>>::borrow_info(infos);
        ViewMut::<Origin<V, N>>::borrow_info(infos);
    }
}

// this impl lets you use it with `AllStorages::borrow`, `AllStorages::run`
impl<'a, V, Q, N> AllStoragesBorrow<'a> for TrsStoragesMut<'a, V, Q, N> 

where
    V: Vec3<N> + Send + Sync + 'static,
    Q: Quat<N> + Send + Sync + 'static,
    N: Copy + Send + Sync + 'static
{
    fn try_borrow(all_storages: &'a AllStorages) -> Result<Self, error::GetStorage>
    where
        Self: Sized,
    {
        Ok(TrsStoragesMut {
            translations: AllStoragesBorrow::try_borrow(all_storages)?,
            rotations: AllStoragesBorrow::try_borrow(all_storages)?,
            scales: AllStoragesBorrow::try_borrow(all_storages)?,
            origins: AllStoragesBorrow::try_borrow(all_storages)?,
        })
    }
}
