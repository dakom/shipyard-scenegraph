/*
 * The input queue is optional
 * Downstream should process and remove events each tick
 * Note that the default uses insert() for all operations
 *
 * if other orderings are desired, replace InputListener
 * in the World with a custom implementation
 */

use shipyard::*;
use std::collections::VecDeque;

pub type InputQueueViewMut<'a> = UniqueViewMut<'a, InputQueue>;


#[derive(Component, Debug)]
pub struct InputQueue(pub VecDeque<Input>);
impl InputQueue {
    pub fn new() -> Self {
        Self(VecDeque::new())
    }

    //The typical use case - will *replace* what was there
    //idea being if a user moves their mouse a bunch of times in a tick
    //we only care about the most recent position
    //if they move, click, move - we still want the click to happen _after_ the move
    //of course this isn't the only use case - others are below
    pub fn insert_replace(&mut self, input:Input) {
        let queue = &mut self.0;
        
        let entry = queue.iter_mut().find(|q_input| {
            std::mem::discriminant(*q_input) == std::mem::discriminant(&input)
        });

        if let Some(entry) = entry {
            //replace what was there
            *entry = input;
        } else {
            self.0.push_back(input);
        }
    }

    //remove what was there, and add the new one to the end
    pub fn _insert_move(&mut self, input:Input) {
        let queue = &mut self.0;
        
        queue.retain(|q_input| {
            std::mem::discriminant(q_input) != std::mem::discriminant(&input)
        });

        self.0.push_back(input);
    }

    //careful - this can create long lists!
    pub fn insert_always(&mut self, input:Input) {
        self.0.push_back(input);
    }
}

//Delta is the change since the last move 
//Diff is the change since pointer down
type X = i32;
type Y = i32;
type DeltaX = i32;
type DeltaY = i32;
type DiffX = i32;
type DiffY = i32;
type WheelX = f64;
type WheelY = f64;
type WheelZ = f64;

#[derive(Debug, Clone)]
pub enum Input {
    PointerDown(X, Y),
    PointerDrag(X, Y, DeltaX, DeltaY, DiffX, DiffY),
    PointerHover(X, Y),
    PointerUp(X, Y, DeltaX, DeltaY, DiffX, DiffY),
    PointerClick(X, Y),
    KeyDown(String),
    KeyUp(String),
    Wheel(WheelDeltaMode, WheelX, WheelY, WheelZ),
}


#[derive(Debug, Clone, Copy)]
pub enum WheelDeltaMode {
    Pixel,
    Line,
    Page
}

impl std::convert::TryFrom<u32> for WheelDeltaMode {
    type Error = &'static str;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Pixel),
            1 => Ok(Self::Line),
            2 => Ok(Self::Page),
            _ => Err("unknown wheel delta mode!")
        }
    }
}
