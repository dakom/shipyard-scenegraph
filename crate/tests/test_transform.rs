use shipyard::prelude::*;
use shipyard_scenegraph::{self as sg, *};
use std::collections::HashMap;

#[test]
fn test_transform() {
    let (world, entities, labels) = create_scene_graph();
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
                let (translation, rotation, scale, local_transform, world_transform) = (&(data.0).0, &(data.1).0, &(data.2).0, &(data.3).0, &(data.4).0);
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
                let (translation, rotation, scale, local_transform, world_transform) = (&(data.0).0, &(data.1).0, &(data.2).0, &(data.3).0, &(data.4).0);

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
                let (translation, rotation, scale, local_transform, world_transform) = (&(data.0).0, &(data.1).0, &(data.2).0, &(data.3).0, &(data.4).0);
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
    let (world, entities, labels) = create_scene_graph();
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
    {
        let mut translation_storage = world.borrow::<&mut Translation>();
        let translation = (&mut translation_storage).get(a).unwrap();
        translation.0.y = 200.0;
        let translation = (&mut translation_storage).get(g).unwrap();
        translation.0.y = 300.0;
        let translation = (&mut translation_storage).get(m).unwrap();
        translation.0.y = 400.0;
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

    //debugging - print tree with transforms
    /*
    {
        let (parent_storage, child_storage, translation_storage, world_storage) = world.borrow::<(&Parent, &Child, &Translation, &WorldTransform)>();
        let storages = (&parent_storage, &child_storage);
        println!("{:?}", storages.debug_tree(root, |e| {
            format!("{:?}: Local: {:?} World: {:?}", 
                labels.get(&e).unwrap(), 
                (&translation_storage).get(e).unwrap().0,
                get_translation(&(&world_storage).get(e).unwrap().0)
            )
        }));
    }
    */
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

fn get_translation(mat:&Matrix4) -> Vec3 {
    Vec3::new(mat.12, mat.13, mat.14)
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
