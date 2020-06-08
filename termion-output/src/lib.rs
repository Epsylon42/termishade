pub struct TermionOutput {
    width: usize,
    height: usize,
}

impl TermionOutput {
    pub fn new() -> std::io::Result<Self> {
        let (w, h) = termion::terminal_size()?;

        Ok(Self {
            width: w as usize,
            height: h as usize,
        })
    }

    pub fn size(&self) -> [usize; 2] {
        [self.width, self.height]
    }

    pub fn render(&self, buffer: &[nalgebra::Vector4<f32>]) {
        let mut cmd = format!("{}", termion::cursor::Goto(1, 1));
        for row in buffer.chunks(self.width).rev() {
            for pixel in row {
                let pixel = pixel.map(|a| a.max(0.0).min(1.0)) * 255.0;
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
