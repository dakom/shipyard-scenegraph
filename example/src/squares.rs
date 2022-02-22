use crate::{
    renderer::item::*,
    physics::*
};
use shipyard_scenegraph::prelude::*;
use shipyard::*;
use nalgebra::Vector3;

pub fn create(world:&World, stage_width: f64, stage_height: f64) {


    let mut index: u32 = 0;

    let mut depth = 0.0;
    let mut create_square = |parent:Option<EntityId>, has_spin: bool, visible: bool, width: u32, height: u32, r: f64, g: f64, b: f64| -> EntityId {

        let origin = {
            if has_spin {
                Some(Vector3::new((width as f64)/2.0, (height as f64)/2.0, 0.0))
            } else {
                None
            }
        };

        let translation = {
            if has_spin {
                None
            } else {
                Some(if parent.is_none() {
                    Vector3::new(0.5 * (stage_width - (width as f64)), 0.5 * (stage_height - (height as f64)), depth)
                } else {
                    Vector3::new((width as f64)/2.0, (height as f64)/2.0, depth)
                })
            }
        };

        let entity = {
            let mut storages = world.borrow::<SceneGraphStoragesMut>().unwrap();
            storages.spawn_child_trs_origin(
                parent,
                translation,
                None,
                None,
                origin,
            )
        };

        depth = 1.0;

        {
           
            let (entities, mut areas, mut colors, mut spins, mut interactables, mut lookup) 
                = world.borrow::<(EntitiesViewMut, ViewMut<ImageArea>, ViewMut<Color>, ViewMut<Spin>, ViewMut<Interactable>, InteractableLookupViewMut)>().unwrap();

            entities.add_component(entity, &mut areas, ImageArea { width, height});

            if visible {
                entities.add_component(entity, &mut colors, Color (r,g,b, 1.0));
                entities.add_component(entity, &mut interactables, Interactable(index));
                lookup.insert(index, entity);
                index += 1;
            }
            if has_spin {
                entities.add_component(entity, &mut spins, Spin(0.0));
            } 

        }

        entity
    };

    let square = create_square(None, false, true, 400, 400, 1.0, 0.0, 0.0);
    let square = create_square(Some(square), false, true, 200, 200, 0.0, 1.0, 0.0);
    let square = create_square(Some(square), false, false, 100, 100, 0.0, 0.0, 1.0);
    let _square = create_square(Some(square), true, true, 100, 100, 0.0, 0.0, 1.0);
}

