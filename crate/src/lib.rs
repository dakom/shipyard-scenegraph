//re-export
pub use shipyard_hierarchy::*;

pub mod slice;
pub mod components;
pub mod views;
pub mod hierarchy;
pub mod transforms;
pub mod math;
pub mod systems;
pub mod init;


pub mod prelude {
    cfg_if::cfg_if! {
        if #[cfg(feature = "nalgebra_math")] {
            pub use crate::math::nalgebra::*;
        } else if #[cfg(feature = "native_math")] {
            pub use crate::math::native::*;
        }
    }

    pub use crate::hierarchy::SceneGraph;
}
