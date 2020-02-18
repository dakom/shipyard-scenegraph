use shipyard::prelude::*;

pub struct Parent {
    pub num_children: usize,
    pub first_child: EntityId,
}

pub struct Child {
    pub parent: EntityId,
    pub prev: EntityId,
    pub next: EntityId,
}