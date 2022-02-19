use shipyard_scenegraph::prelude::*;
#[derive(Debug, Copy, Clone)]
pub struct Area {
    pub width: u32,
    pub height: u32 
}
impl Area {
    pub fn get_tuple(&self) -> (f32, f32) {
        (self.width as f32, self.height as f32)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Bounds {
    pub left: f64,
    pub right: f64,
    pub top: f64,
    pub bottom: f64
}

impl Bounds {
    pub fn contains(&self, point:&Vec3) -> bool {
        self.left <= point.x()
            && self.right >= point.x()
            && self.bottom <= point.y()
            && self.top >= point.y()
    }
}

pub const QUAD_GEOM_UNIT: [f32; 8] = [
    0.0, 1.0, // top-left
    0.0, 0.0, //bottom-left
    1.0, 1.0, // top-right
    1.0, 0.0, // bottom-right
];
