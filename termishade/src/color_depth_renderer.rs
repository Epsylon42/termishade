use crate::base_renderer::BaseRenderer;

pub struct ColorDepthRenderer {
    width: usize,
    height: usize,
    color: Vec<na::Vector4<f32>>,
    depth: Vec<f32>,
}

impl ColorDepthRenderer {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            color: vec![na::Vector4::zeros(); width * height],
            depth: vec![0.0; width * height],
        }
    }
}

impl BaseRenderer for ColorDepthRenderer {
    type Color = na::Vector4<f32>;

    fn size(&self) -> [usize; 2] {
        [self.width, self.height]
    }

    fn color_buffer(&mut self) -> &mut [Self::Color] {
        &mut self.color
    }

    fn depth_buffer(&mut self) -> &mut [f32] {
        &mut self.depth
    }
}
