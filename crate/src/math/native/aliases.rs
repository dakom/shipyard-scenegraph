use super::{matrix4::*, vec3::*, vec4::*};
use crate::components::{DirtyTransform, TransformRoot};
use crate::hierarchy::SceneGraph;
use crate::views::LocalTransformStoragesMut;
use shipyard::*;
use shipyard_hierarchy::*;

//Alias and export the concrete types

//Components
pub type SceneGraphStoragesMut<'a> =
    crate::views::SceneGraphStoragesMut<'a, Vec3, Vec4, Matrix4, f64>;
pub type Translation = crate::components::Translation<Vec3, f64>;
pub type Rotation = crate::components::Rotation<Vec4, f64>;
pub type Scale = crate::components::Scale<Vec3, f64>;
pub type Origin = crate::components::Origin<Vec3, f64>;
pub type LocalTransform = crate::components::LocalTransform<Matrix4, f64>;
pub type WorldTransform = crate::components::WorldTransform<Matrix4, f64>;

//Systems
pub fn local_transform_sys(
    trs_storages_mut: LocalTransformStoragesMut<Vec3, Vec4, Matrix4, f64>,
    dirty_transforms: ViewMut<DirtyTransform>,
) {
    crate::systems::local_transform_sys(trs_storages_mut, dirty_transforms);
}

pub fn world_transform_sys(
    root: UniqueView<TransformRoot>,
    parents: View<Parent<SceneGraph>>,
    children: View<Child<SceneGraph>>,
    local_transforms: View<LocalTransform>,
    dirty_transforms: ViewMut<DirtyTransform>,
    world_transforms: ViewMut<WorldTransform>,
) {
    crate::systems::world_transform_sys(
        root,
        parents,
        children,
        local_transforms,
        dirty_transforms,
        world_transforms,
    );
}

// Init
pub fn init_scenegraph(world: &World) -> EntityId {
    crate::init::init_scenegraph::<Vec3, Vec4, Matrix4, f64>(world)
}
