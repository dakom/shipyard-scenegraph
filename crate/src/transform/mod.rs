use cfg_if::cfg_if;
cfg_if! {
    if #[cfg(not(feature = "nalgebra_transforms"))] {
        mod native_math;
        pub use self::native_math::*;
    }
}