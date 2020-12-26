use shipyard::*;
use shipyard_hierarchy::*;
use std::collections::HashSet;
use crate::{components::*, views::LocalTransformStoragesMut};
use crate::traits::math::*;
use crate::hierarchy::SceneGraph;

pub fn local_transform_sys<V, Q, M, N>(
    mut local_trs_storages_mut: LocalTransformStoragesMut<V, Q, M, N>,
    mut dirty_transforms:ViewMut<DirtyTransform>,
) 
where
    V: Vec3<N> + Send + Sync + 'static,
    Q: Quat<N> + Send + Sync + 'static,
    M: Matrix4<N> + Send + Sync + 'static,
    N: Copy + Send + Sync + 'static
{
    //Inserted storages won't show in modified, gotta clear inserted first
    local_trs_storages_mut.clear_all_inserted();

    //First gather all the unique ids that have been tainted by trso changes
    let mut trso_ids: HashSet<EntityId> = local_trs_storages_mut.translations.modified().iter().ids().chain(
        local_trs_storages_mut.rotations.modified().iter().ids().chain(
            local_trs_storages_mut.scales.modified().iter().ids().chain(
                local_trs_storages_mut.origins.modified().iter().ids()
            )
        )
    ).collect();

    //Stash these in a full list which will also include the localtransform changes
    let mut all_ids = trso_ids.clone();

    //Adapt the list from LocalTransform, and set their TRSO values
    local_trs_storages_mut.local_transforms.modified()
        .iter()
        .ids()
        .for_each(|id| {
            //TODO - derive the TRSO values and set them on the components!

            //don't want to re-set the transform again :P
            trso_ids.remove(&id);

            //but do need to mark dirty
            all_ids.insert(id);
        });

    //For the remaining trso_ids, set the LocalTransforms
    trso_ids
        .iter()
        .for_each(|id| {
            let (translation, rotation, scale, origin, mut local_transform) = 
                (&mut local_trs_storages_mut).get(*id).unwrap();
            local_transform.reset_from_trs_origin(translation.as_slice(), rotation.as_slice(), scale.as_slice(), origin.as_slice());
        });

    //Mark everything that was dirty
    all_ids
        .iter()
        .for_each(|id| {
            let mut dirty_transform = (&mut dirty_transforms).get(*id).unwrap();
            dirty_transform.0 = true;
        });

    //Clear the update packs
    local_trs_storages_mut.clear_all_modified();
}
//See: https://gameprogrammingpatterns.com/dirty-flag.html
pub fn world_transform_sys<M, N>(
    root: UniqueView<TransformRoot>,
    parent_storage: View<Parent<SceneGraph>>,
    child_storage: View<Child<SceneGraph>>,
    local_transform_storage: View<LocalTransform<M, N>>,
    mut dirty_transform_storage: ViewMut<DirtyTransform>,
    mut world_transform_storage: ViewMut<WorldTransform<M, N>>,
) 
where
    M: Matrix4<N> + Clone + Send + Sync,
    N: Copy + Send + Sync + 'static
{
    fn update<M, N>(
        id: EntityId, 
        mut dirty: bool, 
        parent: EntityId, 
        parent_storage: &View<Parent<SceneGraph>>, 
        child_storage: &View<Child<SceneGraph>>, 
        local_transform_storage: &View<LocalTransform<M, N>>, 
        dirty_transform_storage: &mut ViewMut<DirtyTransform>, 
        world_transform_storage: &mut ViewMut<WorldTransform<M, N>>
    ) 
    where
        M: Matrix4<N> + Clone + Send + Sync,
        N: Copy + Send + Sync,
    
    {
        let dirty_transform = &mut dirty_transform_storage[id];
        let is_dirty:bool = dirty_transform.0;
        dirty_transform.0 = false;
        dirty |= is_dirty; 

        if dirty {
            world_transform_storage.apply_mut(id, parent, |world_transform, parent_transform| *world_transform = parent_transform.clone());
            world_transform_storage[id].mul_assign(&local_transform_storage[id]);
        }

        (parent_storage, child_storage).children(id).for_each(|child| {
            update(child, dirty, id, parent_storage, child_storage, local_transform_storage, dirty_transform_storage, world_transform_storage);
        });
    }

    //first propogate the root transform if it changed
    let root_id = root.0;

    let dirty_transform = &mut dirty_transform_storage[root_id];
    let is_dirty:bool = dirty_transform.0;
    dirty_transform.0 = false;
    if is_dirty {
        world_transform_storage[root_id].copy_from_slice(local_transform_storage[root_id].as_slice());
    }

    //then recursively update all the children
    (&parent_storage, &child_storage).children(root_id).for_each(|child| {
        update(child, is_dirty, root_id, &parent_storage, &child_storage, &local_transform_storage, &mut dirty_transform_storage, &mut world_transform_storage);
    });
}