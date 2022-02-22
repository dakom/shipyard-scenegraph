use std::sync::atomic::{AtomicBool, AtomicI32};

pub struct InputState {
    pub is_pointer_down: AtomicBool,
    pub first_pointer_move_x: AtomicI32,
    pub first_pointer_move_y: AtomicI32,
    pub last_pointer_move_x: AtomicI32,
    pub last_pointer_move_y: AtomicI32,
}

impl InputState {
    pub fn new() -> Self {
        Self {
            is_pointer_down: AtomicBool::new(false),
            first_pointer_move_x: AtomicI32::new(0),
            first_pointer_move_y: AtomicI32::new(0),
            last_pointer_move_x: AtomicI32::new(0),
            last_pointer_move_y: AtomicI32::new(0),
        }
    }
}
