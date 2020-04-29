use std::cell::RefCell;
use std::fs;
use std::io::{self, Read, Seek, SeekFrom};

pub enum Input<'a> {
    File(fs::File),
    Stdin(RefCell<io::StdinLock<'a>>),
}

impl<'a> Read for Input<'a> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        match *self {
            Input::File(ref mut file) => file.read(buf),
            Input::Stdin(ref mut stdin) => stdin.borrow_mut().read(buf),
        }
    }
}

impl<'a> Seek for Input<'a> {
    fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
        match *self {
            Input::File(ref mut file) => file.seek(pos),
            Input::Stdin(_) => Err(io::Error::new(
                io::ErrorKind::Other,
                "not supported by stdin-input",
            )),
        }
    }
}

impl<'a> Input<'a> {
    pub fn into_inner(self) -> Box<dyn Read + 'a> {
        match self {
            Self::File(file) => Box::new(file),
            Self::Stdin(stdin) => Box::new(stdin.into_inner()),
        }
    }
}
