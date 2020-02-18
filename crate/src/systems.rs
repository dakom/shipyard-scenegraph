use shipyard::prelude::*;
use crate::transform::*;

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
    local_transforms: &LocalTransform, 
    mut world_transforms: &mut WorldTransform, 
) {
    //TODO - only if dirty
    //Account for hierarchy
    //So maybe we need a custom iterator
    (&local_transforms, &mut world_transforms)
        .iter()
        .for_each(|(local, world)| {
            world.0.copy_from(&local.0);
    });
}