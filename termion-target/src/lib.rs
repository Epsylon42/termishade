use std::io::{self, Write};

use termion::input::{Keys, TermRead};
use termion::raw::{IntoRawMode, RawTerminal};
use termion::AsyncReader;
use termishade::RenderTarget;

pub use termion::event::Key;

pub struct TermionTarget {
    width: usize,
    height: usize,
    reduced_palette: bool,
    input: Option<Keys<AsyncReader>>,
    raw: Option<RawTerminal<io::Stdout>>,
}

impl TermionTarget {
    pub fn new() -> std::io::Result<Self> {
        let (w, h) = termion::terminal_size()?;

        Ok(Self {
            width: w as usize,
            height: h as usize,
            reduced_palette: false,
            input: Some(termion::async_stdin().keys()),
            raw: Some(io::stdout().into_raw_mode()?),
        })
    }

    pub fn new_without_io(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            reduced_palette: false,
            input: None,
            raw: None,
        }
    }

    pub fn reduced_palette(mut self, reduced: bool) -> Self {
        self.reduced_palette = reduced;
        self
    }

    pub fn get_key(&mut self) -> Option<Key> {
        self.input.as_mut()?.next().and_then(Result::ok)
    }
    
    pub fn draw_to_string(&self, buffer: &[nalgebra::Vector4<f32>]) -> String {
        let mut cmd = String::new();
        let mut prev_color = None;

        for (row_num, row) in buffer.chunks(self.width).rev().step_by(2).enumerate() {
            cmd += &format!("{}", termion::cursor::Goto(1, row_num as u16 + 1));
            for pixel in row {
                let pixel = pixel.map(|a| a.max(0.0).min(1.0));

                if self.reduced_palette {
                    let u8pixel = pixel.map(|a| (a * 5.0) as u8);
                    if prev_color == Some(u8pixel) {
                        cmd.push(' ');
                        continue;
                    }
                    prev_color = Some(u8pixel);

                    cmd += &format!(
                        "{} ",
                        termion::color::Bg(termion::color::AnsiValue::rgb(
                            u8pixel.x,
                            u8pixel.y,
                            u8pixel.z
                        ))
                    );
                } else {
                    let u8pixel = pixel.map(|a| (a * 255.0) as u8);
                    if prev_color == Some(u8pixel) {
                        cmd.push(' ');
                        continue;
                    }
                    prev_color = Some(u8pixel);

                    cmd += &format!(
                        "{} ",
                        termion::color::Bg(termion::color::Rgb(
                            u8pixel.x,
                            u8pixel.y,
                            u8pixel.z
                        ))
                    );
                }
            }
        }

        cmd
    }
}

impl RenderTarget<nalgebra::Vector4<f32>> for TermionTarget {
    fn size(&self) -> [usize; 2] {
        [self.width, self.height * 2]
    }

    fn draw(&mut self, buffer: &[nalgebra::Vector4<f32>]) {
        if self.raw.is_some() {
            let s = self.draw_to_string(buffer);
            write!(self.raw.as_mut().unwrap(), "{}", s).unwrap();
        }
    }
}
