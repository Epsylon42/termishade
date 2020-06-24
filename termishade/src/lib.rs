#[macro_use]
extern crate itertools;
extern crate nalgebra as na;

pub mod base_renderer;
pub mod blend;
pub mod color_depth_renderer;
pub mod interpolate;
#[cfg(feature = "na-renderer")]
pub mod nalgebra_renderer;
pub mod program;
pub mod rasterizer;
pub mod target;
pub mod util;

/// nalgebra extensions
pub mod next;

pub use base_renderer::BaseRenderer;
pub use blend::Blender;
pub use color_depth_renderer::ColorDepthRenderer;
pub use interpolate::Interpolate3;
#[cfg(feature = "na-renderer")]
pub use nalgebra_renderer::*;
pub use program::Program;
pub use rasterizer::Rasterizer;
pub use target::RenderTarget;
