use shipyard::*;
use shipyard_scenegraph::prelude::*;
use nalgebra::{Vector3, Unit, UnitQuaternion};
use crate::components::*;

pub const TICK:&str = "TICK";

pub fn register_workloads(world:&World) {

    Workload::builder(TICK)
        .with_system(spin)
        .with_system(local_transform_sys)
        .with_system(world_transform_sys)
        .with_system(render)
        .add_to_world(world)
        .unwrap();
}

pub fn spin(
    tick: UniqueView<Tick>,
    mut rotations: ViewMut<Rotation>, 
    mut spins: ViewMut<Spin>, 
) {
    let Tick {delta, ..} = *tick;

    (&mut spins, &mut rotations)
        .iter()
        .for_each(|(mut spin, mut rotation)| {
            let mut value = spin.0 + (delta * 0.1);

            if tick.total < 10000.0 {
                //log::info!("{:?}", world_transform);
            }
            
            if value > 360.0 {
                value -= 360.0;
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
                    let quat = &mut rotation;
                    quat.set_x(coords.x);
                    quat.set_y(coords.y);
                    quat.set_z(coords.z);
                    quat.set_w(coords.w);
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
