use std::collections::HashMap;

use nom::number::complete::be_u24;

use crate::{ arc::Archive, error::ParseError };

pub const IDX_LENGTH: usize = 6;

#[derive(Clone, Debug, Default)]
pub struct Index {
	archives: HashMap<u32, Archive>,
}

impl Index {
	#[inline]
    pub fn new(buffer: &[u8]) -> crate::Result<Self> {
		let mut archives = HashMap::new();

		for (id, archive_metadata) in buffer.chunks_exact(IDX_LENGTH).enumerate() {
			let id = id as u32;

			let archive = match parse_archive(id, archive_metadata) {
				Ok(archive) => archive,
				Err(_) => return Err(ParseError::Archive(id).into())
			};
			
			archives.insert(id, archive);
		}

        Ok(Self { archives })
	}

	#[inline]
	pub fn archive(&self, archive_id: u32) -> Option<&Archive> {
		self.archives.get(&archive_id)
	}
}

fn parse_archive(id: u32, buffer: &[u8]) -> crate::Result<Archive> {
	let (buffer, len) = be_u24(buffer)?;
	let (_, sec) = be_u24(buffer)?;

	Ok(Archive { id, sector: sec as u32, length: len as usize })
}