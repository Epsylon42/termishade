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
        assert_eq!(data.len(), width * height * 2usize.pow(level as u32));

        let buf = iproduct!(0..width, 0..height)
            .map(|(x, y)| {
                iproduct!(0..level, 0..level)
                    .map(|(dx, dy)| &data[flatten_coord([width, height], [x + dx as usize, y + dy as usize])])
                    .sum::<Color>() * Color::Field::from((level as f32).recip())
            })
            .collect::<Vec<_>>();

        self.draw(&buf);
    }
}
