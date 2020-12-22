use shipyard::*;
use shipyard_hierarchy::*;
use crate::views::SceneGraphStoragesMut;
use crate::components::*;
use crate::math::traits::*;

//Marker for shipyard_hierarchy to know we're targeting _this_ hierarchy
pub struct SceneGraph {}

impl <'a, V, Q, M, N> SceneGraphStoragesMut<'a, V, Q, M, N> 
where
    V: Vec3<N> + Send + Sync + 'static,
    Q: Quat<N> + Send + Sync + 'static,
    M: Matrix4<N> + Send + Sync + 'static,
    N: Copy + Send + Sync + 'static
{
    pub fn spawn_child(&mut self, parent: Option<EntityId>, translation: Option<V>, rotation: Option<Q>, scale: Option<V>, origin:Option<V>) -> EntityId {


        let translation = translation.unwrap_or_else(|| Vec3::zero());
        let rotation = rotation.unwrap_or_else(|| Quat::identity());
        let scale = scale.unwrap_or(Vec3::one());
        let origin = origin.unwrap_or_else(|| Vec3::zero());
        let local_matrix = Matrix4::identity(); //Matrix4::new_from_trs(&translation, &rotation, &scale);
        let world_matrix = Matrix4::identity();

        let Self { 
            entities, 
            transform_root,
            parents,
            children,
            translations,
            rotations,
            scales,
            origins,
            local_transforms,
            world_transforms,
            dirty_transforms,
        } = self;

        let entity = entities.add_entity( 
                (
                    translations,
                    rotations,
                    scales,
                    origins,
                    local_transforms,
                    world_transforms,
                    dirty_transforms
                ),
                (
                    Translation::new(translation),
                    Rotation::new(rotation),
                    Scale::new(scale),
                    Origin::new(origin),
                    LocalTransform::new(local_matrix),
                    WorldTransform::new(world_matrix),
                    DirtyTransform(false)
                )
        );

        {
            let parent = parent.unwrap_or(transform_root.0);

            (entities, parents, children).attach(entity, parent);
        }
        entity
    }

}