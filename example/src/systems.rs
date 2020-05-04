use shipyard::*;
use shipyard_scenegraph::{self as sg, *};
use nalgebra::{Vector3, Quaternion, Unit, UnitQuaternion};
use crate::components::*;
use crate::geometry::*;
use crate::config::*;

pub const TICK:&'static str = "TICK";

pub fn register_workloads(world:&World) {
    world
        .add_workload(TICK)
        .with_system(system!(spin))
        .with_system(system!(sg::systems::trs_to_local))
        .with_system(system!(sg::systems::local_to_world))
        .with_system(system!(render))
        .build();
}

pub fn spin(
    tick: UniqueView<Tick>,
    mut translations: ViewMut<Translation>, 
    mut rotations: ViewMut<Rotation>, 
    mut spins: ViewMut<Spin>, 
    world_transforms: View<WorldTransform>, 
    stage_area:UniqueView<StageArea>, 
    img_areas:View<ImageArea>,
) {
    let Tick {delta, ..} = *tick;

    (&mut spins, &translations, &mut rotations, &img_areas, &world_transforms)
        .iter()
        .for_each(|(spin, transform, rotation, img_area, world_transform)| {
            let mut value = spin.0 + (delta * 0.1);

            if tick.total < 10000.0 {
                //log::info!("{:?}", world_transform);
            }
            
            if value > 360.0 {
                value = value - 360.0;
            }

            spin.0 = value;
            let axis = Unit::new_normalize(Vector3::new(0.0, 0.0, 1.0));
            let coords = UnitQuaternion::from_axis_angle(&axis, value.to_radians()).coords;
        
            //let coords = UnitQuaternion::new_unchecked(Quaternion::new(0.9, 0.0, 0.0, 0.4)).coords;

            cfg_if::cfg_if! {
                if #[cfg(feature = "nalgebra_transforms")] {
                    let quat = rotation.0.as_mut_unchecked();
                    quat.coords = coords;
                } else {
                    let quat = &mut rotation.0;
                    quat.x = coords.x;
                    quat.y = coords.y;
                    quat.z = coords.z;
                    quat.w = coords.w;
                }
            }
        });

}

pub fn render(
    mut renderer: NonSendSync<UniqueViewMut<SceneRenderer>>,
    world_transforms: View<WorldTransform>, 
    stage_area:UniqueView<StageArea>, 
    img_areas:View<ImageArea>,
    colors:View<Color>,
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