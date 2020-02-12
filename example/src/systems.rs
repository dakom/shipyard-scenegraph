use shipyard::prelude::*;
use shipyard_scenegraph::{WorldTransform, TransformValues, systems as sg_systems};
use rand::prelude::*;
use crate::components::*;
use crate::geometry::*;
use crate::config::*;

pub const TICK:&'static str = "TICK";

pub fn register_workloads(world:&World) {
    world.add_workload::<(sg_systems::TrsToLocal, sg_systems::LocalToWorld, Render), _>(TICK); 
}

#[system(Render)]
pub fn run (
    mut renderer: Unique<NonSendSync<&mut SceneRenderer>>,
    world_transforms: &WorldTransform, 
    stage_area:Unique<&StageArea>, 
    img_areas:&ImageArea,
    colors:&Color,
) {
    renderer.pre_render(&stage_area.0).unwrap();

    (&world_transforms, &colors)
        .iter()
        .for_each(|(transform, color)| {
            renderer.draw_square(&transform.0.to_vec_f32(), color).unwrap();
    });
}