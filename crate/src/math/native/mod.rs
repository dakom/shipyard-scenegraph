/*
The math was taken and adapted from various places on the internet
Specifically, from gl-matrix and the gltf-rs crate (which in turn took from cg_math)

The idea is that we have a very minimal math lib with no dependencies for small projects
Currently there's a ton of stuff missing
*/

mod vec3;
mod vec4;
mod matrix4;
mod aliases;

pub use aliases::*;
pub use vec3::*;
pub use vec4::*;
pub use matrix4::*;
