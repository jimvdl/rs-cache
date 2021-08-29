//! Store trait for cache interchangeability.

use std::io::Write;

mod memory_store;
mod file_store;

pub use memory_store::*;
pub use file_store::*;

use std::fs::File;

use crate::arc::ArchiveRef;

/// The internal storage for a cache with a way to read internal data.
pub trait Store: Sized {
    fn new(main_file: File) -> crate::Result<Self>;
    fn read(&self, archive: &ArchiveRef) -> crate::Result<Vec<u8>>;
}

pub trait ReadIntoWriter {
    fn read_into_writer<W: Write>(&self, archive: &ArchiveRef, writer: &mut W) 
        -> crate::Result<()>;
}