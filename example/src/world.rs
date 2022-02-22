use shipyard::*;
use shipyard_scenegraph::init::init_scenegraph;
use crate::mainloop::UpdateTick;
use crate::renderer::{SceneRenderer, item::*};
use shipyard_scenegraph::prelude::*;
use crate::controller::{queue::InputQueue, Controller, controller_set_sys, controller_process_sys, controller_clear_sys};
use crate::renderer::render_sys;
use crate::physics::spin_sys;
use std::collections::HashMap;
use nalgebra::{Vector3, Matrix4, UnitQuaternion};

pub fn init_world(renderer:SceneRenderer) -> World {
    let world = World::new();

    world.add_unique(Controller::Waiting);
    world.add_unique(InputQueue::new());
    world.add_unique(UpdateTick::default());
    world.add_unique(InteractableLookup(HashMap::new()));
    world.add_unique_non_send_sync(renderer);

    register_workloads(&world);

    init_scenegraph::<Vector3<f64>, UnitQuaternion<f64>, Matrix4<f64>, f64>(&world);

    world
}

pub const RENDER:&str = "RENDER";
pub const CONTROLLER:&str = "CONTROLLER";
pub const PHYSICS:&str = "PHYSICS";
pub const CLEANUP:&str = "CLEANUP";

pub fn register_workloads(world:&World) {

    Workload::builder(RENDER)
        .with_system(local_transform_sys)
        .with_system(world_transform_sys)
        .with_system(render_sys)
        .add_to_world(world)
        .unwrap();

    Workload::builder(CONTROLLER)
        .with_system(controller_set_sys)
        .with_system(controller_process_sys)
        .add_to_world(world)
        .unwrap();

    Workload::builder(PHYSICS)
        .with_system(spin_sys)
        .add_to_world(world)
        .unwrap();

    Workload::builder(CLEANUP)
        .with_system(controller_clear_sys)
        .add_to_world(world)
        .unwrap();
}

