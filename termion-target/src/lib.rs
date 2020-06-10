use termishade::RenderTarget;

pub struct TermionTarget {
    width: usize,
    height: usize,
}

impl TermionTarget {
    pub fn new() -> std::io::Result<Self> {
        let (w, h) = termion::terminal_size()?;

        Ok(Self {
            width: w as usize,
            height: h as usize,
        })
    }
}

impl RenderTarget<nalgebra::Vector4<f32>> for TermionTarget {
    fn size(&self) -> [usize; 2] {
        [self.width, self.height*2]
    }

    fn draw(&mut self, buffer: &[nalgebra::Vector4<f32>]) {
        let mut cmd = format!("{}", termion::cursor::Goto(1, 1));

        let row_iter = buffer.chunks(self.width).rev().step_by(2)
            .zip(buffer.chunks(self.width).rev().skip(1).step_by(2));

        for (row1, row2) in row_iter {
            for (pixel1, pixel2) in row1.into_iter().zip(row2) {
                let pixel = ((pixel1) * 1.0).map(|a| a.max(0.0).min(1.0)) * 255.0;
                cmd += &format!(
                    "{} ",
                    termion::color::Bg(termion::color::Rgb(
                        pixel.x as u8,
                        pixel.y as u8,
                        pixel.z as u8
                    ))
                );
            }
        }

        print!(
            "{}{}{}",
            termion::cursor::Hide,
            cmd,
            termion::cursor::Show
        );
    }
}
