// #![deny(missing_docs)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![deny(
    clippy::all,
    clippy::correctness,
    clippy::suspicious,
    clippy::style,
    clippy::complexity,
    clippy::perf
)]

mod archive;
pub mod codec;
pub mod error;
mod index;
pub mod parse;
mod sector;
pub mod xtea;

#[doc(inline)]
pub use error::Error;
use error::Result;

pub const MAIN_DATA: &str = "main_file_cache.dat2";
pub const REFERENCE_TABLE: &str = "main_file_cache.idx255";
pub const REFERENCE_TABLE_ID: u8 = 255;

pub use archive::*;
pub use index::*;
pub use sector::*;

use crate::codec::{Buffer, Encoded};
use error::ParseError;
use memmap2::Mmap;
use std::fs::File;
use std::io::Write;
use std::path::Path;

/// A virtual file type for the `.dat2` file.
#[derive(Debug)]
pub struct Dat2(Mmap);

impl Dat2 {
    /// Initializes a memory map over the specified `.dat2` file.
    pub fn new<P: AsRef<Path>>(path: P) -> crate::Result<Self> {
        Ok(Self(unsafe { Mmap::map(&File::open(path.as_ref())?)? }))
    }

    /// Read all the data that belongs to the `ArchiveRef`.
    pub fn read(&self, archive: &ArchiveRef) -> crate::Result<Buffer<Encoded>> {
        let mut buffer = Buffer::from(Vec::with_capacity(archive.length));
        self.read_into_writer(archive, &mut buffer)?;

        assert_eq!(buffer.len(), archive.length);

        Ok(buffer)
    }

    /// Read all the data that belongs to the `ArchiveRef` into the given writer.
    pub fn read_into_writer<W>(&self, archive: &ArchiveRef, writer: &mut W) -> crate::Result<()>
    where
        W: Write,
    {
        let mut current = archive.sector;
        let header_size = SectorHeaderSize::from(archive);

        for (chunk, data_len) in archive.data_blocks().enumerate() {
            let offset = current * SECTOR_SIZE;

            let data_block = &self.0[offset..offset + data_len];
            match Sector::new(data_block, &header_size) {
                Ok(sector) => {
                    sector
                        .header
                        .validate(archive.id, chunk, archive.index_id)?;
                    current = sector.header.next;
                    writer.write_all(sector.data_block)?;
                }
                Err(_) => return Err(ParseError::Sector(archive.sector).into()),
            };
        }

        Ok(())
    }
}

#[cfg(test)]
fn is_normal<T: Send + Sync + Sized + Unpin>() {}
#[test]
fn normal_types() {
    is_normal::<Dat2>();
}
