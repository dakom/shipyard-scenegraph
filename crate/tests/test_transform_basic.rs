#![allow(dead_code, unused_imports)]
use shipyard::*;
use shipyard_scenegraph::prelude::*;
use std::borrow::{Borrow, BorrowMut};
use std::collections::{HashMap, hash_map::Entry};
use std::hash::Hash;
mod helpers;
use helpers::*;

#[test]
fn test_transform_basic() {
    let (world, entities, labels) = create_scene_graph();
    let (root, a,b,c,d,e,f,g,h,i,j,k,l,m,n) = entities;

    let mut inserted_count = 0;
    let mut modified_count = 0;
    let mut dirty_count = 0;

    //adding the entities makes inserted dirty
    {
        let (translations, rotations, scales, origins) 
            = world.borrow::<(View<Translation>, View<Rotation>, View<Scale>, View<Origin>)>().unwrap(); 

        inserted_count = translations.inserted().iter().count();
        let rlen = rotations.inserted().iter().count();
        let slen = scales.inserted().iter().count();
        let olen = origins.inserted().iter().count();

        assert_eq!(inserted_count, 14);
        assert_eq!(inserted_count, rlen);
        assert_eq!(inserted_count, slen);
        assert_eq!(inserted_count, olen);
    }
    //but not modified
    {
        let (translations, rotations, scales, origins) 
            = world.borrow::<(View<Translation>, View<Rotation>, View<Scale>, View<Origin>)>().unwrap(); 

        modified_count = translations.modified().iter().count();
        let rlen = rotations.modified().iter().count();
        let slen = scales.modified().iter().count();
        let olen = origins.modified().iter().count();

        assert_eq!(modified_count, 0);
        assert_eq!(modified_count, rlen);
        assert_eq!(modified_count, slen);
        assert_eq!(modified_count, olen);
    }
    //however - yes dirty
    {
        let dirty = world.borrow::<View<DirtyTransform>>().unwrap(); 

        dirty_count = dirty.iter().into_iter().filter(|x| x.0).count();

        assert_eq!(dirty_count, 14);
    }

    //update local_transform
    world.run(local_transform_sys).unwrap();

    //inserted should be 0, everything else should be unchanged
    {
        let (translations, rotations, scales, origins, dirty) 
            = world.borrow::<(View<Translation>, View<Rotation>, View<Scale>, View<Origin>, View<DirtyTransform>)>().unwrap(); 


        assert_eq!(translations.inserted().iter().count(), 0);
        assert_eq!(modified_count, translations.modified().iter().count());
        assert_eq!(dirty_count, dirty.iter().into_iter().filter(|x| x.0).count());
    }

    //local_transform should have the expected values (world_transform is unchanged)
    {
        let (translations, local_transforms, world_transforms) 
            = world.borrow::<(View<Translation>, View<LocalTransform>, View<WorldTransform>)>().unwrap(); 

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
               
                assert_eq!(Borrow::<Vec3>::borrow(translation), &get_translation(local_transform));
                assert_eq!(Vec3::new(0.0, 0.0, 0.0), get_translation(world_transform));
            }); 
    }
    //update world_transforms
    world.run(world_transform_sys).unwrap();

    //nothing should be dirty
    {
        let (translations, rotations, scales, origins, dirty) 
            = world.borrow::<(View<Translation>, View<Rotation>, View<Scale>, View<Origin>, View<DirtyTransform>)>().unwrap(); 

        let tlen = translations.modified().iter().count();
        let rlen = rotations.modified().iter().count();
        let slen = scales.modified().iter().count();
        let olen = origins.modified().iter().count();
        let dlen = dirty.iter().into_iter().filter(|x| x.0).count();

        assert_eq!(tlen, 0);
        assert_eq!(tlen, rlen);
        assert_eq!(tlen, slen);
        assert_eq!(tlen, olen);
        assert_eq!(dlen, 0);
    }

    //local_transfrom should not be affected
    {
        let (translations, local_transforms) 
            = world.borrow::<(View<Translation>, View<LocalTransform>)>().unwrap(); 

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
                assert_eq!(Borrow::<Vec3>::borrow(translation), &get_translation(local_transform));
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
                Borrow::<Vec3>::borrow(translation_storage.get(e).unwrap()),
                get_translation(world_storage.get(e).unwrap())
            )
        }));
    }
}
