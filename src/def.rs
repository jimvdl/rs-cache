//! Definitions for certain games.

// All OSRS definitions
pub mod osrs;

use std::io;

/// Marker trait for definitions.
pub trait Definition: Sized {
    fn new(id: u16, buffer: &[u8]) -> io::Result<Self>;
}