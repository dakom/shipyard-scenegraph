use shipyard::*;
use crate::world::{RENDER, CONTROLLER, PHYSICS, CLEANUP};

pub type UpdateTickViewMut<'a> = UniqueViewMut<'a, UpdateTick>;
#[derive(Component, Unique, Default)]
pub struct UpdateTick {
    pub delta:f64,
}

//callbacks
pub fn begin(world: &World, _time: f64, _delta: f64) {
    world.run_workload(CONTROLLER).unwrap();
}

pub fn update(world: &World, delta: f64) {
    *world.borrow::<UpdateTickViewMut>().unwrap() = UpdateTick {delta};
    world.run_workload(PHYSICS).unwrap();
}

pub fn draw(world: &World, _interpolation:f64) {
    world.run_workload(RENDER).unwrap();
}

pub fn end(world: &World, _fps: f64, _abort:bool) {
    world.run_workload(CLEANUP).unwrap();
}
