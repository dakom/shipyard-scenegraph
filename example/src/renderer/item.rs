use std::collections::HashMap;
use shipyard::*;
use derive_deref::{Deref, DerefMut};

#[derive(Component, Clone)]
pub struct Color (pub f64, pub f64, pub f64, pub f64); 
impl Color {
    pub fn get_shader_tuple(&self) -> (f32, f32, f32, f32) {
        (self.0 as f32, self.1 as f32, self.2 as f32, self.3 as f32)
    }
}


pub type InteractableLookupView<'a> = UniqueView<'a, InteractableLookup>;
pub type InteractableLookupViewMut<'a> = UniqueViewMut<'a, InteractableLookup>;
#[derive(Component, Deref, DerefMut)]
pub struct InteractableLookup(pub HashMap<u32, EntityId>);

#[derive(Component, Clone)]
pub struct ImageArea {
    pub width: u32, 
    pub height: u32, 
}
impl ImageArea {
    pub fn get_shader_tuple(&self) -> (f32, f32) {
        (self.width as f32, self.height as f32)
    }
}


#[derive(Component)]
pub struct Interactable(pub u32); // the entity id lookup

pub const QUAD_GEOM_UNIT: [f32; 8] = [
    0.0, 1.0, // top-left
    0.0, 0.0, //bottom-left
    1.0, 1.0, // top-right
    1.0, 0.0, // bottom-right
];
