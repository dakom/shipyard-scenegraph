#![allow(dead_code, unused_imports)]
use shipyard::*;
use shipyard_scenegraph::prelude::*;
use std::borrow::{Borrow, BorrowMut};
use std::collections::{hash_map::Entry, HashMap};
use std::hash::Hash;
mod helpers;
use helpers::*;

const DIRTY_TRANSFORM_ANY: u8 = 0xFF;

#[test]
fn test_trs_dirty_ids() {
    let (world, entities, _labels) = create_scene_graph();
    let (_root, _a, _b, _c, d, e, _f, _g, h, i, j, _k, l, m, n) = entities;

    world.run(local_transform_sys);
    world.run(world_transform_sys);

    //first just get basic setup
    {
        assert_eq!(get_marked_dirty_ids(&world).len(), 0);
    }

    //change D, E and J
    {
        let mut translation_storage = world.borrow::<ViewMut<Translation>>().unwrap();
        let mut translation = (&mut translation_storage).get(d).unwrap();
        translation.set_y(200.0);
        let mut translation = (&mut translation_storage).get(e).unwrap();
        translation.set_y(300.0);
        let mut translation = (&mut translation_storage).get(j).unwrap();
        translation.set_y(400.0);
    }

    //nothing should be dirty until we run local_transform_sys
    {
        assert_eq!(get_marked_dirty_ids(&world).len(), 0);
    }

    world.run(local_transform_sys);

    //number of dirties should now be 3
    {
        assert_eq!(get_marked_dirty_ids(&world).len(), 3);
    }

    //and then only d, e and j should be the changed ones
    {
        assert!(iters_equal_anyorder(
            get_marked_dirty_ids(&world).iter(),
            [d, j, e].iter()
        ));
    }

    //now check the tree of dirty ids due to traversal/propogation
    {
        let expected_dirty_traverse_ids = vec![d, e, h, i, l, j, m, n];
        let traverse_dirty_ids = get_traverse_dirty_ids(&world); //see comment below on get_traverse_dirty_ids()
                                                                 //println!("{} dirty", traverse_dirty_ids.iter().len());
                                                                 //traverse_dirty_ids.iter().for_each(|entity| println!("{:?}", labels.get(entity).unwrap()));
        assert!(iters_equal_anyorder(
            traverse_dirty_ids.iter(),
            expected_dirty_traverse_ids.iter()
        ));
    }
}

#[test]
fn test_transform_dirty_ids() {
    let (world, entities, _labels) = create_scene_graph();
    let (_root, _a, _b, _c, d, e, _f, _g, h, i, j, _k, l, m, n) = entities;

    world.run(local_transform_sys);
    world.run(world_transform_sys);

    //first just get basic setup
    {
        assert_eq!(get_marked_dirty_ids(&world).len(), 0);
    }

    //change D, E and J
    {
        let mut transform_storage = world.borrow::<ViewMut<LocalTransform>>().unwrap();
        let mut transform = (&mut transform_storage).get(d).unwrap();
        transform[0] = 200.0;
        let mut transform = (&mut transform_storage).get(e).unwrap();
        transform[0] = 300.0;
        let mut transform = (&mut transform_storage).get(j).unwrap();
        transform[0] = 400.0;
    }

    //nothing should be dirty until we run local_transform_sys
    {
        assert_eq!(get_marked_dirty_ids(&world).len(), 0);
    }

    world.run(local_transform_sys);

    //number of dirties should now be 3
    {
        assert_eq!(get_marked_dirty_ids(&world).len(), 3);
    }

    //note to self - these were commented out... not sure why, they are passing o_O
    //and then only d, e and j should be the changed ones
    {
        assert!(iters_equal_anyorder(
            get_marked_dirty_ids(&world).iter(),
            [d, j, e].iter()
        ));
    }

    //now check the tree of dirty ids due to traversal/propogation
    {
        let expected_dirty_traverse_ids = vec![d, e, h, i, l, j, m, n];
        let traverse_dirty_ids = get_traverse_dirty_ids(&world); //see comment below on get_traverse_dirty_ids()
                                                                 //println!("{} dirty", traverse_dirty_ids.iter().len());
                                                                 //traverse_dirty_ids.iter().for_each(|entity| println!("{:?}", labels.get(entity).unwrap()));
        assert!(iters_equal_anyorder(
            traverse_dirty_ids.iter(),
            expected_dirty_traverse_ids.iter()
        ));
    }
}

