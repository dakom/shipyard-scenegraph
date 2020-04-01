use shipyard::prelude::*;
use shipyard_scenegraph::{self as sg, *};
use rand::prelude::*;
use crate::components::*;
use crate::geometry::*;
use crate::config::*;

pub const TICK:&'static str = "TICK";

pub fn register_workloads(world:&World) {
    world.add_workload::<(sg::systems::TrsToLocal, sg::systems::LocalToWorld, Render), _>(TICK); 
}

#[system(Render)]
pub fn run (
    mut renderer: Unique<NonSendSync<&mut SceneRenderer>>,
    world_transforms: &WorldTransform, 
    stage_area:Unique<&StageArea>, 
    img_areas:&ImageArea,
    colors:&Color,
) {
    renderer.pre_render(&stage_area).unwrap();

    let mut scratch:[f32;16] = [0.0;16];

    (&world_transforms, &img_areas, &colors)
        .iter()
        .for_each(|(transform, img_area, color)| {
            transform.write_to_vf32(&mut scratch);
            renderer.draw_square(&scratch, &img_area.0, color).unwrap();
        });

}