use shipyard::*;
use shipyard::borrow::{Borrow, BorrowInfo, IntoBorrow};
use shipyard_hierarchy::*;
use crate::components::*;
use crate::traits::math::*;
use crate::hierarchy::SceneGraph;
use core::marker::PhantomData;

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
/////////////////// below here are just impls ///////////////////////
// ideally we'd just #[derive(Borrow, AllStoragesBorrow, BorrowInfo)] on the above
// but the generics make proc macros tricky, so gotta manually implement
////////////////////////////////////////////////////////////////////

pub struct SceneGraphStoragesMutBorrower<V,Q,M,N> {
    v: PhantomData<V>,
    q: PhantomData<Q>,
    m: PhantomData<M>,
    n: PhantomData<N>,
}
impl<'a, V, Q, M, N> ::shipyard::IntoBorrow for SceneGraphStoragesMut<'a, V, Q, M, N> 
where
    V: Vec3<N> + Send + Sync + 'static,
    Q: Quat<N> + Send + Sync + 'static,
    M: Matrix4<N> + Send + Sync + 'static,
    N: Copy + Send + Sync + 'static
{
    type Borrow = SceneGraphStoragesMutBorrower<V,Q,M,N>;
}

pub struct LocalTransformStoragesMutBorrower<V,Q,M,N> {
    v: PhantomData<V>,
    q: PhantomData<Q>,
    m: PhantomData<M>,
    n: PhantomData<N>,
}
impl<'a, V, Q, M, N> ::shipyard::IntoBorrow for LocalTransformStoragesMut<'a, V, Q, M, N> 
where
    V: Vec3<N> + Send + Sync + 'static,
    Q: Quat<N> + Send + Sync + 'static,
    M: Matrix4<N> + Send + Sync + 'static,
    N: Copy + Send + Sync + 'static
{
    type Borrow = LocalTransformStoragesMutBorrower<V,Q,M,N>;
}

