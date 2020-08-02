pub mod osrs;

use std::io;

pub trait Definition: Sized {
    fn new(id: u16, buffer: &[u8]) -> io::Result<Self>;
}