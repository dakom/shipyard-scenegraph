use shipyard::prelude::*;
use crate::transform::*;
use crate::hierarchy::*;

#[system(TrsToLocal)]
pub fn run (
    translations: &Translation, 
    rotations: &Rotation, 
    scales: &Scale,
    mut local_transforms: &mut LocalTransform, 
) {
    //TODO - only if dirty
    (&translations, &rotations, &scales, &mut local_transforms)
        .iter()
        .for_each(|(translation, rotation, scale, local_transform)| {
            local_transform.0.reset_from_trs(&translation.0, &rotation.0, &scale.0);
    });
}

#[system(LocalToWorld)]
pub fn run (
    root: Unique<&TransformRoot>,
    parent_storage: &Parent, 
    child_storage: &Child, 
    local_transform_storage: &LocalTransform, 
    mut world_transform_storage: &mut WorldTransform, 
) {
    let mut parent_matrix:Matrix4 = Matrix4::default(); 
    let mut last_parent = root.0;
    let mut last_parent_matrix_updated = true;

    for entity in (&parent_storage, &child_storage).descendants_breadth_first(root.0) {
        let child = (&child_storage).get(entity).unwrap();
        if last_parent != child.parent {
            last_parent = child.parent;
            last_parent_matrix_updated = false; 
        }

        //TODO - only if dirty
        if !last_parent_matrix_updated {
            parent_matrix.copy_from_slice(world_transform_storage.get(last_parent).unwrap().0.as_ref());
            last_parent_matrix_updated = true;
        }
        let local_transform = &local_transform_storage.get(entity).unwrap().0;
        let mut world_transform = (&mut world_transform_storage).get(entity).unwrap();
        world_transform.0.copy_from_slice(local_transform.as_ref());
        world_transform.0.mul_mut(&parent_matrix);
    }
}