use shipyard::prelude::*;
use shipyard_hierarchy::*;
use shipyard_scenegraph::{self as sg, *};
use std::collections::{HashMap, hash_map::Entry};
use std::hash::Hash;

#[test]
fn test_transform_straight() {
    let (world, entities, _labels) = create_scene_graph();
    let (root, a,b,c,d,e,f,g,h,i,j,k,l,m,n) = entities;

    //adding the entities makes trs dirty, but not yet world
    {
        let (translations, rotations, scales, dirty) = world.borrow::<(&Translation, &Rotation, &Scale, &DirtyTransform)>(); 

        let tlen = translations.inserted_or_modified().len();
        let rlen = rotations.inserted_or_modified().len();
        let slen = scales.inserted_or_modified().len();
        let dirty = dirty.iter().into_iter().any(|x| x.0);

        assert_eq!(tlen, 14);
        assert_eq!(tlen, rlen);
        assert_eq!(tlen, slen);
        assert_eq!(dirty, false);
    }

    //when first added - notes do not have their local_transform updated
    {
        let (translations, rotations, scales, local_transforms, world_transforms) = world.borrow::<(&Translation, &Rotation, &Scale, &LocalTransform, &WorldTransform)>(); 

        (&translations, &rotations, &scales, &local_transforms, &world_transforms)
            .iter()
            .for_each(|data| {
                let (_translation, _rotation, _scale, local_transform, _world_transform) = (&(data.0).0, &(data.1).0, &(data.2).0, &(data.3).0, &(data.4).0);
                assert_eq!(Vec3::new(0.0, 0.0, 0.0), get_translation(local_transform));
            }); 
    }

    //update local_transform - world_transform should be unchanged
    world.run_system::<sg::systems::TrsToLocal>();

    //this now unmarks trs as dirty, but marks world as dirty 
    {
        let (translations, rotations, scales, dirty) = world.borrow::<(&Translation, &Rotation, &Scale, &DirtyTransform)>(); 

        let tlen = translations.inserted_or_modified().len();
        let rlen = rotations.inserted_or_modified().len();
        let slen = scales.inserted_or_modified().len();
        let dirty = dirty.iter().into_iter().any(|x| x.0);

        assert_eq!(tlen, 0);
        assert_eq!(tlen, rlen);
        assert_eq!(tlen, slen);
        assert_eq!(dirty, true);
    }

    //now local_transform should match (world_transform is unchanged)
    {
        let (translations, rotations, scales, local_transforms, world_transforms) = world.borrow::<(&Translation, &Rotation, &Scale, &LocalTransform, &WorldTransform)>(); 
        (&translations, &rotations, &scales, &local_transforms, &world_transforms)
            .iter()
            .with_id()
            .for_each(|(id, data)| {
                let (translation, _rotation, _scale, local_transform, world_transform) = (&(data.0).0, &(data.1).0, &(data.2).0, &(data.3).0, &(data.4).0);

                if id == root {
                    assert_eq!(Vec3::new(0.0, 0.0, 0.0), get_translation(local_transform));
                } else if id == g || id == j {
                    assert_eq!(Vec3::new(20.0, 0.0, 0.0), get_translation(local_transform));
                } else {
                    assert_eq!(Vec3::new(10.0, 0.0, 0.0), get_translation(local_transform));
                }

                assert_eq!(*translation, get_translation(local_transform));
                assert_eq!(Vec3::new(0.0, 0.0, 0.0), get_translation(world_transform));
            }); 
    }

    
    //update world_transforms
    world.run_system::<sg::systems::LocalToWorld>();

    //nothing should be dirty
    {
        let (translations, rotations, scales, dirty) = world.borrow::<(&Translation, &Rotation, &Scale, &DirtyTransform)>(); 

        let tlen = translations.inserted_or_modified().len();
        let rlen = rotations.inserted_or_modified().len();
        let slen = scales.inserted_or_modified().len();
        let dirty = dirty.iter().into_iter().any(|x| x.0);

        assert_eq!(tlen, 0);
        assert_eq!(tlen, rlen);
        assert_eq!(tlen, slen);
        assert_eq!(dirty, false);
    }

    //local_transfrom should not be affected
    {
        let (translations, rotations, scales, local_transforms, world_transforms) = world.borrow::<(&Translation, &Rotation, &Scale, &LocalTransform, &WorldTransform)>(); 
        (&translations, &rotations, &scales, &local_transforms, &world_transforms)
            .iter()
            .with_id()
            .for_each(|(id, data)| {
                let (translation, _rotation, _scale, local_transform, _world_transform) = (&(data.0).0, &(data.1).0, &(data.2).0, &(data.3).0, &(data.4).0);
                if id == root {
                    assert_eq!(Vec3::new(0.0, 0.0, 0.0), get_translation(local_transform));
                } else if id == g || id == j {
                    assert_eq!(Vec3::new(20.0, 0.0, 0.0), get_translation(local_transform));
                } else {
                    assert_eq!(Vec3::new(10.0, 0.0, 0.0), get_translation(local_transform));
                }
                assert_eq!(*translation, get_translation(local_transform));
                //println!("{:?} {:?}", id, translation);
            }); 
    }


    //check all the world transforms
    {
        let world_storage = world.borrow::<&WorldTransform>();


        let world_transform = (&world_storage).get(root).unwrap();
        assert_eq!(Vec3::new(0.0,0.0, 0.0), get_translation(&world_transform.0));

        let world_transform = (&world_storage).get(a).unwrap();
        assert_eq!(Vec3::new(10.0,0.0, 0.0), get_translation(&world_transform.0));

        let world_transform = (&world_storage).get(b).unwrap();
        assert_eq!(Vec3::new(10.0,0.0, 0.0), get_translation(&world_transform.0));

        let world_transform = (&world_storage).get(c).unwrap();
        assert_eq!(Vec3::new(10.0,0.0, 0.0), get_translation(&world_transform.0));

        let world_transform = (&world_storage).get(d).unwrap();
        assert_eq!(Vec3::new(20.0,0.0, 0.0), get_translation(&world_transform.0));

        let world_transform = (&world_storage).get(e).unwrap();
        assert_eq!(Vec3::new(20.0,0.0, 0.0), get_translation(&world_transform.0));

        let world_transform = (&world_storage).get(f).unwrap();
        assert_eq!(Vec3::new(20.0,0.0, 0.0), get_translation(&world_transform.0));

        let world_transform = (&world_storage).get(g).unwrap();
        assert_eq!(Vec3::new(30.0,0.0, 0.0), get_translation(&world_transform.0));

        let world_transform = (&world_storage).get(h).unwrap();
        assert_eq!(Vec3::new(30.0,0.0, 0.0), get_translation(&world_transform.0));

        let world_transform = (&world_storage).get(i).unwrap();
        assert_eq!(Vec3::new(30.0,0.0, 0.0), get_translation(&world_transform.0));

        let world_transform = (&world_storage).get(j).unwrap();
        assert_eq!(Vec3::new(50.0,0.0, 0.0), get_translation(&world_transform.0));

        let world_transform = (&world_storage).get(k).unwrap();
        assert_eq!(Vec3::new(40.0,0.0, 0.0), get_translation(&world_transform.0));
        
        let world_transform = (&world_storage).get(l).unwrap();
        assert_eq!(Vec3::new(40.0,0.0, 0.0), get_translation(&world_transform.0));

        let world_transform = (&world_storage).get(m).unwrap();
        assert_eq!(Vec3::new(60.0,0.0, 0.0), get_translation(&world_transform.0));

        let world_transform = (&world_storage).get(n).unwrap();
        assert_eq!(Vec3::new(70.0,0.0, 0.0), get_translation(&world_transform.0));
    }
    //debugging - print tree with transforms
    /*
    {
        let (parent_storage, child_storage, translation_storage, world_storage) = world.borrow::<(&Parent, &Child, &Translation, &WorldTransform)>();
        let storages = (&parent_storage, &child_storage);
        println!("{:?}", storages.debug_tree(entities.0, |e| {
            format!("{:?}: Local: {:?} World: {:?}", 
                labels.get(&e).unwrap(), 
                &(&translation_storage).get(e).unwrap().0,
                get_translation(&(&world_storage).get(e).unwrap().0)
            )
        }));
    }
    */
}

