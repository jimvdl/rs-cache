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

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Sector<'a> {
	pub header: SectorHeader,
	pub data_block: &'a [u8]
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct SectorHeader {
	pub archive_id: u32,
	pub chunk: u16,
	pub next: u32,
	pub index_id: u8
}

impl<'a> Sector<'a> {
	#[inline]
	pub fn new(buffer: &'a [u8], expanded_header: bool) -> crate::Result<Self> {
		let (buffer, archive_id) = if expanded_header {
			be_u32(buffer)?
		} else {
			let (buffer, archive_id) = be_u16(buffer)?;
			(buffer, archive_id as u32)
		};
		
		let (buffer, chunk) = be_u16(buffer)?;
		let (buffer, next) = be_u24(buffer)?;
		let (buffer, index_id) = be_u8(buffer)?;
		let (_, data_block) = rest(buffer)?;

		let header = SectorHeader { archive_id, chunk, next, index_id };

		Ok(Self { header, data_block })
	}
}

impl SectorHeader {
	#[inline]
	pub fn validate(&self, archive_id: u32, chunk: u16, index_id: u8) -> crate::Result<()> {
		if self.archive_id != archive_id {
			return Err(ReadError::SectorArchiveMismatch(self.archive_id, archive_id).into())
		}

		if self.chunk != chunk {
			return Err(ReadError::SectorChunkMismatch(self.chunk, chunk).into())
		}

		if self.index_id != index_id {
			return Err(ReadError::SectorIndexMismatch(self.index_id, index_id).into())
		}

		Ok(())
	}
}