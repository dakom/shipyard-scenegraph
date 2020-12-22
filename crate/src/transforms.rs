use shipyard::*;
use crate::views::TrsStoragesMut;
use crate::math::traits::*;

impl <'a, V, Q, N> TrsStoragesMut<'a, V, Q, N> 
where
    V: Vec3<N> + Send + Sync + 'static,
    Q: Quat<N> + Send + Sync + 'static,
    N: Copy + Send + Sync + 'static,
{
    pub fn set_trs_origin(&mut self, entity:EntityId, translation: Option<V>, rotation: Option<Q>, scale: Option<V>, origin: Option<V>) {
        let Self {translations, rotations, scales, origins, entities} = self;

        if let Some((t,r,s,o)) = (translations, rotations, scales, origins).get(entity).iter_mut().next() {
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
}