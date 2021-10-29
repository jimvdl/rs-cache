use nom::{
    combinator::rest,
    number::complete::{be_u16, be_u24, be_u32, be_u8},
};
#[cfg(feature = "serde-derive")]
use serde::{Deserialize, Serialize};

use crate::{archive::ArchiveRef, error::ReadError};

pub const SECTOR_HEADER_SIZE: usize = 8;
pub const SECTOR_EXPANDED_HEADER_SIZE: usize = 10;
pub const SECTOR_DATA_SIZE: usize = 512;
pub const SECTOR_EXPANDED_DATA_SIZE: usize = 510;
pub const SECTOR_SIZE: usize = SECTOR_HEADER_SIZE + SECTOR_DATA_SIZE;

pub type HeaderSize = usize;
pub type DataSize = usize;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
#[cfg_attr(feature = "serde-derive", derive(Serialize, Deserialize))]
pub struct Sector<'a> {
    pub header: SectorHeader,
    pub data_block: &'a [u8],
}

impl<'a> Sector<'a> {
    pub fn new(buffer: &'a [u8], header_size: &SectorHeaderSize) -> crate::Result<Self> {
        let (buffer, header) = SectorHeader::new(buffer, header_size)?;
        let (_, data_block) = rest(buffer)?;

        Ok(Self { header, data_block })
    }
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
#[cfg_attr(feature = "serde-derive", derive(Serialize, Deserialize))]
pub struct SectorHeader {
    pub archive_id: u32,
    pub chunk: usize,
    pub next: usize,
    pub index_id: u8,
}

impl<'a> SectorHeader {
    pub fn new(
        buffer: &'a [u8],
        header_size: &SectorHeaderSize,
    ) -> crate::Result<(&'a [u8], Self)> {
        let (buffer, archive_id) = match header_size {
            SectorHeaderSize::Normal => {
                let (buffer, archive_id) = be_u16(buffer)?;
                (buffer, archive_id as u32)
            }
            SectorHeaderSize::Expanded => be_u32(buffer)?,
        };
        let (buffer, chunk) = be_u16(buffer)?;
        let (buffer, next) = be_u24(buffer)?;
        let (buffer, index_id) = be_u8(buffer)?;

        Ok((
            buffer,
            Self {
                archive_id,
                chunk: chunk as usize,
                next: next as usize,
                index_id,
            },
        ))
    }

    pub const fn validate(
        &self,
        archive_id: u32,
        chunk: usize,
        index_id: u8,
    ) -> Result<(), ReadError> {
        if self.archive_id != archive_id {
            return Err(ReadError::SectorArchiveMismatch(
                self.archive_id,
                archive_id,
            ));
        }

        if self.chunk != chunk {
            return Err(ReadError::SectorChunkMismatch(self.chunk, chunk));
        }

        if self.index_id != index_id {
            return Err(ReadError::SectorIndexMismatch(self.index_id, index_id));
        }

        Ok(())
    }
}

impl Default for SectorHeaderSize {
    fn default() -> Self {
        Self::Normal
    }
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[cfg_attr(feature = "serde-derive", derive(Serialize, Deserialize))]
pub enum SectorHeaderSize {
    Normal,
    Expanded,
}

impl SectorHeaderSize {
    pub fn from_archive(archive: &ArchiveRef) -> Self {
        if archive.id > std::u16::MAX.into() {
            Self::Expanded
        } else {
            Self::Normal
        }
    }
}

impl From<SectorHeaderSize> for (HeaderSize, DataSize) {
    #[inline]
    fn from(header_size: SectorHeaderSize) -> Self {
        match header_size {
            SectorHeaderSize::Normal => (SECTOR_HEADER_SIZE, SECTOR_DATA_SIZE),
            SectorHeaderSize::Expanded => (SECTOR_EXPANDED_HEADER_SIZE, SECTOR_EXPANDED_DATA_SIZE),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn header_size_normal() -> crate::Result<()> {
        let archive = ArchiveRef {
            id: u16::MAX as u32,
            index_id: 0,
            sector: 0,
            length: 0,
        };
        let header_size = SectorHeaderSize::from_archive(&archive);

        assert_eq!(header_size, SectorHeaderSize::Normal);

        Ok(())
    }

    #[test]
    fn header_size_expanded() -> crate::Result<()> {
        let archive = ArchiveRef {
            id: (u16::MAX as u32) + 1,
            index_id: 0,
            sector: 0,
            length: 0,
        };
        let header_size = SectorHeaderSize::from_archive(&archive);

        assert_eq!(header_size, SectorHeaderSize::Expanded);

        Ok(())
    }

    #[test]
    fn parse_header() -> crate::Result<()> {
        let buffer = &[0, 0, 0, 0, 0, 0, 2, 255];
        let (_, header) = SectorHeader::new(buffer, &SectorHeaderSize::Normal)?;

        assert_eq!(
            header,
            SectorHeader {
                archive_id: 0,
                chunk: 0,
                next: 2,
                index_id: 255
            }
        );

        Ok(())
    }

    #[test]
    fn header_validation() {
        let header = SectorHeader {
            archive_id: 0,
            chunk: 0,
            next: 2,
            index_id: 255,
        };

        assert_eq!(
            header.validate(1, 0, 255),
            Err(ReadError::SectorArchiveMismatch(header.archive_id, 1))
        );
        assert_eq!(
            header.validate(0, 1, 255),
            Err(ReadError::SectorChunkMismatch(header.chunk, 1))
        );
        assert_eq!(
            header.validate(0, 0, 0),
            Err(ReadError::SectorIndexMismatch(header.index_id, 0))
        );
    }
}
