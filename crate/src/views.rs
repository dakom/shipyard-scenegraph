use shipyard::*;
use shipyard::borrow::{Borrow, BorrowInfo};
use shipyard_hierarchy::*;
use crate::components::*;
use crate::traits::math::*;
use crate::hierarchy::SceneGraph;

/// Custom view for all scene graph + hierarchy stuff
/// Especially useful for adding/removing items from the tree
pub struct SceneGraphStoragesMut<'a, V, Q, M, N> 
where
    V: Vec3<N> + Send + Sync + 'static,
    Q: Quat<N> + Send + Sync + 'static,
    M: Matrix4<N> + Send + Sync + 'static,
    N: Copy + Send + Sync + 'static
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
    type View = Self;

    fn borrow(world: &'a World, last_run: Option<u32>, current: u32) -> Result<Self, error::GetStorage>
    where
        Self: Sized,
    {
        Ok(SceneGraphStoragesMut {
            entities: Borrow::borrow(world, last_run, current)?,
            transform_root: Borrow::borrow(world, last_run, current)?,
            parents: Borrow::borrow(world, last_run, current)?,
            children: Borrow::borrow(world, last_run, current)?,
            translations: Borrow::borrow(world, last_run, current)?,
            rotations: Borrow::borrow(world, last_run, current)?,
            scales: Borrow::borrow(world, last_run, current)?,
            origins: Borrow::borrow(world, last_run, current)?,
            local_transforms: Borrow::borrow(world, last_run, current)?,
            world_transforms: Borrow::borrow(world, last_run, current)?,
            dirty_transforms: Borrow::borrow(world, last_run, current)?,
        })
    }
}

