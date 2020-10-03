#[derive(Debug, Clone, Copy)]
pub struct Termios;

pub mod size {
    use std::io;

    pub fn terminal_size() -> io::Result<(u16, u16)> {
        Err(io::Error::new(io::ErrorKind::Other, "Not supported"))
    }
}

pub mod tty {
    use std::{fs, io};

    pub fn is_tty<T>(_: &T) -> bool {
        false
    }

    pub fn get_tty() -> io::Result<fs::File> {
        Err(io::Error::new(io::ErrorKind::Other, "Not supported"))
    }
}

pub mod attr {
    use std::io;
    use super::Termios;

    pub fn get_terminal_attr() -> io::Result<Termios> {
        Ok(Termios)
    }

    pub fn set_terminal_attr(_: &Termios) -> io::Result<()> {
        Ok(())
    }

    pub fn raw_terminal_attr(_: &mut Termios) {}
}
