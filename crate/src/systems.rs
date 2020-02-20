use shipyard::prelude::*;
use std::collections::HashSet;
use crate::transform::*;
use crate::hierarchy::*;

#[system(TrsToLocal)]
pub fn run (
    root: Unique<&TransformRoot>,
    mut translations: &mut Translation, 
    mut rotations: &mut Rotation, 
    mut scales: &mut Scale,
    mut local_transforms: &mut LocalTransform, 
    mut dirty_transforms: &mut DirtyTransform, 
) {

    /*
        We only want to propogate changes if TRS is dirty
        That's why they are update packs! :D
    */
    let mut unique_ids = HashSet::<EntityId>::new();

    translations.inserted_or_modified().iter_ids().for_each(|id| { unique_ids.insert(id); });
    rotations.inserted_or_modified().iter_ids().for_each(|id| { unique_ids.insert(id); });
    scales.inserted_or_modified().iter_ids().for_each(|id| { unique_ids.insert(id); });

    unique_ids
        .iter()
        .for_each(|id| {
            let (translation, rotation, scale, local_transform, dirty_transform) = (&translations, &rotations, &scales, &mut local_transforms, &mut dirty_transforms).get(*id).unwrap();
            local_transform.0.reset_from_trs(&translation.0, &rotation.0, &scale.0);
            dirty_transform.0 = true;
        });

    translations.clear_inserted_and_modified();
    rotations.clear_inserted_and_modified();
    scales.clear_inserted_and_modified();
}

#[system(LocalToWorld)]
pub fn run (
    root: Unique<&TransformRoot>,
    parent_storage: &Parent, 
    child_storage: &Child, 
    local_transform_storage: &LocalTransform, 
    mut dirty_transform_storage: &mut DirtyTransform, 
    mut world_transform_storage: &mut WorldTransform, 
) {

    //we a copy since we can't have both immutable and mutable refs to the world transform storage
    //maintaining a cache and copying into it is faster than cloning
    let mut parent_matrix:Matrix4 = Matrix4::default(); 
    //tracking when we've moved to the next level
    let mut parent_id = root.0;
    //no need to update matrices multiple times per siblings or when not dirty
    let mut parent_matrix_updated = true;
    //descendents from this branch onwards are dirty
    let mut branch_dirty = false;

    for entity in (&parent_storage, &child_storage).descendants_breadth_first(root.0) {
        let child = (&child_storage).get(entity).unwrap();

        //Next level in the tree
        if parent_id != child.parent {
            parent_id = child.parent;
            //Signal that we'll need to update the cached parent matrix before using it 
            //But we may not need it so don't do it just yet
            parent_matrix_updated = false; 
        }

        let _dirty_transform = (&mut dirty_transform_storage).get(entity).unwrap();

        //set entire branch (descendents) to be dirty if this entity is dirty 
        /*
TODO: I think there are unneccessary updates... consider:
        A B
        /  \
        C    D
if A is dirty, but B is not 
then C should be dirty but D should not
However if the `branch_dirty` flag is set by A being dirty
then it will make D dirty, since it will be the next iteration.

B can be avoided by keeping a separate siblings_dirty flag
but that doesn't really solve the issue
        */
        branch_dirty |= _dirty_transform.0;

        if branch_dirty { 
            _dirty_transform.0 = false;
            if !parent_matrix_updated {
                parent_matrix.copy_from_slice(world_transform_storage.get(parent_id).unwrap().0.as_ref());
                parent_matrix_updated = true;
            }
            let local_transform = &local_transform_storage.get(entity).unwrap().0;
            let mut world_transform = (&mut world_transform_storage).get(entity).unwrap();
            world_transform.0.copy_from_slice(local_transform.as_ref());
            world_transform.0.mul_mut(&parent_matrix);
        } 
    }
}