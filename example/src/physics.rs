use shipyard::*;
use derive_deref::{Deref, DerefMut};
use shipyard_scenegraph::prelude::*;
use crate::mainloop::UpdateTick;
use nalgebra::{Unit, UnitQuaternion, Vector3};

#[derive(Component, Clone, Deref, DerefMut)]
pub struct Spin(pub f64);

pub fn spin_sys(
    tick: UniqueView<UpdateTick>,
    mut rotations: ViewMut<Rotation>, 
    mut spins: ViewMut<Spin>, 
) {
    let UpdateTick {delta} = *tick;

    (&mut spins, &mut rotations)
        .iter()
        .for_each(|(spin, mut rotation)| {
            let mut value = spin.0 + (delta * 0.1);

            if value > 360.0 {
                value -= 360.0;
            }

            spin.0 = value;
            let axis = Unit::new_normalize(Vector3::new(0.0, 0.0, 1.0));
            let coords = UnitQuaternion::from_axis_angle(&axis, value.to_radians()).coords;
        
            rotation.coords = coords;
        });

}
