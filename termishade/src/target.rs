use crate::util::flatten_coord;

use alga::linear::VectorSpace;

pub trait RenderTarget<Color: 'static> {
    fn size(&self) -> [usize; 2];
    fn draw(&mut self, data: &[Color]);

    fn size_multisampled(&self, level: u8) -> [usize; 2] {
        let [w, h] = self.size();
        [w * level as usize, h * level as usize]
    }

    fn draw_multisampled(&mut self, data: &[Color], level: u8)
        where Color: VectorSpace + for<'a> std::iter::Sum<&'a Color>,
              Color::Field: From<f32>
    {
        let [width, height] = self.size();
        let [mwidth, mheight] = self.size_multisampled(level);
        assert_eq!(data.len(), mwidth * mheight);

        let level = level as usize;

        let buf = iproduct!(0..height, 0..width)
            .map(|(y, x)| {
                iproduct!(0..level, 0..level)
                    .map(|(dx, dy)| &data[flatten_coord([mwidth, mheight], [x * level + dx, y * level + dy])])
                    .sum::<Color>() * Color::Field::from((level as f32).powi(2).recip())
            })
            .collect::<Vec<_>>();

        self.draw(&buf);
    }
}
