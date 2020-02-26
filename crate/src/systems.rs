use shipyard::prelude::*;
use std::collections::HashSet;
use crate::transform::*;
use crate::hierarchy::*;

#[system(TrsToLocal)]
pub fn run (
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

//See: https://gameprogrammingpatterns.com/dirty-flag.html
#[system(LocalToWorld)]
pub fn run (
    root: Unique<&TransformRoot>,
    parent_storage: &Parent, 
    child_storage: &Child, 
    local_transform_storage: &LocalTransform, 
    mut dirty_transform_storage: &mut DirtyTransform, 
    mut world_transform_storage: &mut WorldTransform, 
) {

    //the idea is really: world_transform_storage[id].0 = &local_transform_storage[id].0 * &world_transform_storage[parent].0;
    //but this creates a new clone every time
    //by passing a scratch target around we can avoid the allocation, at the expense of an extra memcpy 
    let mut scratch_matrix:Matrix4 = Matrix4::default(); 

    fn update(id: EntityId, mut dirty: bool, parent: EntityId, scratch_matrix:&mut Matrix4, parent_storage: &View<Parent>, child_storage: &View<Child>, local_transform_storage: &View<LocalTransform>, dirty_transform_storage: &mut ViewMut<DirtyTransform>, world_transform_storage: &mut ViewMut<WorldTransform>) {
        dirty |= dirty_transform_storage[id].0;
        dirty_transform_storage[id].0 = false;

        if dirty {
            scratch_matrix.copy_from_slice(local_transform_storage[id].0.as_ref()); 
            scratch_matrix.mul_mut(&world_transform_storage[parent].0);
            world_transform_storage[id].0.copy_from_slice(scratch_matrix.as_ref());
        }

        (parent_storage, child_storage).children(id).for_each(|child| {
            update(child, dirty, id, scratch_matrix, parent_storage, child_storage, local_transform_storage, dirty_transform_storage, world_transform_storage);
        });
    }

    //first propogate the root transform if it changed
    let root_id = root.0;
    let dirty = dirty_transform_storage[root_id].0;
    dirty_transform_storage[root_id].0 = false;

    if dirty {
        world_transform_storage[root_id].0.copy_from_slice(local_transform_storage[root_id].0.as_ref());
    }

    //then recursively update all the children
    (&parent_storage, &child_storage).children(root_id).for_each(|child| {
        update(root_id, dirty, child, &mut scratch_matrix, &parent_storage, &child_storage, &local_transform_storage, &mut dirty_transform_storage, &mut world_transform_storage);
    });
}

