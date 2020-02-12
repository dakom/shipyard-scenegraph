use shipyard::prelude::*;

pub fn init(world:&World) {
    crate::world::pack_storages(world);
}