#[test]
fn test_transform_dirty() {
    let (world, entities, _labels) = create_scene_graph();
    let (root, a,b,c,d,e,f,g,h,i,j,k,l,m,n) = entities;

    world.run_system::<sg::systems::TrsToLocal>();
    world.run_system::<sg::systems::LocalToWorld>();

    //check all the world transforms before making changes
    {
        let world_storage = world.borrow::<&WorldTransform>();


        let world_transform = (&world_storage).get(root).unwrap();
        assert_eq!(Vec3::new(0.0,0.0, 0.0), get_translation(&world_transform.0));

        let world_transform = (&world_storage).get(a).unwrap();
        assert_eq!(Vec3::new(10.0,0.0, 0.0), get_translation(&world_transform.0));

        let world_transform = (&world_storage).get(b).unwrap();
        assert_eq!(Vec3::new(10.0,0.0, 0.0), get_translation(&world_transform.0));

        let world_transform = (&world_storage).get(c).unwrap();
        assert_eq!(Vec3::new(10.0,0.0, 0.0), get_translation(&world_transform.0));

        let world_transform = (&world_storage).get(d).unwrap();
        assert_eq!(Vec3::new(20.0,0.0, 0.0), get_translation(&world_transform.0));

        let world_transform = (&world_storage).get(e).unwrap();
        assert_eq!(Vec3::new(20.0,0.0, 0.0), get_translation(&world_transform.0));

        let world_transform = (&world_storage).get(f).unwrap();
        assert_eq!(Vec3::new(20.0,0.0, 0.0), get_translation(&world_transform.0));

        let world_transform = (&world_storage).get(g).unwrap();
        assert_eq!(Vec3::new(30.0,0.0, 0.0), get_translation(&world_transform.0));

        let world_transform = (&world_storage).get(h).unwrap();
        assert_eq!(Vec3::new(30.0,0.0, 0.0), get_translation(&world_transform.0));

        let world_transform = (&world_storage).get(i).unwrap();
        assert_eq!(Vec3::new(30.0,0.0, 0.0), get_translation(&world_transform.0));

        let world_transform = (&world_storage).get(j).unwrap();
        assert_eq!(Vec3::new(50.0,0.0, 0.0), get_translation(&world_transform.0));

        let world_transform = (&world_storage).get(k).unwrap();
        assert_eq!(Vec3::new(40.0,0.0, 0.0), get_translation(&world_transform.0));
        
        let world_transform = (&world_storage).get(l).unwrap();
        assert_eq!(Vec3::new(40.0,0.0, 0.0), get_translation(&world_transform.0));

        let world_transform = (&world_storage).get(m).unwrap();
        assert_eq!(Vec3::new(60.0,0.0, 0.0), get_translation(&world_transform.0));

        let world_transform = (&world_storage).get(n).unwrap();
        assert_eq!(Vec3::new(70.0,0.0, 0.0), get_translation(&world_transform.0));
    }

    //make a change
    #[cfg(not(feature = "easy_deref"))]
    {
        let mut translation_storage = world.borrow::<&mut Translation>();
        let translation = (&mut translation_storage).get(a).unwrap();
        translation.0.y = 200.0;
        let translation = (&mut translation_storage).get(g).unwrap();
        translation.0.y = 300.0;
        let translation = (&mut translation_storage).get(m).unwrap();
        translation.0.y = 400.0;
    }
    //might as well test easy_deref here too ;)
    #[cfg(feature = "easy_deref")]
    {
        let mut translation_storage = world.borrow::<&mut Translation>();
        let translation = (&mut translation_storage).get(a).unwrap();
        translation.y = 200.0;
        let translation = (&mut translation_storage).get(g).unwrap();
        translation.y = 300.0;
        let translation = (&mut translation_storage).get(m).unwrap();
        translation.y = 400.0;
    }

    world.run_system::<sg::systems::TrsToLocal>();
    world.run_system::<sg::systems::LocalToWorld>();

    //check all the transforms after making changes
    {
        let world_storage = world.borrow::<&WorldTransform>();

        let world_transform = (&world_storage).get(root).unwrap();
        assert_eq!(Vec3::new(0.0,0.0, 0.0), get_translation(&world_transform.0));

        let world_transform = (&world_storage).get(a).unwrap();
        assert_eq!(Vec3::new(10.0,200.0, 0.0), get_translation(&world_transform.0));

        let world_transform = (&world_storage).get(b).unwrap();
        assert_eq!(Vec3::new(10.0,0.0, 0.0), get_translation(&world_transform.0));

        let world_transform = (&world_storage).get(c).unwrap();
        assert_eq!(Vec3::new(10.0,0.0, 0.0), get_translation(&world_transform.0));

        let world_transform = (&world_storage).get(d).unwrap();
        assert_eq!(Vec3::new(20.0,200.0, 0.0), get_translation(&world_transform.0));

        let world_transform = (&world_storage).get(e).unwrap();
        assert_eq!(Vec3::new(20.0,200.0, 0.0), get_translation(&world_transform.0));

        let world_transform = (&world_storage).get(f).unwrap();
        assert_eq!(Vec3::new(20.0,0.0, 0.0), get_translation(&world_transform.0));

        let world_transform = (&world_storage).get(g).unwrap();
        assert_eq!(Vec3::new(30.0,300.0, 0.0), get_translation(&world_transform.0));

        let world_transform = (&world_storage).get(h).unwrap();
        assert_eq!(Vec3::new(30.0,200.0, 0.0), get_translation(&world_transform.0));

        let world_transform = (&world_storage).get(i).unwrap();
        assert_eq!(Vec3::new(30.0,200.0, 0.0), get_translation(&world_transform.0));

        let world_transform = (&world_storage).get(j).unwrap();
        assert_eq!(Vec3::new(50.0,300.0, 0.0), get_translation(&world_transform.0));

        let world_transform = (&world_storage).get(k).unwrap();
        assert_eq!(Vec3::new(40.0,300.0, 0.0), get_translation(&world_transform.0));
        
        let world_transform = (&world_storage).get(l).unwrap();
        assert_eq!(Vec3::new(40.0,200.0, 0.0), get_translation(&world_transform.0));

        let world_transform = (&world_storage).get(m).unwrap();
        assert_eq!(Vec3::new(60.0,700.0, 0.0), get_translation(&world_transform.0));

        let world_transform = (&world_storage).get(n).unwrap();
        assert_eq!(Vec3::new(70.0,700.0, 0.0), get_translation(&world_transform.0));
    }

}

