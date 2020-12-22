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
fn test_transform_basic() {
    let (world, entities, labels) = create_scene_graph();
    let (root, a,b,c,d,e,f,g,h,i,j,k,l,m,n) = entities;

    //adding the entities makes trs dirty, but not yet world
    {
        let (translations, rotations, scales, dirty) 
            = world.borrow::<(View<Translation>, View<Rotation>, View<Scale>, View<DirtyTransform>)>().unwrap(); 

        let tlen = translations.inserted_or_modified().iter().count();
        let rlen = rotations.inserted_or_modified().iter().count();
        let slen = scales.inserted_or_modified().iter().count();
        let dirty = dirty.iter().into_iter().any(|x| x.0);

        assert_eq!(tlen, 14);
        assert_eq!(tlen, rlen);
        assert_eq!(tlen, slen);
        assert_eq!(dirty, false);
    }

    //when first added - nodes do not have their local_transform updated
    {
        world.borrow::<View<LocalTransform>>()
            .unwrap()
            .iter()
            .for_each(|local_transform| {
                assert_eq!(Vec3::new(0.0, 0.0, 0.0), get_translation(local_transform));
            }); 
    }
    //update local_transform - world_transform should be unchanged
    world.run(trs_to_local).unwrap();

    //this now unmarks trs as dirty, but marks world as dirty 
    {
        let (translations, rotations, scales, dirty) 
            = world.borrow::<(View<Translation>, View<Rotation>, View<Scale>, View<DirtyTransform>)>().unwrap(); 

        let tlen = translations.inserted_or_modified().iter().count();
        let rlen = rotations.inserted_or_modified().iter().count();
        let slen = scales.inserted_or_modified().iter().count();
        let dirty = dirty.iter().into_iter().any(|x| x.0);

        assert_eq!(tlen, 0);
        assert_eq!(tlen, rlen);
        assert_eq!(tlen, slen);
        assert_eq!(dirty, true);
    }

    //now local_transform should match (world_transform is unchanged)
    {
        let (translations, rotations, scales, local_transforms, world_transforms) 
            = world.borrow::<(View<Translation>, View<Rotation>, View<Scale>, View<LocalTransform>, View<WorldTransform>)>().unwrap(); 
        (&translations, &local_transforms, &world_transforms)
            .iter()
            .with_id()
            .for_each(|(id, (translation, local_transform, world_transform))| {

                if id == root {
                    assert_eq!(Vec3::new(0.0, 0.0, 0.0), get_translation(local_transform));
                } else if id == g || id == j {
                    assert_eq!(Vec3::new(20.0, 0.0, 0.0), get_translation(local_transform));
                } else {
                    assert_eq!(Vec3::new(10.0, 0.0, 0.0), get_translation(local_transform));
                }

                assert_eq!(translation.values(), &get_translation(local_transform));
                assert_eq!(Vec3::new(0.0, 0.0, 0.0), get_translation(world_transform));
            }); 
    }
    //update world_transforms
    world.run(local_to_world).unwrap();

    //nothing should be dirty
    {
        let (translations, rotations, scales, dirty) 
            = world.borrow::<(View<Translation>, View<Rotation>, View<Scale>, View<DirtyTransform>)>().unwrap(); 

        let tlen = translations.inserted_or_modified().iter().count();
        let rlen = rotations.inserted_or_modified().iter().count();
        let slen = scales.inserted_or_modified().iter().count();
        let dirty = dirty.iter().into_iter().any(|x| x.0);

        assert_eq!(tlen, 0);
        assert_eq!(tlen, rlen);
        assert_eq!(tlen, slen);
        assert_eq!(dirty, false);
    }

    //local_transfrom should not be affected
    {
        let (translations, rotations, scales, local_transforms, world_transforms) 
            = world.borrow::<(View<Translation>, View<Rotation>, View<Scale>, View<LocalTransform>, View<WorldTransform>)>().unwrap(); 

        (&translations, &local_transforms)
            .iter()
            .with_id()
            .for_each(|(id, (translation, local_transform))| {
                if id == root {
                    assert_eq!(Vec3::new(0.0, 0.0, 0.0), get_translation(local_transform));
                } else if id == g || id == j {
                    assert_eq!(Vec3::new(20.0, 0.0, 0.0), get_translation(local_transform));
                } else {
                    assert_eq!(Vec3::new(10.0, 0.0, 0.0), get_translation(local_transform));
                }
                assert_eq!(translation.values(), &get_translation(local_transform));
            }); 
    }

    //check all the world transforms
    {
        let world_storage = world.borrow::<View<WorldTransform>>().unwrap();


        let world_transform = (&world_storage).get(root).unwrap();
        assert_eq!(Vec3::new(0.0,0.0, 0.0), get_translation(&world_transform));

        let world_transform = (&world_storage).get(a).unwrap();
        assert_eq!(Vec3::new(10.0,0.0, 0.0), get_translation(&world_transform));

        let world_transform = (&world_storage).get(b).unwrap();
        assert_eq!(Vec3::new(10.0,0.0, 0.0), get_translation(&world_transform));

        let world_transform = (&world_storage).get(c).unwrap();
        assert_eq!(Vec3::new(10.0,0.0, 0.0), get_translation(&world_transform));

        let world_transform = (&world_storage).get(d).unwrap();
        assert_eq!(Vec3::new(20.0,0.0, 0.0), get_translation(&world_transform));

        let world_transform = (&world_storage).get(e).unwrap();
        assert_eq!(Vec3::new(20.0,0.0, 0.0), get_translation(&world_transform));

        let world_transform = (&world_storage).get(f).unwrap();
        assert_eq!(Vec3::new(20.0,0.0, 0.0), get_translation(&world_transform));

        let world_transform = (&world_storage).get(g).unwrap();
        assert_eq!(Vec3::new(30.0,0.0, 0.0), get_translation(&world_transform));

        let world_transform = (&world_storage).get(h).unwrap();
        assert_eq!(Vec3::new(30.0,0.0, 0.0), get_translation(&world_transform));

        let world_transform = (&world_storage).get(i).unwrap();
        assert_eq!(Vec3::new(30.0,0.0, 0.0), get_translation(&world_transform));

        let world_transform = (&world_storage).get(j).unwrap();
        assert_eq!(Vec3::new(50.0,0.0, 0.0), get_translation(&world_transform));

        let world_transform = (&world_storage).get(k).unwrap();
        assert_eq!(Vec3::new(40.0,0.0, 0.0), get_translation(&world_transform));
        
        let world_transform = (&world_storage).get(l).unwrap();
        assert_eq!(Vec3::new(40.0,0.0, 0.0), get_translation(&world_transform));

        let world_transform = (&world_storage).get(m).unwrap();
        assert_eq!(Vec3::new(60.0,0.0, 0.0), get_translation(&world_transform));

        let world_transform = (&world_storage).get(n).unwrap();
        assert_eq!(Vec3::new(70.0,0.0, 0.0), get_translation(&world_transform));
    }

    //debugging - print tree with transforms
    {
        let (parent_storage, child_storage, translation_storage, world_storage) 
            = world.borrow::<(View<Parent<SceneGraph>>, View<Child<SceneGraph>>, View<Translation>, View<WorldTransform>)>().unwrap();

        let storages = (&parent_storage, &child_storage);
        println!("{:?}", storages.debug_tree(entities.0, |e| {
            format!("{:?}: Local: {:?} World: {:?}", 
                labels.get(&e).unwrap(), 
                translation_storage.get(e).unwrap().values(),
                get_translation(world_storage.get(e).unwrap())
            )
        }));
    }
}