fn iters_equal_anyorder<T: Eq + Hash>(
    i1: impl Iterator<Item = T>,
    i2: impl Iterator<Item = T>,
) -> bool {
    fn get_lookup<T: Eq + Hash>(iter: impl Iterator<Item = T>) -> HashMap<T, usize> {
        let mut lookup = HashMap::<T, usize>::new();
        for value in iter {
            match lookup.entry(value) {
                Entry::Occupied(entry) => {
                    *entry.into_mut() += 1;
                }
                Entry::Vacant(entry) => {
                    entry.insert(0);
                }
            }
        }
        lookup
    }
    get_lookup(i1) == get_lookup(i2)
}

fn get_marked_dirty_ids(world: &World) -> Vec<EntityId> {
    let dirty_storage = world.borrow::<View<DirtyTransform>>().unwrap();

    (&dirty_storage)
        .iter()
        .with_id()
        .filter(|(_id, dirty)| dirty.0)
        .map(|(id, _)| id)
        .collect()
}

//The actual system usage is LocalToWorld but that doesn't collect the updated ids into a Vec
//therefore, the logic is duplicated here - with the addition of pushing to dirty_list
//and without actually updating the transforms (i.e. it's purely to collect the ids)
//It's not ideal but not too terrible either
fn get_traverse_dirty_ids(world: &World) -> Vec<EntityId> {
    let mut dirty_list = Vec::<EntityId>::new();
    let (
        root,
        parent_storage,
        child_storage,
        local_transform_storage,
        mut dirty_transform_storage,
        mut world_transform_storage,
    ) = world
        .borrow::<(
            UniqueView<TransformRoot>,
            View<Parent<SceneGraph>>,
            View<Child<SceneGraph>>,
            View<LocalTransform>,
            ViewMut<DirtyTransform>,
            ViewMut<WorldTransform>,
        )>()
        .unwrap();

    fn update(
        dirty_list: &mut Vec<EntityId>,
        id: EntityId,
        mut dirty: bool,
        _parent: EntityId,
        parent_storage: &View<Parent<SceneGraph>>,
        child_storage: &View<Child<SceneGraph>>,
        local_transform_storage: &View<LocalTransform>,
        dirty_transform_storage: &mut ViewMut<DirtyTransform>,
        world_transform_storage: &mut ViewMut<WorldTransform>,
    ) {
        let dirty_transform = &mut dirty_transform_storage[id];
        let is_dirty: bool = dirty_transform.0;
        dirty_transform.0 = false;
        dirty |= is_dirty;

        if dirty {
            dirty_list.push(id);
        }

        (parent_storage, child_storage)
            .children(id)
            .for_each(|child| {
                update(
                    dirty_list,
                    child,
                    dirty,
                    id,
                    parent_storage,
                    child_storage,
                    local_transform_storage,
                    dirty_transform_storage,
                    world_transform_storage,
                );
            });
    }

    //first propogate the root transform if it changed
    let root_id = root.0;
    let dirty_transform = &mut dirty_transform_storage[root_id];
    let is_dirty: bool = dirty_transform.0;
    dirty_transform.0 = false;

    //then recursively update all the children
    (&parent_storage, &child_storage)
        .children(root_id)
        .for_each(|child| {
            update(
                &mut dirty_list,
                root_id,
                is_dirty,
                child,
                &parent_storage,
                &child_storage,
                &local_transform_storage,
                &mut dirty_transform_storage,
                &mut world_transform_storage,
            );
        });
    dirty_list
}
