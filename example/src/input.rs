use crate::components::*;
use crate::geometry::*;
use shipyard_scenegraph::{Vec3, Translation};
use std::rc::{Rc};
use gloo_events::{EventListener};
use web_sys::{Event, MouseEvent};
use shipyard::prelude::*;
use web_sys::{HtmlCanvasElement};
use wasm_bindgen::{JsCast, UnwrapThrowExt};

pub fn start(world:Rc<World>, canvas:&HtmlCanvasElement) {
    EventListener::new(canvas, "pointerdown", {
        let world = Rc::clone(&world);
        move |event:&Event| {
            let stage_area = world.borrow::<Unique<&mut StageArea>>();
            let stage_area = stage_area.0;
            let event = event.dyn_ref::<web_sys::MouseEvent>().unwrap_throw();
            let mouse_point = get_point(&stage_area, &event);

            let (positions, areas) = world.borrow::<(&Translation, &ImageArea)>();

            let hits:Vec<(EntityId, Vec3)> = (&positions, &areas)
                .iter()
                .with_id()
                .filter(|(id, (pos, obj_area))| get_bounds(&pos.0, &obj_area.0, &stage_area).contains(&mouse_point))
                .map(|(id, (pos, obj_area))| (id, pos.0.clone()))
                .collect();
           
            hits.last().map(|(id, pos)| {
                *world.borrow::<Unique<&mut Controller>>() = Controller::Selected(*id); 
                let mut motion = world.borrow::<Unique<&mut Motion>>();
                motion.last_pos = Some(mouse_point);
                motion.current_pos = None;
            });
        }
    }).forget();

    EventListener::new(canvas, "pointerup", {
        let world = Rc::clone(&world);
        move |_e:&Event| {
            *world.borrow::<Unique<&mut Controller>>() = Controller::Waiting; 
            let mut motion = world.borrow::<Unique<&mut Motion>>();
            motion.last_pos = None;
            motion.current_pos = None;
        }
    }).forget();

    EventListener::new(canvas, "pointermove", {
        let world = Rc::clone(&world);
        move |event:&Event| {
            if let Controller::Selected(id) = *world.borrow::<Unique<&mut Controller>>() {
                let stage_area = world.borrow::<Unique<&mut StageArea>>();
                let stage_area = stage_area.0;
                let event = event.dyn_ref::<web_sys::MouseEvent>().unwrap_throw();
                let mouse_point = get_point(&stage_area, &event);
                let mut motion = world.borrow::<Unique<&mut Motion>>();
                if let Some(ref last_pos) = motion.current_pos {
                    let delta_x = mouse_point.x - last_pos.x;
                    let delta_y = mouse_point.y - last_pos.y;
                    
                    let mut positions = world.borrow::<&mut Translation>();
                    let mut position = (&mut positions).get(id).unwrap();
                    position.0.x += delta_x;
                    position.0.y += delta_y;
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