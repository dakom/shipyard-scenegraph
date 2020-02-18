use shipyard::prelude::*;
use shipyard_scenegraph::*;
use std::collections::HashMap;

#[test]
fn test_hierarchy() {
    let world = World::new();

    let mut storages = world.borrow::<(EntitiesMut, &mut Parent, &mut Child)>();

    let mut hierarchy = (&mut storages.0, &mut storages.1, &mut storages.2);

    let entities = &mut hierarchy.0;

    let root1 = entities.add_entity((), ());
    let root2 = entities.add_entity((), ());

    let e1 = hierarchy.attach_new(root1);
    let e2 = hierarchy.attach_new(e1);
    let e3 = hierarchy.attach_new(e1);
    let e4 = hierarchy.attach_new(e3);

    hierarchy.attach(e3, root2);

    let e5 = hierarchy.attach_new(e3);

    {
        let storages = (&*hierarchy.1, &*hierarchy.2);

        assert!(storages.children(e3).eq([e4, e5].iter().cloned()));
        assert!(storages.ancestors(e4).eq([e3, root2].iter().cloned()));

        assert!(storages.descendants_depth_first(root1).eq([e1, e2].iter().cloned()));
        assert!(storages.descendants_depth_first(root2).eq([e3, e4, e5].iter().cloned()));
    }

    hierarchy.remove_single(e1);

    {
        let storages = (&*hierarchy.1, &*hierarchy.2);
        assert!(storages.children(e1).eq(None));
    }

    hierarchy.remove(root2);

    {
        let storages = (&*hierarchy.1, &*hierarchy.2);
        assert!(storages.descendants_depth_first(root2).eq(None));
        assert!(storages.descendants_depth_first(e3).eq(None));
        assert!(storages.ancestors(e5).eq(None));
    }
}


#[test]
fn test_sorting_depth_first() {
    let world = World::new();

    let (mut hierarchy, mut usizes) = world.borrow::<((EntitiesMut, &mut Parent, &mut Child), &mut usize)>();

    let mut hierarchy = (&mut hierarchy.0, &mut hierarchy.1, &mut hierarchy.2);

    let root = {
        let entities = &mut hierarchy.0;
        entities.add_entity((), ())
    };

    let e0 = hierarchy.attach_new(root);
    let e1 = hierarchy.attach_new(root);
    let e2 = hierarchy.attach_new(root);
    let e3 = hierarchy.attach_new(root);
    let e4 = hierarchy.attach_new(root);

    {
        let entities = &mut hierarchy.0;
        entities.add_component(&mut usizes, 7, e0);
        entities.add_component(&mut usizes, 5, e1);
        entities.add_component(&mut usizes, 6, e2);
        entities.add_component(&mut usizes, 1, e3);
        entities.add_component(&mut usizes, 3, e4);
    }

    {
        let storages = (&*hierarchy.1, &*hierarchy.2);
        assert!(storages.children(root).eq([e0, e1, e2, e3, e4].iter().cloned()));
    }

    hierarchy.sort_children_by(root, |a, b| usizes[*a].cmp(&usizes[*b]));

    {
        let storages = (&*hierarchy.1, &*hierarchy.2);
        assert!(storages.children(root).eq([e3, e4, e1, e2, e0].iter().cloned()));
    }
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
fn create_world_tree() -> (World, TestEntities, HashMap<EntityId, &'static str>) {

    let world = World::new();

    let mut labels = HashMap::<EntityId, &'static str>::new();

    let entities = {
        let mut hierarchy = world.borrow::<(EntitiesMut, &mut Parent, &mut Child)>();

        let mut hierarchy = (&mut hierarchy.0, &mut hierarchy.1, &mut hierarchy.2);
        let entities = &mut hierarchy.0;

        
        let root = entities.add_entity((), ());

        //attach them somewhat out of order
        let a = hierarchy.attach_new(root);
        let b = hierarchy.attach_new(root);
        let c = hierarchy.attach_new(root);
        let d = hierarchy.attach_new(a);
        let e = hierarchy.attach_new(a);
        let f = hierarchy.attach_new(c);
        let g = hierarchy.attach_new(c);
        let h = hierarchy.attach_new(d);
        let i = hierarchy.attach_new(d);
        let l = hierarchy.attach_new(h);
        let j = hierarchy.attach_new(g);
        let m = hierarchy.attach_new(j);
        let k = hierarchy.attach_new(g);
        let n = hierarchy.attach_new(m);
   
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
#[test]
fn test_hierarchy_tree() {

    let (world, (root, a,b,c,d,e,f,g,h,i,j,k,l,m,n), _) = create_world_tree();

    {
        let (parent_storage, child_storage) = world.borrow::<(&Parent, &Child)>();

        let storages = (&parent_storage, &child_storage);
        assert!(storages.descendants_depth_first(root).eq([a,d,h,l,i,e,b,c,f,g,j,m,n,k].iter().cloned()));
        assert!(storages.descendants_breadth_first(root).eq([a,b,c,d,e,f,g,h,i,j,k,l,m,n].iter().cloned()));
    }
}


#[test]
fn test_debug_print() {
    let world = World::new();

    let mut hierarchy = world.borrow::<(EntitiesMut, &mut Parent, &mut Child)>();

    let mut hierarchy = (&mut hierarchy.0, &mut hierarchy.1, &mut hierarchy.2);
    let entities = &mut hierarchy.0;

    let root = entities.add_entity((), ());

    let e1 = hierarchy.attach_new(root);
    let e2 = hierarchy.attach_new(root);

    let _e3 = hierarchy.attach_new(e1);
    let _e4 = hierarchy.attach_new(e1);
    
    let e5 = hierarchy.attach_new(e2);
    let e6 = hierarchy.attach_new(e2);
    let _e7 = hierarchy.attach_new(e5);
    let _e8 = hierarchy.attach_new(e6);


    {
        let storages = (&*hierarchy.1, &*hierarchy.2);
        assert_eq!(EXPECTED_DEBUG_TREE_1, format!("{:?}", storages.debug_tree(root, |e| format!("{:?}", e))));
    }

    let (world, entities, labels) = create_world_tree();

    {

        let (parent_storage, child_storage) = world.borrow::<(&Parent, &Child)>();
        let storages = (&parent_storage, &child_storage);
        assert_eq!(EXPECTED_DEBUG_TREE_2, format!("{:?}", storages.debug_tree(entities.0, |e| labels.get(&e).unwrap().to_string())));
        //println!("{:?}", storages.debug_tree(entities.0, |e| labels.get(&e).unwrap().to_string()));
    }
}

const EXPECTED_DEBUG_TREE_1:&'static str = 
r#"EntityId { index: 0, version: 0 }
  EntityId { index: 1, version: 0 }
    EntityId { index: 3, version: 0 }
    EntityId { index: 4, version: 0 }
  EntityId { index: 2, version: 0 }
    EntityId { index: 5, version: 0 }
      EntityId { index: 7, version: 0 }
    EntityId { index: 6, version: 0 }
      EntityId { index: 8, version: 0 }
"#;

const EXPECTED_DEBUG_TREE_2:&'static str =
r#"root
  a
    d
      h
        l
      i
    e
  b
  c
    f
    g
      j
        m
          n
      k
"#;