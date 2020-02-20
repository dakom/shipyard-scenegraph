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
) {
    let mut unique_ids = HashSet::<EntityId>::new();

    translations
        .inserted_or_modified()
        .iter()
        .with_id()
        .map(|(id, _)| id)
        .into_iter()
        .zip(
            rotations 
                .inserted_or_modified()
                .iter()
                .with_id()
                .map(|(id, _)| id)
                .into_iter()
        )
        .zip(
            scales 
                .inserted_or_modified()
                .iter()
                .with_id()
                .map(|(id, _)| id)
                .into_iter()
        )
        .for_each(|((t, r), s)| {
            unique_ids.insert(t);
            unique_ids.insert(r);
            unique_ids.insert(s);
        });

    unique_ids
        .iter()
        .for_each(|id| {
            let (translation, rotation, scale, local_transform) = (&translations, &rotations, &scales, &mut local_transforms).get(*id).unwrap();
            local_transform.0.reset_from_trs(&translation.0, &rotation.0, &scale.0);
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