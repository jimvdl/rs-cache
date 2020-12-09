//! Store trait for cache interchangeability.

mod memory_store;
mod file_store;

pub use memory_store::*;
pub use file_store::*;

use std::fs::File;

use crate::arc::Archive;

/// The internal storage for a cache with a way to read internal data.
pub trait Store: Sized {
    fn new(main_file: File) -> crate::Result<Self>;
    fn read(&self, archive: &Archive) -> crate::Result<Vec<u8>>;
}