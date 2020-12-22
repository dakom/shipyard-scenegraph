#![allow(warnings)]
use shipyard::*;

use shipyard_scenegraph::math::native::*;
use shipyard_scenegraph::hierarchy::SceneGraph;
use shipyard_hierarchy::*;
use std::borrow::{Borrow, BorrowMut};
use std::collections::{HashMap, hash_map::Entry};
use std::hash::Hash;
mod helpers;
use helpers::*;

#[test]
fn test_transform_dirty_ids() {
    let (world, entities, _labels) = create_scene_graph();
    let (_root, _a,_b,_c,d,e,_f,_g,h,i,j,_k,l,m,n) = entities;

    world.run(trs_to_local);
    world.run(local_to_world);

    fn iters_equal_anyorder<T: Eq + Hash>(i1:impl Iterator<Item = T>, i2: impl Iterator<Item = T>) -> bool {
        fn get_lookup<T: Eq + Hash>(iter:impl Iterator<Item = T>) -> HashMap<T, usize> {
            let mut lookup = HashMap::<T, usize>::new();
            for value in iter {
                match lookup.entry(value) {
                    Entry::Occupied(entry) => { *entry.into_mut() += 1; },
                    Entry::Vacant(entry) => { entry.insert(0); }
                }
            }
            lookup
        }
        get_lookup(i1) == get_lookup(i2)
    }

    fn get_marked_dirty_ids(world:&World) -> Vec<EntityId> {
        let dirty_storage = world.borrow::<View<DirtyTransform>>().unwrap();

        (&dirty_storage)
            .iter()
            .with_id()
            .filter(|(_id, dirty)| dirty.0)
            .map(|(id, _)| id)
            .collect()
    }

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


    //nothing should be dirty until we run TrsToLocal
    {
        assert_eq!(get_marked_dirty_ids(&world).len(), 0);
    }

    world.run(trs_to_local);

    //and then only d, e and j should be the changed ones
    {
        assert!(iters_equal_anyorder(get_marked_dirty_ids(&world).iter(), [d,j,e].iter()));
    }


    //now check the tree of dirty ids due to traversal/propogation
    {
        let expected_dirty_traverse_ids = vec![d,e,h,i,l,j,m,n];
        let traverse_dirty_ids = get_traverse_dirty_ids(&world); //see comment below on get_traverse_dirty_ids()
        //println!("{} dirty", traverse_dirty_ids.iter().len());
        //traverse_dirty_ids.iter().for_each(|entity| println!("{:?}", labels.get(entity).unwrap()));
        assert!(iters_equal_anyorder(traverse_dirty_ids.iter(), expected_dirty_traverse_ids.iter()));
    }
}
//The actual system usage is LocalToWorld but that doesn't collect the updated ids into a Vec
//therefore, the logic is duplicated here - with the addition of pushing to dirty_list
//It's not ideal but not too terrible either 
fn get_traverse_dirty_ids(world:&World) -> Vec<EntityId> {
    let mut dirty_list = Vec::<EntityId>::new();
    let ( root, parent_storage, child_storage, local_transform_storage, mut dirty_transform_storage, mut world_transform_storage, ) = 
        world.borrow::<( UniqueView<TransformRoot>, View<Parent<SceneGraph>>, View<Child<SceneGraph>>, View<LocalTransform>, ViewMut<DirtyTransform>, ViewMut<WorldTransform>, )>().unwrap();

    fn update(dirty_list:&mut Vec<EntityId>, id: EntityId, mut dirty: bool, _parent: EntityId, parent_storage: &View<Parent<SceneGraph>>, child_storage: &View<Child<SceneGraph>>, local_transform_storage: &View<LocalTransform>, dirty_transform_storage: &mut ViewMut<DirtyTransform>, world_transform_storage: &mut ViewMut<WorldTransform>) {
        dirty |= dirty_transform_storage[id].0;
        dirty_transform_storage[id].0 = false;

        if dirty {
            dirty_list.push(id);
        }

        (parent_storage, child_storage).children(id).for_each(|child| {
            update(dirty_list, child, dirty, id, parent_storage, child_storage, local_transform_storage, dirty_transform_storage, world_transform_storage);
        });
    }

    //first propogate the root transform if it changed
    let root_id = root.0;
    let dirty = dirty_transform_storage[root_id].0;
    dirty_transform_storage[root_id].0 = false;

    //then recursively update all the children
    (&parent_storage, &child_storage).children(root_id).for_each(|child| {
        update(&mut dirty_list, root_id, dirty, child, &parent_storage, &child_storage, &local_transform_storage, &mut dirty_transform_storage, &mut world_transform_storage);
    });
    dirty_list
}