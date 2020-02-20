use shipyard::prelude::*;
use std::collections::VecDeque;
use super::*;

pub struct ChildrenIter<C> {
    pub child_storage: C,
    pub cursor: (EntityId, usize),
}

impl<'a, C> Iterator for ChildrenIter<C>
where
    C: GetComponent<Out = &'a Child> + Copy,
{
    type Item = EntityId;

    fn next(&mut self) -> Option<Self::Item> {
        let (entity, num_children) = &mut self.cursor;
        if *num_children > 0 {
            *num_children -= 1;
            let ret = *entity;
            self.cursor.0 = self.child_storage.get(ret).unwrap().next;
            Some(ret)
        } else {
            None
        }
    }
}

pub struct AncestorIter<C> {
    pub child_storage: C,
    pub cursor: EntityId,
}

impl<'a, C> Iterator for AncestorIter<C>
where
    C: GetComponent<Out = &'a Child> + Copy,
{
    type Item = EntityId;

    fn next(&mut self) -> Option<Self::Item> {
        self.child_storage.get(self.cursor).ok().map(|child| {
            self.cursor = child.parent;
            child.parent
        })
    }
}

pub struct DescendantsDepthFirstIter<P, C> {
    pub parent_storage: P,
    pub child_storage: C,
    pub cursors: Vec<(EntityId, usize)>,
}

impl<'a, P, C> Iterator for DescendantsDepthFirstIter<P, C>
where
    P: GetComponent<Out = &'a Parent> + Copy,
    C: GetComponent<Out = &'a Child> + Copy,
{
    type Item = EntityId;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(cursor) = self.cursors.last_mut() {
            let (entity, num_children) = cursor;
            if *num_children > 0 {
                *num_children -= 1;

                let ret = *entity;

                *entity = self.child_storage.get(ret).unwrap().next;

                if let Ok(parent) = self.parent_storage.get(ret) {
                    self.cursors.push((parent.first_child, parent.num_children));
                }
                Some(ret)
            } else {
                self.cursors.pop();
                self.next()
            }
        } else {
            None
        }
    }
}

pub struct DescendantsBreadthFirstIter<P, C> {
    pub parent_storage: P,
    pub child_storage: C,
    pub cursors: VecDeque<(EntityId, usize)>,
}

impl<'a, P, C> Iterator for DescendantsBreadthFirstIter<P, C>
where
    P: GetComponent<Out = &'a Parent> + Copy,
    C: GetComponent<Out = &'a Child> + Copy,
{
    type Item = EntityId;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(cursor) = self.cursors.front_mut() {
            let (entity, num_children) = cursor;
            if *num_children > 0 {
                *num_children -= 1;

                let ret = *entity;

                *entity = self.child_storage.get(ret).unwrap().next;

                if let Ok(parent) = self.parent_storage.get(ret) {
                    self.cursors.push_back((parent.first_child, parent.num_children));
                }
                Some(ret)
            } else {
                self.cursors.pop_front();
                self.next()
            }
        } else {
            None
        }
    }
}

pub trait HierarchyIter<'a, P, C> 
{
    fn ancestors(&self, id: EntityId) -> AncestorIter<C>;
    fn children(&self, id: EntityId) -> ChildrenIter<C>;
    fn descendants_depth_first(&self, id: EntityId) -> DescendantsDepthFirstIter<P, C>;
    fn descendants_breadth_first(&self, id: EntityId) -> DescendantsBreadthFirstIter<P, C>;
}

impl<'a, P, C> HierarchyIter<'a, P, C> for (P, C)
where
    P: GetComponent<Out = &'a Parent> + Copy,
    C: GetComponent<Out = &'a Child> + Copy,
    //<P as IntoIter>::IntoIter: Shiperator + CurrentId<Id = EntityId>,
    //<C as IntoIter>::IntoIter: Shiperator + CurrentId<Id = EntityId>,
{
    fn ancestors(&self, id: EntityId) -> AncestorIter<C> {
        let (_, child_storage) = *self;
        AncestorIter {
            child_storage,
            cursor: id,
        }
    }

    fn children(&self, id: EntityId) -> ChildrenIter<C> {
        let (parent_storage, child_storage) = *self;
        ChildrenIter {
            child_storage,
            cursor: parent_storage
                .get(id)
                .map_or((id, 0), |parent| (parent.first_child, parent.num_children)),
        }
    }

    fn descendants_depth_first(&self, id: EntityId) -> DescendantsDepthFirstIter<P, C> {
        let (parent_storage, child_storage) = *self;
        DescendantsDepthFirstIter {
            parent_storage,
            child_storage,
            cursors: parent_storage.get(id).map_or_else(|_| Vec::new(), |parent| {
                vec![(parent.first_child, parent.num_children)]
            }),
        }
    }
    fn descendants_breadth_first(&self, id: EntityId) -> DescendantsBreadthFirstIter<P, C> {
        let (parent_storage, child_storage) = *self;
        DescendantsBreadthFirstIter {
            parent_storage,
            child_storage,
            cursors: parent_storage
                .get(id)
                .map_or_else(|_| VecDeque::new(), |parent| {
                    let mut queue = VecDeque::new();
                    queue.push_front((parent.first_child, parent.num_children));
                    queue
                }),
        }
    }
}
