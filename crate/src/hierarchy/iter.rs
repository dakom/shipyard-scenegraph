use shipyard::prelude::*;
use super::*;

pub struct ChildrenIter<C> {
    pub get_child: C,
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
            self.cursor.0 = self.get_child.get(ret).unwrap().next;
            Some(ret)
        } else {
            None
        }
    }
}

pub struct AncestorIter<C> {
    pub get_child: C,
    pub cursor: EntityId,
}

impl<'a, C> Iterator for AncestorIter<C>
where
    C: GetComponent<Out = &'a Child> + Copy,
{
    type Item = EntityId;

    fn next(&mut self) -> Option<Self::Item> {
        self.get_child.get(self.cursor).ok().map(|child| {
            self.cursor = child.parent;
            child.parent
        })
    }
}

pub struct DescendantsDepthFirstIter<P, C> {
    pub get_parent: P,
    pub get_child: C,
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

                *entity = self.get_child.get(ret).unwrap().next;

                if let Ok(parent) = self.get_parent.get(ret) {
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

pub trait HierarchyIter<'a, P, C> {
    fn ancestors(&self, id: EntityId) -> AncestorIter<C>;
    fn children(&self, id: EntityId) -> ChildrenIter<C>;
    fn descendants_depth_first(&self, id: EntityId) -> DescendantsDepthFirstIter<P, C>;
}

impl<'a, P, C> HierarchyIter<'a, P, C> for (P, C)
where
    P: GetComponent<Out = &'a Parent> + Copy,
    C: GetComponent<Out = &'a Child> + Copy,
{
    fn ancestors(&self, id: EntityId) -> AncestorIter<C> {
        let (_, get_child) = *self;
        AncestorIter {
            get_child,
            cursor: id,
        }
    }

    fn children(&self, id: EntityId) -> ChildrenIter<C> {
        let (get_parent, get_child) = *self;
        ChildrenIter {
            get_child,
            cursor: get_parent
                .get(id)
                .map_or((id, 0), |parent| (parent.first_child, parent.num_children)),
        }
    }

    fn descendants_depth_first(&self, id: EntityId) -> DescendantsDepthFirstIter<P, C> {
        let (get_parent, get_child) = *self;
        DescendantsDepthFirstIter {
            get_parent,
            get_child,
            cursors: get_parent.get(id).map_or_else(|_| Vec::new(), |parent| {
                vec![(parent.first_child, parent.num_children)]
            }),
        }
    }
}