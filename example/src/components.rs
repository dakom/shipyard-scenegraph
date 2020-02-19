use shipyard::prelude::*;
use shipyard_scenegraph::Vec3;

use crate::geometry::*;
//re-exported so its easier to just use components::*
pub use crate::renderer::SceneRenderer;

#[derive(Clone)]
pub struct Color (pub f64, pub f64, pub f64, pub f64); 
impl Color {
    pub fn get_tuple(&self) -> (f32, f32, f32, f32) {
        (self.0 as f32, self.1 as f32, self.2 as f32, self.3 as f32)
    }
}

pub struct ImageArea(pub Area);
pub struct StageArea(pub Area);
#[derive(PartialEq)]
pub enum Controller {
    Waiting,
    Selected(EntityId),
}

pub struct Motion {
    pub last_pos: Option<Vec3>,
    pub current_pos: Option<Vec3>,
}