use shipyard::*;
use shipyard_hierarchy::*;
use std::collections::HashSet;
use std::ops::{Mul, MulAssign};
use crate::components::*;
use crate::math::*;

pub fn trs_to_local(
    mut translations:ViewMut<Translation>,
    mut rotations:ViewMut<Rotation>,
    mut scales:ViewMut<Scale>,
    mut origins:ViewMut<Origin>,
    mut local_transforms:ViewMut<LocalTransform>,
    mut dirty_transforms:ViewMut<DirtyTransform>,
) {

    /*
        We only want to propogate changes if TRS is dirty
        That's why they are update packs! :D
    */
    let mut unique_ids = HashSet::<EntityId>::new();

    translations.inserted_or_modified().iter_ids().for_each(|id| { unique_ids.insert(id); });
    rotations.inserted_or_modified().iter_ids().for_each(|id| { unique_ids.insert(id); });
    scales.inserted_or_modified().iter_ids().for_each(|id| { unique_ids.insert(id); });
    origins.inserted_or_modified().iter_ids().for_each(|id| { unique_ids.insert(id); });

    unique_ids
        .iter()
        .for_each(|id| {
            let (translation, rotation, scale, origin, local_transform, dirty_transform) = (&translations, &rotations, &scales, &origins, &mut local_transforms, &mut dirty_transforms).try_get(*id).unwrap();
            local_transform.0.reset_from_trs_origin(&translation.0, &rotation.0, &scale.0, &origin.0);
            dirty_transform.0 = true;
        });

    translations.clear_inserted_and_modified();
    rotations.clear_inserted_and_modified();
    scales.clear_inserted_and_modified();
    origins.clear_inserted_and_modified();
}

//See: https://gameprogrammingpatterns.com/dirty-flag.html
pub fn local_to_world(
    root: UniqueView<TransformRoot>,
    parent_storage: View<Parent>,
    child_storage: View<Child>,
    local_transform_storage: View<LocalTransform>,
    mut dirty_transform_storage: ViewMut<DirtyTransform>,
    mut world_transform_storage: ViewMut<WorldTransform>,
) {
    fn update(id: EntityId, mut dirty: bool, parent: EntityId, parent_storage: &View<Parent>, child_storage: &View<Child>, local_transform_storage: &View<LocalTransform>, dirty_transform_storage: &mut ViewMut<DirtyTransform>, world_transform_storage: &mut ViewMut<WorldTransform>) {
        dirty |= dirty_transform_storage[id].0;
        dirty_transform_storage[id].0 = false;

        if dirty {
            //we need to operate on 2 parts of the storage at the same time
            //which effectively means taking 2 mutable refs (or a mutable and immutable)
            //this is technically unsafe but the system gets world_transform_storage as mut
            //so the scheduler will disallow another system from accessing it in parallel 
            //in order to avoid the UB we need to get each pointer _separately_ then call .mul_mut() with them
            //only the first part of the operation (copying world transform of parent to world of entity)
            //is actually unsafe - after that's done we can do a safe mul_assign against local_transform
            unsafe {
                let world_ptr = &mut world_transform_storage[id].0 as *mut Matrix4;
                let parent_world_ptr = &world_transform_storage[parent].0 as *const Matrix4;
                (&mut *world_ptr).copy_from_slice((&*parent_world_ptr).as_slice()); 
            }
            
            world_transform_storage[id].0 *= &local_transform_storage[id].0;


            //Safe version, but costs a clone of Mat4 each time
            /*
            let local = &local_transform_storage[id].0;
            let parent_world = &world_transform_storage[parent].0;
            let local_world = parent_world * local;
            world_transform_storage[id].0 = local_world;
            */
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
        world_transform_storage[root_id].0.copy_from_slice(local_transform_storage[root_id].0.as_slice());
    }

    //then recursively update all the children
    (&parent_storage, &child_storage).children(root_id).for_each(|child| {
        update(child, dirty, root_id, &parent_storage, &child_storage, &local_transform_storage, &mut dirty_transform_storage, &mut world_transform_storage);
    });
}

