use std::io::{self, Write};

use termion::input::{Keys, TermRead};
use termion::raw::{IntoRawMode, RawTerminal};
use termion::AsyncReader;
use termishade::RenderTarget;

pub use termion::event::Key;

pub struct TermionTarget {
    width: usize,
    height: usize,
    input: Option<Keys<AsyncReader>>,
    raw: Option<RawTerminal<io::Stdout>>,
}

impl TermionTarget {
    pub fn new() -> std::io::Result<Self> {
        let (w, h) = termion::terminal_size()?;

        Ok(Self {
            width: w as usize,
            height: h as usize,
            input: Some(termion::async_stdin().keys()),
            raw: Some(io::stdout().into_raw_mode()?),
        })
    }

    pub fn new_without_io(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            input: None,
            raw: None,
        }
    }

    pub fn get_key(&mut self) -> Option<Key> {
        self.input.as_mut()?.next().and_then(Result::ok)
    }
    
    pub fn draw_to_string(&self, buffer: &[nalgebra::Vector4<f32>]) -> String {
        let mut cmd = format!("{}", termion::cursor::Goto(1, 1));

        for row in buffer.chunks(self.width).rev().step_by(2) {
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
            cmd += "\n";
        }

        format!("{}{}{}", termion::cursor::Hide, cmd, termion::cursor::Show)
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
