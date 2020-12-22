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
fn test_transform_dirty() {
    let (world, entities, _labels) = create_scene_graph();
    let (root, a,b,c,d,e,f,g,h,i,j,k,l,m,n) = entities;

    world.run(trs_to_local);
    world.run(local_to_world);

    //check all the world transforms before making changes
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

    //make a change
    {
        let mut translations = world.borrow::<ViewMut<Translation>>().unwrap();
        let mut translation = (&mut translations).get(a).unwrap();
        translation.set_y(200.0);
        let mut translation = (&mut translations).get(g).unwrap();
        translation.set_y(300.0);
        let mut translation = (&mut translations).get(m).unwrap();
        translation.set_y(400.0);
    }

    world.run(trs_to_local);
    world.run(local_to_world);

    //check all the transforms after making changes
    {
        let world_storage = world.borrow::<View<WorldTransform>>().unwrap();

        let world_transform = (&world_storage).get(root).unwrap();
        assert_eq!(Vec3::new(0.0,0.0, 0.0), get_translation(&world_transform));

        let world_transform = (&world_storage).get(a).unwrap();
        assert_eq!(Vec3::new(10.0,200.0, 0.0), get_translation(&world_transform));

        let world_transform = (&world_storage).get(b).unwrap();
        assert_eq!(Vec3::new(10.0,0.0, 0.0), get_translation(&world_transform));

        let world_transform = (&world_storage).get(c).unwrap();
        assert_eq!(Vec3::new(10.0,0.0, 0.0), get_translation(&world_transform));

        let world_transform = (&world_storage).get(d).unwrap();
        assert_eq!(Vec3::new(20.0,200.0, 0.0), get_translation(&world_transform));

        let world_transform = (&world_storage).get(e).unwrap();
        assert_eq!(Vec3::new(20.0,200.0, 0.0), get_translation(&world_transform));

        let world_transform = (&world_storage).get(f).unwrap();
        assert_eq!(Vec3::new(20.0,0.0, 0.0), get_translation(&world_transform));

        let world_transform = (&world_storage).get(g).unwrap();
        assert_eq!(Vec3::new(30.0,300.0, 0.0), get_translation(&world_transform));

        let world_transform = (&world_storage).get(h).unwrap();
        assert_eq!(Vec3::new(30.0,200.0, 0.0), get_translation(&world_transform));

        let world_transform = (&world_storage).get(i).unwrap();
        assert_eq!(Vec3::new(30.0,200.0, 0.0), get_translation(&world_transform));

        let world_transform = (&world_storage).get(j).unwrap();
        assert_eq!(Vec3::new(50.0,300.0, 0.0), get_translation(&world_transform));

        let world_transform = (&world_storage).get(k).unwrap();
        assert_eq!(Vec3::new(40.0,300.0, 0.0), get_translation(&world_transform));
        
        let world_transform = (&world_storage).get(l).unwrap();
        assert_eq!(Vec3::new(40.0,200.0, 0.0), get_translation(&world_transform));

        let world_transform = (&world_storage).get(m).unwrap();
        assert_eq!(Vec3::new(60.0,700.0, 0.0), get_translation(&world_transform));

        let world_transform = (&world_storage).get(n).unwrap();
        assert_eq!(Vec3::new(70.0,700.0, 0.0), get_translation(&world_transform));
    }

}