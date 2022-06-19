pub mod listeners;
pub mod state;
pub mod helpers;
pub mod queue;

use queue::*;
use std::convert::TryInto;
use crate::renderer::RendererViewMut;
use shipyard::*;
use shipyard_scenegraph::prelude::*;
use crate::renderer::item::*;

pub type ControllerViewMut<'a> = UniqueViewMut<'a, Controller>;
#[derive(Component, Unique, PartialEq)]
pub enum Controller {
    Waiting,
    Selected(EntityId),
    Move(EntityId, i32, i32),
}

impl Controller {
    pub fn get_selected(&self) -> Option<EntityId> {
        match self {
            Self::Selected(entity) => Some(*entity),
            Self::Move(entity, _, _) => Some(*entity),
            _ => None
        }
    }

}

pub fn controller_set_sys(
    mut renderer: RendererViewMut, 
    mut input_queue: InputQueueViewMut,
    mut controller: ControllerViewMut,
    lookup: InteractableLookupView,
) {
     for input in input_queue.0.drain(..) {
        match input {
            Input::PointerDown(x, y) => {
                if let Some(index) = renderer.get_picker_index(x.try_into().unwrap(), y.try_into().unwrap()).unwrap() {
                    let entity = lookup.get(&index).unwrap();
                    *controller = Controller::Selected(*entity);
                }
                //log::info!("got pointer down: {}, {}, color: {:?}", x, y, color);
            },
            Input::PointerDrag(_x, _y, delta_x, delta_y, _diff_x, _diff_y) => {
                if let Some(entity) = controller.get_selected() {
                    match *controller {
                        Controller::Move(entity, old_delta_x, old_delta_y) => {
                            *controller = Controller::Move(entity, old_delta_x + delta_x, old_delta_y + delta_y);
                        },
                        _ => {
                            *controller = Controller::Move(entity, delta_x, delta_y);
                        }
                    }
                }
            }
            _ => {}
        }
     }
}

pub fn controller_process_sys(
    controller: ControllerViewMut,
    mut translations:ViewMut<Translation>,
) {
    if let Controller::Move(entity, x, y) = *controller {
        if let Ok(mut p) = (&mut translations).get(entity) {
            p.x += f64::from(x);
            p.y += f64::from(y);
        }
    }
}
pub fn controller_clear_sys(
    mut controller: ControllerViewMut,
) {
    *controller = match controller.get_selected() {
        Some(entity) => Controller::Selected(entity),
        _ => Controller::Waiting
    }
}