#[test]
fn test_transform_dirty_ids() {
    let (world, entities, _labels) = create_scene_graph();
    let (_root, _a,_b,_c,d,e,_f,_g,h,i,j,_k,l,m,n) = entities;

    world.run_system::<sg::systems::TrsToLocal>();
    world.run_system::<sg::systems::LocalToWorld>();

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
        let dirty_storage = world.borrow::<&DirtyTransform>();

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
        let mut translation_storage = world.borrow::<&mut Translation>();
        let translation = (&mut translation_storage).get(d).unwrap();
        translation.0.y = 200.0;
        let translation = (&mut translation_storage).get(e).unwrap();
        translation.0.y = 300.0;
        let translation = (&mut translation_storage).get(j).unwrap();
        translation.0.y = 400.0;
    }


    //nothing should be dirty until we run TrsToLocal
    {
        assert_eq!(get_marked_dirty_ids(&world).len(), 0);
    }

    world.run_system::<sg::systems::TrsToLocal>();

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
        world.borrow::<( Unique<&TransformRoot>, &Parent, &Child, &LocalTransform, &mut DirtyTransform, &mut WorldTransform, )>();

    fn update(dirty_list:&mut Vec<EntityId>, id: EntityId, mut dirty: bool, _parent: EntityId, parent_storage: &View<Parent>, child_storage: &View<Child>, local_transform_storage: &View<LocalTransform>, dirty_transform_storage: &mut ViewMut<DirtyTransform>, world_transform_storage: &mut ViewMut<WorldTransform>) {
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

/*
       *
       |
    |--|--|
    A  B  C
  |-|     |-|
  D E     F G
|-|         |-|
H I         J K
|           |
L           M
            |
            N

Breadth-first: alphabetical
Depth-first: A,D,H,L,I,E,B,C,F,G,J,M,N,K
*/
fn create_scene_graph() -> (World, TestEntities, HashMap<EntityId, &'static str>) {

    let world = World::new();
  
    let mut labels = HashMap::<EntityId, &'static str>::new();

    let entities = {
        let root = sg::init(&world);

        //attach them somewhat out of order
        let a = sg::spawn_child(&world, None, Some(Vec3::new(10.0, 0.0, 0.0)), None, None);
        let b = sg::spawn_child(&world, None, Some(Vec3::new(10.0, 0.0, 0.0)), None, None);
        let c = sg::spawn_child(&world, None, Some(Vec3::new(10.0, 0.0, 0.0)), None, None);
        let d = sg::spawn_child(&world, Some(a), Some(Vec3::new(10.0, 0.0, 0.0)), None, None);
        let e = sg::spawn_child(&world, Some(a), Some(Vec3::new(10.0, 0.0, 0.0)), None, None);
        let f = sg::spawn_child(&world, Some(c), Some(Vec3::new(10.0, 0.0, 0.0)), None, None);
        let g = sg::spawn_child(&world, Some(c), Some(Vec3::new(20.0, 0.0, 0.0)), None, None);
        let h = sg::spawn_child(&world, Some(d), Some(Vec3::new(10.0, 0.0, 0.0)), None, None);
        let i = sg::spawn_child(&world, Some(d), Some(Vec3::new(10.0, 0.0, 0.0)), None, None);
        let l = sg::spawn_child(&world, Some(h), Some(Vec3::new(10.0, 0.0, 0.0)), None, None);
        let j = sg::spawn_child(&world, Some(g), Some(Vec3::new(20.0, 0.0, 0.0)), None, None);
        let m = sg::spawn_child(&world, Some(j), Some(Vec3::new(10.0, 0.0, 0.0)), None, None);
        let k = sg::spawn_child(&world, Some(g), Some(Vec3::new(10.0, 0.0, 0.0)), None, None);
        let n = sg::spawn_child(&world, Some(m), Some(Vec3::new(10.0, 0.0, 0.0)), None, None);
   
        labels.insert(root, "root");
        labels.insert(a, "a");
        labels.insert(b, "b");
        labels.insert(c, "c");
        labels.insert(d, "d");
        labels.insert(e, "e");
        labels.insert(f, "f");
        labels.insert(g, "g");
        labels.insert(h, "h");
        labels.insert(i, "i");
        labels.insert(j, "j");
        labels.insert(k, "k");
        labels.insert(l, "l");
        labels.insert(m, "m");
        labels.insert(n, "n");

        (root, a,b,c,d,e,f,g,h,i,j,k,l,m,n)
    };


    (world, entities, labels)
}


type TestEntities = (
    EntityId, //root
    EntityId, //a
    EntityId, //b
    EntityId, //c
    EntityId, //d
    EntityId, //e
    EntityId, //f
    EntityId, //g
    EntityId, //h
    EntityId, //i
    EntityId, //j
    EntityId, //k
    EntityId, //l
    EntityId, //m
    EntityId, //n
);

fn get_translation(mat:&Matrix4) -> Vec3 {
    let values = mat.as_slice();
    Vec3::new(values[12], values[13], values[14])
}