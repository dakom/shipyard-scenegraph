#[cfg(feature = "nalgebra_math")]
//waiting for stable: #[doc(cfg(feature = "nalgebra_math"))]
/// Only if nalgebra_math feature is enabled
pub mod nalgebra;

#[cfg(feature = "native_math")]
//waiting for stable: #[doc(cfg(feature = "native_math"))]
/// Only if native_math feature is enabled
pub mod native;
