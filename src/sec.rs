//! Represents linked sectors in main data file.

use nom::{
    combinator::rest,
	number::complete::{
        be_u8,
		be_u16,
		be_u24,
		be_u32,
    },
};

use crate::error::ReadError;

pub const SECTOR_HEADER_SIZE: usize = 8;
pub const SECTOR_EXPANDED_HEADER_SIZE: usize = 10;
pub const SECTOR_DATA_SIZE: usize = 512;
pub const SECTOR_EXPANDED_DATA_SIZE: usize = 510;
pub const SECTOR_SIZE: usize = SECTOR_HEADER_SIZE + SECTOR_DATA_SIZE;

/// Sector data for reading.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Sector<'a> {
	pub header: SectorHeader,
	pub data_block: &'a [u8]
}

/// Contains the sector header for reading the next sector
/// and validating the current sector.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct SectorHeader {
	pub archive_id: u32,
	pub chunk: usize,
	pub next: usize,
	pub index_id: u8
}

impl<'a> Sector<'a> {
	/// Decodes the buffer from the reference table into a `Sector`.
	#[inline]
	pub fn new(buffer: &'a [u8], expanded_header: bool) -> crate::Result<Self> {
		let (buffer, header) = SectorHeader::new(buffer, expanded_header)?;
		let (_, data_block) = rest(buffer)?;

		Ok(Self { header, data_block })
	}
}

impl SectorHeader {
	/// Decodes only the header data from the buffer leaving the data block untouched.
	/// 
	/// The expanded header should be 10 bytes instead of the usual 8 bytes.
	#[inline]
	pub fn new(buffer: &[u8], expanded_header: bool) -> crate::Result<(&[u8], Self)> {
		let (buffer, archive_id) = if expanded_header {
			be_u32(buffer)?
		} else {
			let (buffer, archive_id) = be_u16(buffer)?;
			(buffer, archive_id as u32)
		};
		
		let (buffer, chunk) = be_u16(buffer)?;
		let (buffer, next) = be_u24(buffer)?;
		let (buffer, index_id) = be_u8(buffer)?;

		Ok((buffer, Self { 
			archive_id, 
			chunk: chunk as usize, 
			next: next as usize, 
			index_id 
		}))
	}

	/// Validates the current `archive_id`, `chunk` and `index_id` against the expected
	/// values from this header struct.
	#[inline]
	pub const fn validate(&self, archive_id: u32, chunk: usize, index_id: u8) -> Result<(), ReadError> {
		if self.archive_id != archive_id {
			return Err(ReadError::SectorArchiveMismatch(self.archive_id, archive_id))
		}

		if self.chunk != chunk {
			return Err(ReadError::SectorChunkMismatch(self.chunk, chunk))
		}

		if self.index_id != index_id {
			return Err(ReadError::SectorIndexMismatch(self.index_id, index_id))
		}

		Ok(())
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_parse_header() -> crate::Result<()> {
		let buffer = &[0, 0, 0, 0, 0, 0, 2, 255];

		let expected = SectorHeader { archive_id: 0, chunk: 0, next: 2, index_id: 255 };
		let (_, actual) = SectorHeader::new(buffer, false)?;

		assert_eq!(actual, expected);

		Ok(())
	}

	#[test]
	fn test_header_validation() {
		let header = SectorHeader { archive_id: 0, chunk: 0, next: 2, index_id: 255 };

		assert_eq!(header.validate(1, 0, 255), Err(ReadError::SectorArchiveMismatch(header.archive_id, 1)));
		assert_eq!(header.validate(0, 1, 255), Err(ReadError::SectorChunkMismatch(header.chunk, 1)));
		assert_eq!(header.validate(0, 0, 0), Err(ReadError::SectorIndexMismatch(header.index_id, 0)));
	}
}