// this impl lets you use it with `World::borrow`, `World::run` and in workloads
impl<'a, V, Q, M, N> Borrow<'a> for SceneGraphStoragesMutBorrower<V,Q,M,N>
where
    V: Vec3<N> + Send + Sync + 'static,
    Q: Quat<N> + Send + Sync + 'static,
    M: Matrix4<N> + Send + Sync + 'static,
    N: Copy + Send + Sync + 'static
{
    type View = SceneGraphStoragesMut<'a, V, Q, M, N>;

    fn borrow(world: &'a World, last_run: Option<u32>, current: u32) -> Result<Self::View, error::GetStorage>
    where
        Self: Sized,
    {
        Ok(SceneGraphStoragesMut {
            entities: <EntitiesViewMut<'a> as IntoBorrow>::Borrow::borrow(world, last_run, current)?,
            transform_root:<UniqueView<'a, TransformRoot> as IntoBorrow>::Borrow::borrow(world, last_run, current)?,
            parents: <ViewMut<'a, Parent<SceneGraph>> as IntoBorrow>::Borrow::borrow(world, last_run, current)?,
            children: <ViewMut<'a, Child<SceneGraph>> as IntoBorrow>::Borrow::borrow(world, last_run, current)?,
            translations: <ViewMut<'a, Translation<V, N>> as IntoBorrow>::Borrow::borrow(world, last_run, current)?,
            rotations: <ViewMut<'a, Rotation<Q, N>> as IntoBorrow>::Borrow::borrow(world, last_run, current)?,
            scales: <ViewMut<'a, Scale<V, N>> as IntoBorrow>::Borrow::borrow(world, last_run, current)?,
            origins: <ViewMut<'a, Origin<V, N>> as IntoBorrow>::Borrow::borrow(world, last_run, current)?,
            local_transforms: <ViewMut<'a, LocalTransform<M, N>> as IntoBorrow>::Borrow::borrow(world, last_run, current)?,
            world_transforms: <ViewMut<'a, WorldTransform<M, N>> as IntoBorrow>::Borrow::borrow(world, last_run, current)?,
            dirty_transforms: <ViewMut<'a, DirtyTransform> as IntoBorrow>::Borrow::borrow(world, last_run, current)?,
        })
    }
}

impl<'a, V, Q, M, N> Borrow<'a> for LocalTransformStoragesMutBorrower<V, Q, M, N> 
where
    V: Vec3<N> + Send + Sync + 'static,
    Q: Quat<N> + Send + Sync + 'static,
    M: Matrix4<N> + Send + Sync + 'static,
    N: Copy + Send + Sync + 'static

{
    type View = LocalTransformStoragesMut<'a, V, Q, M, N>;

    fn borrow(world: &'a World, last_run: Option<u32>, current: u32) -> Result<Self::View, error::GetStorage>

    where
        Self: Sized,
    {
        Ok(LocalTransformStoragesMut{
            translations: <ViewMut<'a, Translation<V, N>> as IntoBorrow>::Borrow::borrow(world, last_run, current)?,
            rotations: <ViewMut<'a, Rotation<Q, N>> as IntoBorrow>::Borrow::borrow(world, last_run, current)?,
            scales: <ViewMut<'a, Scale<V, N>> as IntoBorrow>::Borrow::borrow(world, last_run, current)?,
            origins: <ViewMut<'a, Origin<V, N>> as IntoBorrow>::Borrow::borrow(world, last_run, current)?,
            local_transforms: <ViewMut<'a, LocalTransform<M, N>> as IntoBorrow>::Borrow::borrow(world, last_run, current)?,
        })
    }
}

// this impl lets you use it with `AllStorages::borrow`, `AllStorages::run`
impl<'a, V, Q, M, N> AllStoragesBorrow<'a> for SceneGraphStoragesMutBorrower<V, Q, M, N> 
where
    V: Vec3<N> + Send + Sync + 'static,
    Q: Quat<N> + Send + Sync + 'static,
    M: Matrix4<N> + Send + Sync + 'static,
    N: Copy + Send + Sync + 'static
{

    fn all_borrow(all_storages: &'a AllStorages, last_run: Option<u32>, current: u32) -> Result<Self::View, error::GetStorage>
    where
        Self: Sized,
    {
        Ok(SceneGraphStoragesMut {
            entities: <EntitiesViewMut<'a> as IntoBorrow>::Borrow::all_borrow(all_storages, last_run, current)?,
            transform_root:<UniqueView<'a, TransformRoot> as IntoBorrow>::Borrow::all_borrow(all_storages, last_run, current)?,
            parents: <ViewMut<'a, Parent<SceneGraph>> as IntoBorrow>::Borrow::all_borrow(all_storages, last_run, current)?,
            children: <ViewMut<'a, Child<SceneGraph>> as IntoBorrow>::Borrow::all_borrow(all_storages, last_run, current)?,
            translations: <ViewMut<'a, Translation<V, N>> as IntoBorrow>::Borrow::all_borrow(all_storages, last_run, current)?,
            rotations: <ViewMut<'a, Rotation<Q, N>> as IntoBorrow>::Borrow::all_borrow(all_storages, last_run, current)?,
            scales: <ViewMut<'a, Scale<V, N>> as IntoBorrow>::Borrow::all_borrow(all_storages, last_run, current)?,
            origins: <ViewMut<'a, Origin<V, N>> as IntoBorrow>::Borrow::all_borrow(all_storages, last_run, current)?,
            local_transforms: <ViewMut<'a, LocalTransform<M, N>> as IntoBorrow>::Borrow::all_borrow(all_storages, last_run, current)?,
            world_transforms: <ViewMut<'a, WorldTransform<M, N>> as IntoBorrow>::Borrow::all_borrow(all_storages, last_run, current)?,
            dirty_transforms: <ViewMut<'a, DirtyTransform> as IntoBorrow>::Borrow::all_borrow(all_storages, last_run, current)?,
        })
    }
}

// this impl lets you use it with `AllStorages::borrow`, `AllStorages::run`
impl<'a, V, Q, M, N> AllStoragesBorrow<'a> for LocalTransformStoragesMutBorrower<V, Q, M, N> 

where
    V: Vec3<N> + Send + Sync + 'static,
    Q: Quat<N> + Send + Sync + 'static,
    M: Matrix4<N> + Send + Sync + 'static,
    N: Copy + Send + Sync + 'static
{

    fn all_borrow(all_storages: &'a AllStorages, last_run: Option<u32>,current: u32,) -> Result<Self::View, error::GetStorage>
    where
        Self: Sized,
    {
        Ok(LocalTransformStoragesMut {
            translations: <ViewMut<'a, Translation<V, N>> as IntoBorrow>::Borrow::all_borrow(all_storages, last_run, current)?,
            rotations: <ViewMut<'a, Rotation<Q, N>> as IntoBorrow>::Borrow::all_borrow(all_storages, last_run, current)?,
            scales: <ViewMut<'a, Scale<V, N>> as IntoBorrow>::Borrow::all_borrow(all_storages, last_run, current)?,
            origins: <ViewMut<'a, Origin<V, N>> as IntoBorrow>::Borrow::all_borrow(all_storages, last_run, current)?,
            local_transforms: <ViewMut<'a, LocalTransform<M, N>> as IntoBorrow>::Borrow::all_borrow(all_storages, last_run, current)?,
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

// Explains to a workload which storage are borrowed by a system.
// Used to automate parallel running
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