unsafe impl<'a, V, Q, M, N> BorrowInfo for SceneGraphStoragesMut<'a, V, Q, M, N> 
where
    V: Vec3<N> + Send + Sync + 'static,
    Q: Quat<N> + Send + Sync + 'static,
    M: Matrix4<N> + Send + Sync + 'static,
    N: Copy + Send + Sync + 'static
{
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

    fn all_borrow(all_storages: &'a AllStorages, last_run: Option<u32>, current: u32) -> Result<Self, error::GetStorage>
    where
        Self: Sized,
    {
        Ok(SceneGraphStoragesMut {
            entities: AllStoragesBorrow::all_borrow(all_storages, last_run, current)?,
            transform_root: AllStoragesBorrow::all_borrow(all_storages, last_run, current)?,
            parents: AllStoragesBorrow::all_borrow(all_storages, last_run, current)?,
            children: AllStoragesBorrow::all_borrow(all_storages, last_run, current)?,
            translations: AllStoragesBorrow::all_borrow(all_storages, last_run, current)?,
            rotations: AllStoragesBorrow::all_borrow(all_storages, last_run, current)?,
            scales: AllStoragesBorrow::all_borrow(all_storages, last_run, current)?,
            origins: AllStoragesBorrow::all_borrow(all_storages, last_run, current)?,
            local_transforms: AllStoragesBorrow::all_borrow(all_storages, last_run, current)?,
            world_transforms: AllStoragesBorrow::all_borrow(all_storages, last_run, current)?,
            dirty_transforms: AllStoragesBorrow::all_borrow(all_storages, last_run, current)?,
        })
    }
}


/// Custom view for local transforms without the hierarchy 
/// Useful for easily setting translation, rotation, and scale all at once
pub struct LocalTransformStoragesMut<'a, V, Q, M, N> 
where
    V: Vec3<N> + Send + Sync + 'static,
    Q: Quat<N> + Send + Sync + 'static,
    M: Matrix4<N> + Send + Sync + 'static,
    N: Copy + Send + Sync + 'static
{
    pub translations: ViewMut<'a, Translation<V, N>>,
    pub rotations: ViewMut<'a, Rotation<Q, N>>,
    pub scales: ViewMut<'a, Scale<V, N>>,
    pub origins: ViewMut<'a, Origin<V, N>>,
    pub local_transforms: ViewMut<'a, LocalTransform<M, N>>,
}


// this impl lets you use it with `World::borrow`, `World::run` and in workloads
impl<'a, V, Q, M, N> Borrow<'a> for LocalTransformStoragesMut<'a, V, Q, M, N> 
where
    V: Vec3<N> + Send + Sync + 'static,
    Q: Quat<N> + Send + Sync + 'static,
    M: Matrix4<N> + Send + Sync + 'static,
    N: Copy + Send + Sync + 'static

{
    type View = Self;

    fn borrow(world: &'a World, last_run: Option<u32>, current: u32) -> Result<Self, error::GetStorage>

    where
        Self: Sized,
    {
        Ok(LocalTransformStoragesMut{
            translations: Borrow::borrow(world, last_run, current)?,
            rotations: Borrow::borrow(world, last_run, current)?,
            scales: Borrow::borrow(world, last_run, current)?,
            origins: Borrow::borrow(world, last_run, current)?,
            local_transforms: Borrow::borrow(world, last_run, current)?,
        })
    }
}

unsafe impl<'a, V, Q, M, N> BorrowInfo for LocalTransformStoragesMut<'a, V, Q, M, N> 
where
    V: Vec3<N> + Send + Sync + 'static,
    Q: Quat<N> + Send + Sync + 'static,
    M: Matrix4<N> + Send + Sync + 'static,
    N: Copy + Send + Sync + 'static

{
    fn borrow_info(infos: &mut Vec<info::TypeInfo>) {
        EntitiesViewMut::borrow_info(infos);
        ViewMut::<Translation<V, N>>::borrow_info(infos);
        ViewMut::<Rotation<Q, N>>::borrow_info(infos);
        ViewMut::<Scale<V, N>>::borrow_info(infos);
        ViewMut::<Origin<V, N>>::borrow_info(infos);
        ViewMut::<LocalTransform<M, N>>::borrow_info(infos);
    }
}

// this impl lets you use it with `AllStorages::borrow`, `AllStorages::run`
impl<'a, V, Q, M, N> AllStoragesBorrow<'a> for LocalTransformStoragesMut<'a, V, Q, M, N> 

where
    V: Vec3<N> + Send + Sync + 'static,
    Q: Quat<N> + Send + Sync + 'static,
    M: Matrix4<N> + Send + Sync + 'static,
    N: Copy + Send + Sync + 'static
{

    fn all_borrow(all_storages: &'a AllStorages, last_run: Option<u32>,current: u32,) -> Result<Self, error::GetStorage>
    where
        Self: Sized,
    {
        Ok(LocalTransformStoragesMut {
            translations: AllStoragesBorrow::all_borrow(all_storages, last_run, current)?,
            rotations: AllStoragesBorrow::all_borrow(all_storages, last_run, current)?,
            scales: AllStoragesBorrow::all_borrow(all_storages, last_run, current)?,
            origins: AllStoragesBorrow::all_borrow(all_storages, last_run, current)?,
            local_transforms: AllStoragesBorrow::all_borrow(all_storages, last_run, current)?,
        })
    }
}

impl<'a: 'b, 'b, V, Q, M, N> Get for &'b LocalTransformStoragesMut<'a, V, Q, M, N>
where
    V: Vec3<N> + Send + Sync + 'static,
    Q: Quat<N> + Send + Sync + 'static,
    M: Matrix4<N> + Send + Sync + 'static,
    N: Copy + Send + Sync + 'static
{
    type Out = (
        <&'b ViewMut<'a, Translation<V, N>> as Get>::Out,
        <&'b ViewMut<'a, Rotation<Q, N>> as Get>::Out,
        <&'b ViewMut<'a, Scale<V, N>> as Get>::Out,
        <&'b ViewMut<'a, Origin<V, N>> as Get>::Out,
        <&'b ViewMut<'a, LocalTransform<M, N>> as Get>::Out
    );

    fn get(self, entity: EntityId) -> Result<Self::Out, shipyard::error::MissingComponent> {
        Ok((
            self.translations.get(entity)?,
            self.rotations.get(entity)?,
            self.scales.get(entity)?,
            self.origins.get(entity)?,
            self.local_transforms.get(entity)?,
    ))
    }

}


impl<'a: 'b, 'b, V, Q, M, N> Get for &'b mut LocalTransformStoragesMut<'a, V, Q, M, N>
where
    V: Vec3<N> + Send + Sync + 'static,
    Q: Quat<N> + Send + Sync + 'static,
    M: Matrix4<N> + Send + Sync + 'static,
    N: Copy + Send + Sync + 'static
{
    type Out = (
        <&'b mut ViewMut<'a, Translation<V, N>> as Get>::Out,
        <&'b mut ViewMut<'a, Rotation<Q, N>> as Get>::Out,
        <&'b mut ViewMut<'a, Scale<V, N>> as Get>::Out,
        <&'b mut ViewMut<'a, Origin<V, N>> as Get>::Out,
        <&'b mut ViewMut<'a, LocalTransform<M, N>> as Get>::Out
    );
    fn get(self, entity: EntityId) -> Result<Self::Out, shipyard::error::MissingComponent> {
        Ok((
            (&mut self.translations).get(entity)?,
            (&mut self.rotations).get(entity)?,
            (&mut self.scales).get(entity)?,
            (&mut self.origins).get(entity)?,
            (&mut self.local_transforms).get(entity)?,
    ))
    }

}
