use crate::components::*;
use crate::traits::required::*;
use crate::views::SceneGraphStoragesMut;
use shipyard::*;
use shipyard_hierarchy::*;

//Marker for shipyard_hierarchy to know we're targeting _this_ hierarchy
pub struct SceneGraph {}

impl<'a, V, Q, M, N> SceneGraphStoragesMut<'a, V, Q, M, N>
where
    V: Vec3Ext<N> + Send + Sync + 'static,
    Q: QuatExt<N> + Send + Sync + 'static,
    M: Matrix4Ext<N> + Send + Sync + 'static,
    N: Copy + Send + Sync + 'static,
{
    pub fn spawn_child_identity(&mut self, parent: Option<EntityId>) -> EntityId {
        self.spawn_child_transform(parent, M::identity())
    }

    //TODO - derive these from the transform
    //until that's done, this function needs to be private
    fn spawn_child_transform(&mut self, parent: Option<EntityId>, local_matrix: M) -> EntityId {
        let translation = V::zero();
        let rotation = Q::identity();
        let scale = V::one();
        let origin = V::zero();
        self.spawn_child(parent, translation, rotation, scale, origin, local_matrix)
    }

    pub fn spawn_child_trs(
        &mut self,
        parent: Option<EntityId>,
        translation: Option<V>,
        rotation: Option<Q>,
        scale: Option<V>,
    ) -> EntityId {
        self.spawn_child_trs_origin(parent, translation, rotation, scale, None)
    }

    pub fn spawn_child_trs_origin(
        &mut self,
        parent: Option<EntityId>,
        translation: Option<V>,
        rotation: Option<Q>,
        scale: Option<V>,
        origin: Option<V>,
    ) -> EntityId {
        let translation = translation.unwrap_or_else(Vec3Ext::zero);
        let rotation = rotation.unwrap_or_else(QuatExt::identity);
        let scale = scale.unwrap_or_else(Vec3Ext::one);
        let origin = origin.unwrap_or_else(Vec3Ext::zero);
        let mut local_matrix = M::identity();
        local_matrix.reset_from_trs_origin(
            translation.as_slice(),
            rotation.as_slice(),
            scale.as_slice(),
            origin.as_slice(),
        );

        self.spawn_child(parent, translation, rotation, scale, origin, local_matrix)
    }
    fn spawn_child(
        &mut self,
        parent: Option<EntityId>,
        translation: V,
        rotation: Q,
        scale: V,
        origin: V,
        local_matrix: M,
    ) -> EntityId {
        let entity = self.entities.add_entity(
            (
                &mut self.translations,
                &mut self.rotations,
                &mut self.scales,
                &mut self.origins,
                &mut self.local_transforms,
                &mut self.world_transforms,
                &mut self.dirty_transforms,
            ),
            (
                Translation::new(translation),
                Rotation::new(rotation),
                Scale::new(scale),
                Origin::new(origin),
                LocalTransform::new(local_matrix),
                WorldTransform::new(M::identity()),
                DirtyTransform(true),
            ),
        );

        let parent = parent.unwrap_or(self.transform_root.0);

        (&mut self.entities, &mut self.parents, &mut self.children).attach(entity, parent);

        entity
    }
}
