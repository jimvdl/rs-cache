mod archive;
pub mod codec;
pub mod error;
mod index;
pub mod parse;
mod sector;
mod xtea;

use error::Result;

pub const MAIN_DATA: &str = "main_file_cache.dat2";
pub const REFERENCE_TABLE: u8 = 255;

pub use archive::*;
pub use index::*;
pub use sector::*;

use error::ParseError;
use memmap2::Mmap;
use std::fs::File;
use std::io::Write;
use std::path::Path;

#[derive(Debug)]
pub struct Dat2(Mmap);

impl Dat2 {
    pub fn new<P: AsRef<Path>>(path: P) -> crate::Result<Self> {
        Ok(Self(unsafe { Mmap::map(&File::open(path.as_ref())?)? }))
    }

    pub fn read<W>(&self, archive: &ArchiveRef, writer: &mut W) -> crate::Result<()>
    where
        W: Write,
    {
        let mut current_sector = archive.sector;
        let (header_size, chunks) = archive.chunks();

        for (chunk, data_len) in chunks.enumerate() {
            let offset = current_sector * SECTOR_SIZE;

            let data_block = &self.0[offset..offset + data_len];
            match Sector::new(data_block, &header_size) {
                Ok(sector) => {
                    sector
                        .header
                        .validate(archive.id, chunk, archive.index_id)?;
                    current_sector = sector.header.next;
                    writer.write_all(sector.data_block)?;
                }
                Err(_) => return Err(ParseError::Sector(archive.sector).into()),
            };
        }

        Ok(())
    }
}
