use std::fmt;
use std::marker::PhantomData;
use std::collections::HashMap;
use shipyard::prelude::*;
use super::*;
/*
    The type signatures here are a bit intense. Some help from leudz:

    P: IntoIter let us call iter() on P. This will give us <P as IntoIter>::IntoIter, whatever that is
    We specify that this type needs to be iterable with Shiperator, which is like an iterator over the components.
    Specifically making it Shiperator lets us call try_for_each()
    Finally CurrentId will allow us to use with_id() and the id has to be EntityId (e.g. it won't work with chunk iterators but we don't care about those)
    */

pub trait HierarchyIterDebug<'a, P, C> 
{
    fn debug(&'a self) -> DebugHierarchy<'a, P, C>;
    fn debug_tree(&'a self, root:EntityId) -> DebugHierarchyTree<'a, P, C>;
}

impl<'a, P, C> HierarchyIterDebug<'a, P, C> for (P, C)
where
    P: GetComponent<Out = &'a Parent> + Copy + IntoIter,
    <P as IntoIter>::IntoIter: Shiperator + CurrentId<Id = EntityId>,
    C: GetComponent<Out = &'a Child> + Copy,
{
    fn debug(&'a self) -> DebugHierarchy<'a, P, C> {
        DebugHierarchy(self)
    }
    fn debug_tree(&'a self, root: EntityId) -> DebugHierarchyTree<'a, P, C> {
        DebugHierarchyTree(self, root)
    }
}

pub struct DebugHierarchy<'a, P, C>(&'a (P, C));

impl<'a, P, C> std::fmt::Debug for DebugHierarchy<'a, P, C>
where
    P: GetComponent<Out = &'a Parent> + Copy + IntoIter,
    <P as IntoIter>::IntoIter: Shiperator + CurrentId<Id = EntityId>,
    C: GetComponent<Out = &'a Child> + Copy,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        (self.0).0.iter().with_id().try_for_each(|(parent, _)| {
            f.write_fmt(format_args!(
                "{:?}'s children: {:?}",
                parent,
                self.0.children(parent).collect::<Vec<_>>()
            ))
        })
    }
}

pub struct DebugHierarchyTree<'a, P, C>(&'a (P, C), EntityId);

impl<'a, P, C> std::fmt::Debug for DebugHierarchyTree<'a, P, C>
where
    P: GetComponent<Out = &'a Parent> + Copy + IntoIter,
    <P as IntoIter>::IntoIter: Shiperator + CurrentId<Id = EntityId>,
    C: GetComponent<Out = &'a Child> + Copy,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let parent_storage = (self.0).0;
        let child_storage = (self.0).1;
        let root = self.1;

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

        write!(f, "{:?}\n", root)?;
        for entity_id in (parent_storage, child_storage).descendants_depth_first(root) {
            let parent = child_storage.get(entity_id).unwrap().parent;
            if !depth_map.contains_key(&parent) {
                depth += 1;
                depth_map.insert(parent, depth);
            } else if parent != last_parent {
                depth -= 1;
            }
            last_parent = parent;
            write!(f, "{}{:?}\n", get_spaces(parent, &depth_map), entity_id)?;
        }

        Ok(())
    }
}
/*
pub trait HierarchyIterDebug<'a, P, C, T> {
    fn debug(&self, id: EntityId) -> HierarchyDebug<P, C, T>;
}

impl<'a, P, C> HierarchyIterDebug<'a, P, C, Self> for (P, C)
where
    P: GetComponent<Out = &'a Parent> + Copy,
    C: GetComponent<Out = &'a Child> + Copy,
{
    fn debug(&self, id: EntityId) -> HierarchyDebug<P,C,Self> {
    }
}

pub struct HierarchyDebug<'a, P, C, T> 
where
    P: GetComponent<Out = &'a Parent> + Copy,
    C: GetComponent<Out = &'a Child> + Copy,
    T: HierarchyIter<'a, P, C>
{
    hierarchy: T,
    root: EntityId,
    phantom: PhantomData<(P,C)>
}

impl <'a, P, C, T> HierarchyDebug<'a, P, C, T> 
where
    P: GetComponent<Out = &'a Parent> + Copy,
    C: GetComponent<Out = &'a Child> + Copy,
    T: HierarchyIter<'a, P, C>
{
    pub fn new(hierarchy: T, root: EntityId) -> Self {
        Self { 
            hierarchy, 
            root, 
            phantom: PhantomData
        }
    }
}
impl <'a, P, C, T> fmt::Debug for HierarchyDebug<'a, P, C, T>
where
    P: GetComponent<Out = &'a Parent> + Copy,
    C: GetComponent<Out = &'a Child> + Copy,
    T: HierarchyIter<'a, P, C>
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut n_spaces = "".to_string();
        for (parent, children) in self.hierarchy.children_by_level(self.root) {
            let n_spaces_next = n_spaces.to_string() + "  ";
            write!(f, "(P) {}{:?}\n", &n_spaces, parent)?;
            for child in children {
                write!(f, "(C) {}{:?}\n", n_spaces_next, child)?;
            }
            n_spaces = n_spaces_next;
        }
        Ok(())
        fn write_level<'f,'b,'a, P,C,T>(env:LevelEnv<'f, 'b, 'a, P,C,T>) -> fmt::Result 
        where
            P: GetComponent<Out = &'a Parent> + Copy,
            C: GetComponent<Out = &'a Child> + Copy,
            T: HierarchyIter<'a, P, C>
        {
            let LevelEnv {hierarchy, f, root, n_spaces, ..} = env;
            write!(f, "{:?}\n", root)?;
            for child in hierarchy.children(root) {
                write!(f, "{}{:?}\n", n_spaces, child)?;
                if(hierarchy.children(child).next().is_some()) {
                    //write_level(child)?;
                }
            }
            n_spaces += "  ";

            Ok(())
        };
        write_level(env)
    }
}

pub struct HierarchyDebugEnv<'a, P, C, T> 
where
    P: GetComponent<Out = &'a Parent> + Copy,
    C: GetComponent<Out = &'a Child> + Copy,
    T: HierarchyIter<'a, P, C>
{
    hierarchy: T,
    root: EntityId,
    n_spaces: String,
    phantom: PhantomData<(P,C)>
}

impl <'a, P, C, T> HierarchyDebugEnv<'a, P, C, T> 
where
    P: GetComponent<Out = &'a Parent> + Copy,
    C: GetComponent<Out = &'a Child> + Copy,
    T: HierarchyIter<'a, P, C>
{
    pub fn new(hierarchy: T, root: EntityId) -> Self {
        Self { 
            hierarchy, 
            root, 
            n_spaces: "  ".to_string(),
            phantom: PhantomData
        }
    }
}

fn write_level<'a, P, C, T>(state: &mut HierarchyDebug<'a, P, C, T>, f: &mut fmt::Formatter<'_>) -> std::fmt::Result
where
    P: GetComponent<Out = &'a Parent> + Copy,
    C: GetComponent<Out = &'a Child> + Copy,
    T: HierarchyIter<'a, P, C>
{
    let HierarchyDebug {hierarchy, root, n_spaces, ..} = state;
    for child in hierarchy.children(*root) {
        write!(f, "{}{:?}\n", n_spaces, child)?;
        if(hierarchy.children(child).next().is_some()) {
            state.root = child.clone();
            write_level(&mut state, f)?;
        }
    }

    Ok(())
}*/