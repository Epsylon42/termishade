pub trait BaseRenderer {
    type Color;

    fn size(&self) -> [usize; 2];

    fn color_buffer(&mut self) -> &mut [Self::Color];
    fn depth_buffer(&mut self) -> &mut [f32];

    fn clear_color(&mut self, color: &Self::Color)
    where
        Self::Color: Clone,
    {
        for c in self.color_buffer() {
            *c = color.clone();
        }
    }

    fn clear_depth(&mut self, depth: f32) {
        for d in self.depth_buffer() {
            *d = depth;
        }
    }
}
