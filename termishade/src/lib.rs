#[macro_use]
extern crate itertools;
extern crate nalgebra as na;

pub mod interpolate;
pub mod program;
pub mod renderer;
pub mod rasterizer;
pub mod blend;
pub mod util;
pub mod target;

pub use interpolate::Interpolate3;
pub use program::Program;
pub use renderer::Renderer;
pub use rasterizer::Rasterizer;
pub use blend::Blender;
pub use target::RenderTarget;
