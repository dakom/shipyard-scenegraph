use shipyard::*;
use shipyard_scenegraph::prelude::*;
use derive_deref::{Deref, DerefMut};
use crate::geometry::*;
//re-exported so its easier to just use components::*
pub use crate::renderer::SceneRenderer;

#[derive(Component, Clone)]
pub struct Color (pub f64, pub f64, pub f64, pub f64); 
impl Color {
    pub fn get_tuple(&self) -> (f32, f32, f32, f32) {
        (self.0 as f32, self.1 as f32, self.2 as f32, self.3 as f32)
    }
}

#[derive(Component, Clone, Deref, DerefMut)]
pub struct ImageArea(pub Area);

#[derive(Component, Clone, Deref, DerefMut)]
pub struct StageArea(pub Area);

#[derive(Component)]
pub struct Tick {
    pub last_time:f64,
    pub now:f64,
    pub delta:f64,
    pub total: f64,
}
impl Default for Tick {
    fn default() -> Self {
        Self {
            last_time: 0.0,
            now: 0.0,
            delta: 0.0,
            total: 0.0,
        }
    }
}


#[derive(Component, Clone, Deref, DerefMut)]
pub struct Spin(pub f64);

#[derive(Component)]
pub struct Interactable {}

#[derive(Component, PartialEq)]
pub enum Controller {
    Waiting,
    Selected(EntityId),
}

#[derive(Component)]
pub struct Motion {
    pub last_pos: Option<Vec3>,
    pub current_pos: Option<Vec3>,
}
