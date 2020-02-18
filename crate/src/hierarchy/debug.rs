use std::collections::HashMap;
use shipyard::prelude::*;
use super::*;
/*
    The type signatures here are a bit intense. Some help from leudz:

    * P: IntoIter let us call iter() on P. This will give us <P as IntoIter>::IntoIter, whatever that is
    * We specify that this type needs to be iterable with Shiperator, which is like an iterator over the components.
    * Specifically making it Shiperator lets us call try_for_each()
    * Finally CurrentId will allow us to use with_id() and the id has to be EntityId (e.g. it won't work with chunk iterators but we don't care about those)

    Also, I'm not entirely sure if the debug printing is entirely correct. It's tested a bit but I'm not super confident
    Seems to work so far though!
*/

pub trait HierarchyIterDebug<'a, P, C> 
{
    fn debug_tree<F>(&'a self, root: EntityId, get_label:F) -> DebugHierarchyTree<'a, P, C, F>
        where F: Fn(EntityId) -> String;
}

impl<'a, P, C> HierarchyIterDebug<'a, P, C> for (P, C)
where
    P: GetComponent<Out = &'a Parent> + Copy + IntoIter,
    <P as IntoIter>::IntoIter: Shiperator + CurrentId<Id = EntityId>,
    C: GetComponent<Out = &'a Child> + Copy,
{
    fn debug_tree<F>(&'a self, root: EntityId, get_label:F) -> DebugHierarchyTree<'a, P, C, F> 
        where F: Fn(EntityId) -> String 
    {
        DebugHierarchyTree(self, root, get_label)
    }
}

pub struct DebugHierarchyTree<'a, P, C, F>(&'a (P, C), EntityId, F)
    where F: Fn(EntityId) -> String;

impl<'a, P, C, F> std::fmt::Debug for DebugHierarchyTree<'a, P, C, F>
where
    P: GetComponent<Out = &'a Parent> + Copy + IntoIter,
    <P as IntoIter>::IntoIter: Shiperator + CurrentId<Id = EntityId>,
    C: GetComponent<Out = &'a Child> + Copy,
    F: Fn(EntityId) -> String 
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let parent_storage = (self.0).0;
        let child_storage = (self.0).1;
        let root = self.1;
        let get_label = &(self.2);

        let mut depth_map = HashMap::<EntityId, usize>::new();
        let mut depth = 1;
        let mut last_parent = root;
        depth_map.insert(root, depth);

        fn get_spaces(entity_id:EntityId, depth_map:&HashMap<EntityId, usize>) -> String {
            let n = depth_map.get(&entity_id).unwrap();
            let mut s = "".to_string();
            for _ in 0..*n {
                s += "  ";
            }
            s
        };

        write!(f, "{}\n", get_label(root))?;
        for entity_id in (parent_storage, child_storage).descendants_depth_first(root) {
            let parent = child_storage.get(entity_id).unwrap().parent;
            if !depth_map.contains_key(&parent) {
                depth += 1;
                depth_map.insert(parent, depth);
            } else if parent != last_parent {
                depth -= 1;
            }
            last_parent = parent;
            write!(f, "{}{}\n", get_spaces(parent, &depth_map), get_label(entity_id))?;
        }

        Ok(())
    }
}