use shipyard::*;
use shipyard_hierarchy::*;

use super::*;
/*
    these need access to the whole hierarchy
*/

//a marker for the hierarchy
pub struct SceneGraph {}

pub trait TransformHierarchyMut {
    fn spawn_child(&mut self, parent: Option<EntityId>, translation: Option<Vec3>, rotation: Option<Quat>, scale: Option<Vec3>, origin: Option<Vec3>) -> EntityId;
}

pub type TransformHierarchyStoragesMut<'a, 'b> = (
    &'b mut EntitiesViewMut<'a>, 
    &'b UniqueView<'a, TransformRoot>, 
    &'b mut ViewMut<'a, Parent<SceneGraph>>, 
    &'b mut ViewMut<'a, Child<SceneGraph>>,
    &'b mut ViewMut<'a, Translation>,
    &'b mut ViewMut<'a, Rotation>,
    &'b mut ViewMut<'a, Scale>,
    &'b mut ViewMut<'a, Origin>,
    &'b mut ViewMut<'a, LocalTransform>,
    &'b mut ViewMut<'a, WorldTransform>,
    &'b mut ViewMut<'a, DirtyTransform>,
);

impl TransformHierarchyMut for TransformHierarchyStoragesMut<'_, '_> {
    fn spawn_child(&mut self, parent: Option<EntityId>, translation: Option<Vec3>, rotation: Option<Quat>, scale: Option<Vec3>, origin:Option<Vec3>) -> EntityId {

        let (
            entities, 
            root,
            parents,
            childs,
            translations,
            rotations,
            scales,
            origins,
            local_transforms,
            world_transforms,
            dirty_transforms
        ) = self;

        let translation = translation.unwrap_or_else(|| Vec3::zero());
        let rotation = rotation.unwrap_or_else(|| Quat::identity());
        let scale = scale.unwrap_or(Vec3::new(1.0, 1.0, 1.0));
        let origin = origin.unwrap_or_else(|| Vec3::zero());
        let local_matrix = Matrix4::identity(); //Matrix4::new_from_trs(&translation, &rotation, &scale);
        let world_matrix = Matrix4::identity();

        let entity = entities.add_entity( 
                (
                    &mut **translations,
                    &mut **rotations,
                    &mut **scales,
                    &mut **origins,
                    &mut **local_transforms,
                    &mut **world_transforms,
                    &mut **dirty_transforms
                ),
                (
                    Translation(translation),
                    Rotation(rotation),
                    Scale(scale),
                    Origin(origin),
                    LocalTransform(local_matrix),
                    WorldTransform(world_matrix),
                    DirtyTransform(false)
                )
        );

        {
            let parent = parent.unwrap_or(root.0);

            (&mut **entities, &mut **parents, &mut **childs).attach(entity, parent);
        }
        entity
    }

}


// these methods don't need access to the hierarchy
pub trait TransformMut {
    fn set_trs_origin(&mut self, entity:EntityId, translation: Option<Vec3>, rotation: Option<Quat>, scale: Option<Vec3>, origin: Option<Vec3>);
}

pub type TransformStoragesMut<'a, 'b> = (
    &'b mut ViewMut<'a, Translation>,
    &'b mut ViewMut<'a, Rotation>,
    &'b mut ViewMut<'a, Scale>,
    &'b mut ViewMut<'a, Origin>,
);

impl TransformMut for TransformStoragesMut<'_, '_> {
    fn set_trs_origin(&mut self, entity:EntityId, translation: Option<Vec3>, rotation: Option<Quat>, scale: Option<Vec3>, origin: Option<Vec3>) {
        let ( translations, rotations, scales, origins) = self;

        if let Some((t,r,s,o)) = (&mut **translations, &mut **rotations, &mut **scales, &mut **origins).try_get(entity).iter_mut().next() {
            if let Some(translation) = translation {
                t.0.copy_from(&translation);
            }
            if let Some(rotation) = rotation {
                r.0.copy_from(&rotation);
            }
            if let Some(scale) = scale {
                s.0.copy_from(&scale);
            }
        }
    }
}
