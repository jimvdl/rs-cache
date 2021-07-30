//! Definitions for certain games.

/// All OSRS definitions
pub mod osrs;
/// All RS3 definitions
pub mod rs3;

use std::io;

/// Marker trait for definitions.
pub trait Definition: Sized {
    fn new(id: u32, buffer: &[u8]) -> io::Result<Self>;
}