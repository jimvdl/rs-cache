//! Index with parsing.

use std::{
	path::Path,
	collections::{ hash_map, HashMap },
	fs::File,
	io::Read,
	ops,
};

use serde::{ Serialize, Deserialize };

use crate::{ 
	arc::Archive, 
	error::{ ReadError, ParseError },
	cache::REFERENCE_TABLE,
};

pub const IDX_PREFIX: &str = "main_file_cache.idx";
pub const IDX_LENGTH: usize = 6;

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Indices(HashMap<u8, Index>);

/// Represents an .idx file.
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Index {
	pub id: u8,
	pub archives: HashMap<u32, Archive>,
}

impl Indices {
	#[inline]
	pub fn new<P: AsRef<Path>>(path: P) -> crate::Result<Self> {
		let path = path.as_ref();
		let mut indices = HashMap::new();

		let ref_tbl_path = path.join(format!("{}{}", IDX_PREFIX, REFERENCE_TABLE));
		if !ref_tbl_path.exists() {
			return Err(ReadError::ReferenceTableNotFound.into());
		}

		for index_id in 0..=REFERENCE_TABLE {
			let path = path.join(format!("{}{}", IDX_PREFIX, index_id));

			if path.exists() {
				let mut index_file = File::open(path)?;
				let mut index_buffer = Vec::with_capacity(index_file.metadata()?.len() as usize);

				index_file.read_to_end(&mut index_buffer)?;
				indices.insert(index_id, Index::new(index_id, &index_buffer)?);
			}
		}

		Ok(Self(indices))
	}

	#[inline]
	pub fn get(&self, key: &u8) -> Option<&Index> {
		self.0.get(key)
	}

	#[inline]
	pub fn len(&self) -> usize {
		self.0.len()
	}

	#[inline]
	pub fn is_empty(&self) -> bool {
		self.0.is_empty()
	}

	#[inline]
	pub const fn inner(&self) -> &HashMap<u8, Index> {
		&self.0
	}
}

impl<'a> ops::Index<&'a u8> for Indices {
	type Output = Index;

	#[inline]
	fn index(&self, key: &'a u8) -> &Self::Output {
		&self.0[key]
	}
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

			let archive = match Archive::from_buffer(archive_id, id, archive_metadata) {
				Ok(archive) => archive,
				Err(_) => return Err(ParseError::Archive(archive_id).into())
			};
			
			archives.insert(archive_id, archive);
		}

        Ok(Self { id, archives })
	}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_archive() -> crate::Result<()> {
		let buffer = &[0, 0, 77, 0, 1, 196];

		let expected = Archive { id: 10, index_id: 255, sector: 452, length: 77 };
		let actual = Archive::from_buffer(10, 255, buffer)?;

		assert_eq!(actual, expected);

		Ok(())
	}
}

impl IntoIterator for Indices {
	type Item = (u8, Index);
	type IntoIter = hash_map::IntoIter<u8, Index>;

	#[inline]
	fn into_iter(self) -> Self::IntoIter {
		self.0.into_iter()
	}
}

impl<'a> IntoIterator for &'a Indices {
	type Item = (&'a u8, &'a Index);
	type IntoIter = hash_map::Iter<'a, u8, Index>;

	#[inline]
	fn into_iter(self) -> Self::IntoIter {
		self.0.iter()
	}
}