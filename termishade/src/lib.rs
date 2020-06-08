extern crate nalgebra as na;

pub mod interpolate;
pub mod program;
pub mod renderer;
pub mod rasterizer;

pub use interpolate::Interpolate3;
pub use program::Program;
pub use renderer::Renderer;
pub use rasterizer::Rasterizer;
