use shipyard::*;
use shipyard_hierarchy::*;
use std::collections::HashSet;
use crate::components::*;
use crate::math::traits::*;
use crate::hierarchy::SceneGraph;

pub fn trs_to_local<V, Q, M, N>(
    mut translations:ViewMut<Translation<V, N>>,
    mut rotations:ViewMut<Rotation<Q, N>>,
    mut scales:ViewMut<Scale<V, N>>,
    mut origins:ViewMut<Origin<V, N>>,
    mut local_transforms:ViewMut<LocalTransform<M, N>>,
    mut dirty_transforms:ViewMut<DirtyTransform>,
) 
where
    V: Vec3<N> + Send + Sync + 'static,
    Q: Quat<N> + Send + Sync + 'static,
    M: Matrix4<N> + Send + Sync + 'static,
    N: Copy + Send + Sync + 'static
{

    /*
        We only want to propogate changes if TRS is dirty
        That's why they are update packs! :D
    */
    let mut unique_ids = HashSet::<EntityId>::new();

    translations.inserted_or_modified().iter().ids().for_each(|id| { unique_ids.insert(id); });
    rotations.inserted_or_modified().iter().ids().for_each(|id| { unique_ids.insert(id); });
    scales.inserted_or_modified().iter().ids().for_each(|id| { unique_ids.insert(id); });
    origins.inserted_or_modified().iter().ids().for_each(|id| { unique_ids.insert(id); });

    unique_ids
        .iter()
        .for_each(|id| {
            let (translation, rotation, scale, origin, mut local_transform, mut dirty_transform) = 
                (&translations, &rotations, &scales, &origins, &mut local_transforms, &mut dirty_transforms).get(*id).unwrap();
            local_transform.reset_from_trs_origin(translation.as_slice(), rotation.as_slice(), scale.as_slice(), origin.as_slice());
            dirty_transform.0 = true;
        });

    translations.clear_inserted_and_modified();
    rotations.clear_inserted_and_modified();
    scales.clear_inserted_and_modified();
    origins.clear_inserted_and_modified();
}

//See: https://gameprogrammingpatterns.com/dirty-flag.html
pub fn local_to_world<M, N>(
    root: UniqueView<TransformRoot>,
    parent_storage: View<Parent<SceneGraph>>,
    child_storage: View<Child<SceneGraph>>,
    local_transform_storage: View<LocalTransform<M, N>>,
    mut dirty_transform_storage: ViewMut<DirtyTransform>,
    mut world_transform_storage: ViewMut<WorldTransform<M, N>>,
) 
where
    M: Matrix4<N> + Send + Sync,
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
        M: Matrix4<N> + Send + Sync,
        N: Copy + Send + Sync,
    
    {
        dirty |= dirty_transform_storage[id].0;
        dirty_transform_storage[id].0 = false;

        if dirty {
            //we need to operate on 2 parts of the storage at the same time
            //which effectively means taking 2 mutable refs (or a mutable and immutable)
            //this is technically unsafe but the system gets world_transform_storage as mut
            //so the scheduler will disallow another system from accessing it in parallel 
            //in order to avoid the UB we need to get each pointer _separately_ then multiply them
            //only the first part of the operation (copying world transform of parent to world of entity)
            //is actually unsafe - after that's done we can do a safe mul_assign against local_transform
            unsafe {
                let world_ptr = world_transform_storage[id].as_slice_mut().as_mut_ptr();
                let parent_world_ptr = world_transform_storage[parent].as_slice().as_ptr();
                
                //4x4 matrix is always 16 items
                //the std function will get the byte count internally
                std::ptr::copy_nonoverlapping(parent_world_ptr, world_ptr, 16)
            }
            
            world_transform_storage[id].mul_assign(&local_transform_storage[id]);

        }

        (parent_storage, child_storage).children(id).for_each(|child| {
            update(child, dirty, id, parent_storage, child_storage, local_transform_storage, dirty_transform_storage, world_transform_storage);
        });
    }

    //first propogate the root transform if it changed
    let root_id = root.0;
    let dirty = dirty_transform_storage[root_id].0;
    dirty_transform_storage[root_id].0 = false;

    if dirty {
        world_transform_storage[root_id].copy_from_slice(local_transform_storage[root_id].as_slice());
    }

    //then recursively update all the children
    (&parent_storage, &child_storage).children(root_id).for_each(|child| {
        update(child, dirty, root_id, &parent_storage, &child_storage, &local_transform_storage, &mut dirty_transform_storage, &mut world_transform_storage);
    });
}