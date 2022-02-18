use shipyard::*;
use shipyard_scenegraph::prelude::*;
use shipyard_scenegraph::traits::required::SliceExt;
use std::collections::{hash_map::Entry, HashMap};
use std::hash::Hash;

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

pub fn create_scene_graph() -> (World, TestEntities, HashMap<EntityId, &'static str>) {
    let world = World::new();

    let mut labels = HashMap::<EntityId, &'static str>::new();

    let entities = {
        let root = init_scenegraph(&world);

        let mut storages = world.borrow::<SceneGraphStoragesMut>().unwrap();

        //attach them somewhat out of order
        let a = storages.spawn_child_trs_origin(
            None,
            Some(Vec3::new(10.0, 0.0, 0.0)),
            None,
            None,
            None,
        );
        let b = storages.spawn_child_trs_origin(
            None,
            Some(Vec3::new(10.0, 0.0, 0.0)),
            None,
            None,
            None,
        );
        let c = storages.spawn_child_trs_origin(
            None,
            Some(Vec3::new(10.0, 0.0, 0.0)),
            None,
            None,
            None,
        );
        let d = storages.spawn_child_trs_origin(
            Some(a),
            Some(Vec3::new(10.0, 0.0, 0.0)),
            None,
            None,
            None,
        );
        let e = storages.spawn_child_trs_origin(
            Some(a),
            Some(Vec3::new(10.0, 0.0, 0.0)),
            None,
            None,
            None,
        );
        let f = storages.spawn_child_trs_origin(
            Some(c),
            Some(Vec3::new(10.0, 0.0, 0.0)),
            None,
            None,
            None,
        );
        let g = storages.spawn_child_trs_origin(
            Some(c),
            Some(Vec3::new(20.0, 0.0, 0.0)),
            None,
            None,
            None,
        );
        let h = storages.spawn_child_trs_origin(
            Some(d),
            Some(Vec3::new(10.0, 0.0, 0.0)),
            None,
            None,
            None,
        );
        let i = storages.spawn_child_trs_origin(
            Some(d),
            Some(Vec3::new(10.0, 0.0, 0.0)),
            None,
            None,
            None,
        );
        let l = storages.spawn_child_trs_origin(
            Some(h),
            Some(Vec3::new(10.0, 0.0, 0.0)),
            None,
            None,
            None,
        );
        let j = storages.spawn_child_trs_origin(
            Some(g),
            Some(Vec3::new(20.0, 0.0, 0.0)),
            None,
            None,
            None,
        );
        let m = storages.spawn_child_trs_origin(
            Some(j),
            Some(Vec3::new(10.0, 0.0, 0.0)),
            None,
            None,
            None,
        );
        let k = storages.spawn_child_trs_origin(
            Some(g),
            Some(Vec3::new(10.0, 0.0, 0.0)),
            None,
            None,
            None,
        );
        let n = storages.spawn_child_trs_origin(
            Some(m),
            Some(Vec3::new(10.0, 0.0, 0.0)),
            None,
            None,
            None,
        );

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

        (root, a, b, c, d, e, f, g, h, i, j, k, l, m, n)
    };

    (world, entities, labels)
}

pub type TestEntities = (
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

pub fn get_translation(mat: &Matrix4) -> Vec3 {
    let values = mat.as_slice();
    Vec3::new(values[12], values[13], values[14])
}
