//! Index with parsing.

use std::collections::HashMap;

use nom::number::complete::be_u24;

use crate::{ arc::Archive, error::ParseError };

pub const IDX_LENGTH: usize = 6;

/// Represents an .idx file.
#[derive(Clone, Debug, Default)]
pub struct Index {
	pub id: u8,
	pub archives: HashMap<u32, Archive>,
}

impl Index {
	/// Creates an `Index` from the given buffer.
	/// 
	/// The buffer always contains the entire index file in bytes
	/// and is internally parsed.
	/// 
	/// # Examples
	/// 
	/// ```
	/// # use std::fs::File;
	/// # use std::io::{self, Read};
	/// # use rscache::idx::Index;
	/// # fn main() -> rscache::Result<()> {
	///     let mut index_file = File::open("./data/osrs_cache/main_file_cache.idx2")?;
	///     let mut index_buffer = Vec::with_capacity(index_file.metadata()?.len() as usize);
	/// 
	///     index_file.read_to_end(&mut index_buffer)?;
	///     let index = Index::new(2, &index_buffer)?;
	/// # Ok(())
	/// # }
	/// ```
	#[inline]
    pub fn new(id: u8, buffer: &[u8]) -> crate::Result<Self> {
		let mut archives = HashMap::new();

		for (archive_id, archive_metadata) in buffer.chunks_exact(IDX_LENGTH).enumerate() {
			let archive_id = archive_id as u32;

			let archive = match parse_archive(archive_id, id, archive_metadata) {
				Ok(archive) => archive,
				Err(_) => return Err(ParseError::Archive(archive_id).into())
			};
			
			archives.insert(archive_id, archive);
		}

        Ok(Self { id, archives })
	}
}

fn parse_archive(id: u32, index_id: u8, buffer: &[u8]) -> crate::Result<Archive> {
	let (buffer, len) = be_u24(buffer)?;
	let (_, sec) = be_u24(buffer)?;
	
	Ok(Archive { id, index_id, sector: sec as usize, length: len as usize })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_archive() -> crate::Result<()> {
		let buffer = &[0, 0, 77, 0, 1, 196];

		let expected = Archive { id: 10, index_id: 255, sector: 452, length: 77 };
		let actual = parse_archive(10, 255, buffer)?;

		assert_eq!(actual, expected);

		Ok(())
	}
}