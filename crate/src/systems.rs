use shipyard::prelude::*;
use crate::transform::*;

#[system(TrsToLocal)]
pub fn run (
    translations: &Translation, 
    rotations: &Rotation, 
    scales: &Scale,
    mut local_transforms: &mut LocalTransform, 
) {
    (&translations, &rotations, &scales, &mut local_transforms)
        .iter()
        .for_each(|(translation, rotation, scale, local_transform)| {
            //local_transform.0.from_trs_mut(&translation.0, &rotation.0, &scale.0);
    });
}

#[system(LocalToWorld)]
pub fn run (
    local_transforms: &LocalTransform, 
    mut world_transforms: &mut WorldTransform, 
) {
    (&local_transforms, &mut world_transforms)
        .iter()
        .for_each(|(local, world)| {
            //TODO - account for hierarchy!!
            world.0.copy_from(&local.0);
    });
}