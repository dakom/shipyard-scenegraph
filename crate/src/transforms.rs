use shipyard::*;
use crate::views::LocalTransformStoragesMut;
use crate::traits::math::*;

impl <'a, V, Q, M, N> LocalTransformStoragesMut<'a, V, Q, M, N> 
where
    V: Vec3<N> + Send + Sync + 'static,
    Q: Quat<N> + Send + Sync + 'static,
    M: Matrix4<N> + Send + Sync + 'static,
    N: Copy + Send + Sync + 'static,
{
    pub fn set_local_transform(&mut self, entity:EntityId, transform:M) {
        let Self {local_transforms, ..} = self;

        if let Some(local_transform) = (local_transforms).get(entity).iter_mut().next() {
            local_transform.copy_from(&transform);
        }
    }
    pub fn set_trs(&mut self, entity:EntityId, translation: Option<&V>, rotation: Option<&Q>, scale: Option<&V>) {
        let Self {translations, rotations, scales, ..} = self;

        if let Some((t,r,s)) = (translations, rotations, scales).get(entity).iter_mut().next() {
            if let Some(translation) = translation {
                t.copy_from(&translation);
            }
            if let Some(rotation) = rotation {
                r.copy_from(&rotation);
            }
            if let Some(scale) = scale {
                s.copy_from(&scale);
            }
        }
    }

    pub fn set_trs_origin(&mut self, entity:EntityId, translation: Option<&V>, rotation: Option<&Q>, scale: Option<&V>, origin: Option<&V>) {
        let Self {translations, rotations, scales, origins, ..} = self;

        if let Some((t,r,s,o)) = (translations, rotations, scales, origins).get(entity).iter_mut().next() {
            if let Some(translation) = translation {
                t.copy_from(translation);
            }
            if let Some(rotation) = rotation {
                r.copy_from(rotation);
            }
            if let Some(scale) = scale {
                s.copy_from(scale);
            }
            if let Some(origin) = origin {
                o.copy_from(origin);
            }
        }
    }

    /*
    pub(crate) fn clear_inserted(&mut self, id: EntityId) {
        self.translations.clear_inserted(id);
        self.rotations.clear_inserted(id);
        self.scales.clear_inserted(id);
        self.origins.clear_inserted(id);
        self.local_transforms.clear_inserted(id);
    }

    pub(crate) fn clear_modified(&mut self, id: EntityId) {
        self.translations.clear_modified(id);
        self.rotations.clear_modified(id);
        self.scales.clear_modified(id);
        self.origins.clear_modified(id);
        self.local_transforms.clear_modified(id);
    }
    */

    pub(crate) fn clear_all_inserted(&mut self) {
        self.translations.clear_all_inserted();
        self.rotations.clear_all_inserted();
        self.scales.clear_all_inserted();
        self.origins.clear_all_inserted();
        self.local_transforms.clear_all_inserted();
    }

    pub(crate) fn clear_all_modified(&mut self) {
        self.translations.clear_all_modified();
        self.rotations.clear_all_modified();
        self.scales.clear_all_modified();
        self.origins.clear_all_modified();
        self.local_transforms.clear_all_modified();
    }
}