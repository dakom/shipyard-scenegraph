use super::{matrix4::*, vec3::*, quat::*};
use shipyard::*;
use shipyard_hierarchy::{Parent, Child};
use crate::hierarchy::SceneGraph;

//Alias and export the concrete types

//Components
pub type SceneGraphStoragesMut<'a> = crate::views::SceneGraphStoragesMut<'a, Vec3, Quat, Matrix4, f64>;
pub type Translation = crate::components::Translation<Vec3, f64>;
pub type Rotation = crate::components::Rotation<Quat, f64>;
pub type Scale = crate::components::Scale<Vec3, f64>;
pub type Origin = crate::components::Origin<Vec3, f64>;
pub type LocalTransform = crate::components::LocalTransform<Matrix4, f64>;
pub type WorldTransform = crate::components::WorldTransform<Matrix4, f64>;

//Systems
pub fn trs_to_local(
    mut translations:ViewMut<Translation>,
    mut rotations:ViewMut<Rotation>,
    mut scales:ViewMut<Scale>,
    mut origins:ViewMut<Origin>,
    mut local_transforms:ViewMut<LocalTransform>,
    mut dirty_transforms:ViewMut<DirtyTransform>,
) {
    crate::systems::trs_to_local(translations, rotations, scales, origins, local_transforms, dirty_transforms);
}

pub fn local_to_world (
    root: UniqueView<TransformRoot>,
    parents: View<Parent<SceneGraph>>,
    children: View<Child<SceneGraph>>,
    local_transforms: View<LocalTransform>,
    mut dirty_transforms: ViewMut<DirtyTransform>,
    mut world_transforms: ViewMut<WorldTransform>,
) {
    crate::systems::local_to_world(root, parents, children, local_transforms, dirty_transforms, world_transforms);
}

// Init
pub fn init_scenegraph(world:&World) -> EntityId {
    crate::init::init_scenegraph::<Vec3, Quat, Matrix4, f64>(&world)
}

//Re-export the other scenegraph components
pub use crate::components::{TransformRoot, DirtyTransform};
pub use crate::slice::*;