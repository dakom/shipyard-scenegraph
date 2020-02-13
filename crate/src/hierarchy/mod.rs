//Mostly copy/paste from https://leudz.github.io/shipyard/book/recipes/hierarchy.html
mod iter;

use shipyard::prelude::*;

pub use iter::*;

pub struct Parent {
    pub num_children: usize,
    pub first_child: EntityId,
}

pub struct Child {
    pub parent: EntityId,
    pub prev: EntityId,
    pub next: EntityId,
}

pub trait Hierarchy {
    // Attaches an entity as a child to a given parent entity.
    fn attach(&mut self, id: EntityId, parent: EntityId);

    // Creates a new entity and attaches it to the given parent.
    fn attach_new(&mut self, parent: EntityId) -> EntityId;

    // Removes an entity from the hierarchy
    fn remove_single(&mut self, id: EntityId);

    // Removes an subtree from the hierarchy
    fn remove(&mut self, id: EntityId);
}

//the storages we'll impl Hierarchy on
type HierarchyStorages<'a> = (EntitiesViewMut<'a>, ViewMut<'a, Parent>, ViewMut<'a, Child>);

// detach an entity from the hierarchy.
// it's not on the trait since it's only for internal use
// the public api is remove/remove_single 
pub fn detach (hierarchy: &mut HierarchyStorages, id: EntityId) {
    let (_, parent_storage, child_storage) = hierarchy;

    // remove the Child component - if nonexistent, do nothing
    if let Some(child) = child_storage.remove(id) {
        // retrieve and update Parent component from ancestor
        let parent = &mut parent_storage[child.parent];
        parent.num_children -= 1;

        if parent.num_children == 0 {
            // if the number of children is zero, the Parent component must be removed
            parent_storage.remove(child.parent);
        } else {
            // the ancestor still has children, and we have to change some linking
            // check if we have to change first_child
            if parent.first_child == id {
                parent.first_child = child.next;
            }
            // remove the detached child from the sibling chain
            child_storage[child.prev].next = child.next;
            child_storage[child.next].prev = child.prev;
        }
    }
}

impl Hierarchy for HierarchyStorages<'_> {
    fn attach(&mut self, id: EntityId, parent: EntityId) {
        detach(self, id);

        let (entities, parent_storage, child_storage) = self;
        // the entity we want to attach might already be attached to another parent

        // either the designated parent already has a Parent component – and thus one or more children
        if let Ok(p) = parent_storage.get(parent) {
            // increase the parent's children counter
            p.num_children += 1;

            // get the ids of the new previous and next siblings of our new child
            let prev = child_storage[p.first_child].prev;
            let next = p.first_child;

            // change the linking
            child_storage[prev].next = id;
            child_storage[next].prev = id;

            // add the Child component to the new entity
            entities.add_component(child_storage, Child { parent, prev, next }, id);
        } else {
            // in this case our designated parent is missing a Parent component
            // we don't need to change any links, just insert both components
            entities.add_component(
                child_storage,
                Child {
                    parent,
                    prev: id,
                    next: id,
                },
                id,
            );
            entities.add_component(
                parent_storage,
                Parent {
                    num_children: 1,
                    first_child: id,
                },
                parent,
            );
        }
    }

    fn attach_new(&mut self, parent: EntityId) -> EntityId {
        let entities = &mut self.0;
        let id = entities.add_entity((), ());
        self.attach(id, parent);
        id
    }


    fn remove_single(&mut self, id: EntityId) {
        detach(self, id);

        let (_, parent_storage, child_storage) = &self;
        let children = (parent_storage, child_storage).children(id).collect::<Vec<_>>();
        for child_id in children {
            detach(self, child_id);
        }

        let parent_storage = &mut self.1;
        parent_storage.remove(id);
    }


    fn remove(&mut self, id: EntityId) {
        let (_, parent_storage, child_storage) = &self;
        for child_id in (parent_storage, child_storage).children(id).collect::<Vec<_>>() {
            self.remove(child_id);
        }
        self.remove_single(id);
    }
}