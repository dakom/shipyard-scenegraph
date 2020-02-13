use shipyard::prelude::*;
use shipyard_scenegraph::*;

#[test]
fn test_hierarchy() {
    let world = World::new();

    let mut hierarchy = world.borrow::<(EntitiesMut, &mut Parent, &mut Child)>();

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
        let storages = (&hierarchy.1, &hierarchy.2);

        assert!(storages.children(e3).eq([e4, e5].iter().cloned()));
        assert!(storages.ancestors(e4).eq([e3, root2].iter().cloned()));

        assert!(storages.descendants_depth_first(root1).eq([e1, e2].iter().cloned()));

        assert!(storages.descendants_depth_first(root2).eq([e3, e4, e5].iter().cloned()));
    }

    detach(&mut hierarchy, e1);

    {
        let storages = (&hierarchy.1, &hierarchy.2);
        assert!(storages.descendants_depth_first(root1).eq(None));
        assert!(storages.ancestors(e1).eq(None));
        assert!(storages.children(e1).eq([e2].iter().cloned()));
    }

    hierarchy.remove_single(e1);

    {
        let storages = (&hierarchy.1, &hierarchy.2);
        assert!(storages.children(e1).eq(None));
    }

    hierarchy.remove(root2);

    {
        let storages = (&hierarchy.1, &hierarchy.2);
        assert!(storages.descendants_depth_first(root2).eq(None));
        assert!(storages.descendants_depth_first(e3).eq(None));
        assert!(storages.ancestors(e5).eq(None));
    }
}