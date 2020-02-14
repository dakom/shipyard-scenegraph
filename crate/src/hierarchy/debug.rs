use std::fmt;
use std::marker::PhantomData;
use shipyard::prelude::*;
use super::*;

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
/*
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
*/
    }
}

/*
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