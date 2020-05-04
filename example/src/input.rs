use crate::components::*;
use crate::geometry::*;
use shipyard_scenegraph::{Vec3, AsSliceExt, Translation, Origin, WorldTransform};
use std::rc::{Rc};
use gloo_events::{EventListener};
use web_sys::{Event, MouseEvent};
use shipyard::*;
use web_sys::{HtmlCanvasElement};
use wasm_bindgen::{JsCast, UnwrapThrowExt};
pub fn start(world:Rc<World>, canvas:&HtmlCanvasElement) {
    EventListener::new(canvas, "pointerdown", {
        let world = Rc::clone(&world);
        move |event:&Event| {
            let stage_area = world.borrow::<UniqueViewMut<StageArea>>();
            let event = event.dyn_ref::<web_sys::MouseEvent>().unwrap_throw();
            let mouse_point = get_point(&stage_area, &event);

            let (positions, origins, areas, interactables) = world.borrow::<(View<WorldTransform>, View<Origin>, View<ImageArea>, View<Interactable>)>();

            let hits:Vec<(EntityId, Vec3)> = 
                (&positions, &origins, &areas, &interactables)
                    .iter()
                    .with_id()
                    .map(|(id, (transform, origin, obj_area, _interactable))| {
                        //get the position from world matrix
                        let mat = transform.as_slice();
                        let pos = Vec3::new(mat[12], mat[13], mat[14]);
                       
                        if origin.x == 0.0 && origin.y == 0.0 && origin.z == 0.0 {
                        }  else {
                            log::info!("{:?}", mouse_point);
                            log::info!("{:?}", pos);
                        }
                        (id,  pos, obj_area)
                    })
                    .filter(|(id, pos, obj_area)| get_bounds(&pos, &obj_area, &stage_area).contains(&mouse_point))
                    .map(|(id, pos, obj_area)| (id, pos))
                    .collect();
           
            hits.last().map(|(id, pos)| {
                *world.borrow::<UniqueViewMut<Controller>>() = Controller::Selected(*id); 
                let mut motion = world.borrow::<UniqueViewMut<Motion>>();
                motion.last_pos = Some(mouse_point);
                motion.current_pos = None;
            });
        }
    }).forget();

    EventListener::new(canvas, "pointerup", {
        let world = Rc::clone(&world);
        move |_e:&Event| {
            *world.borrow::<UniqueViewMut<Controller>>() = Controller::Waiting; 
            let mut motion = world.borrow::<UniqueViewMut<Motion>>();
            motion.last_pos = None;
            motion.current_pos = None;
        }
    }).forget();

    EventListener::new(canvas, "pointermove", {
        let world = Rc::clone(&world);
        move |event:&Event| {
            if let Controller::Selected(id) = *world.borrow::<UniqueViewMut< Controller>>() {
                let stage_area = world.borrow::<UniqueViewMut< StageArea>>();
                let event = event.dyn_ref::<web_sys::MouseEvent>().unwrap_throw();
                let mouse_point = get_point(&stage_area, &event);
                let mut motion = world.borrow::<UniqueViewMut< Motion>>();
                if let Some(ref last_pos) = motion.current_pos {
                    let delta_x = mouse_point.x - last_pos.x;
                    let delta_y = mouse_point.y - last_pos.y;
                    
                    let mut positions = world.borrow::<ViewMut<Translation>>();
                    let mut position = (&mut positions).try_get(id).unwrap();
                    position.x += delta_x;
                    position.y += delta_y;
                    //log::info!("moving {:?} {} {}", id, delta_x, delta_y);
                }
                motion.last_pos = motion.current_pos.clone();
                motion.current_pos = Some(mouse_point);
            }
        }
    }).forget();
}

fn get_point(stage_area:&Area, event:&MouseEvent) -> Vec3 {
    Vec3::new(
        event.client_x() as f64, 
        ((stage_area.height as i32) - event.client_y()) as f64,
        0.0
    )
}
fn get_bounds(pos: &Vec3, obj_area: &Area, screen_area: &Area) -> Bounds {
    Bounds {
        left: pos.x,
        right: pos.x + (obj_area.width as f64),
        top: pos.y + (obj_area.height as f64),
        bottom: pos.y
    }
}