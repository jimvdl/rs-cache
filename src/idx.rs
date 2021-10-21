//! Index with parsing.

use std::{
    path::Path,
    collections::{ hash_map, HashMap },
    fs::File,
    io::Read,
};

use memmap::Mmap;
use serde::{ Serialize, Deserialize };

use crate::{ 
    codec,
    arc::{ ArchiveRef, Archive, ARCHIVE_REF_LEN }, 
    error::{ ReadError, ParseError },
    cache::{ REFERENCE_TABLE, ReadInternal },
};

/// Index prefix name.
pub const IDX_PREFIX: &str = "main_file_cache.idx";

/// Represents all indicies loaded by the cache.
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Indices(HashMap<u8, Index>);

/// Represents an .idx file.
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Index {
    id: u8,
    archive_refs: HashMap<u32, ArchiveRef>,
    archives: Vec<Archive>,
}

impl Indices {
    /// Loads all indices present in the cache folder.
    /// 
    /// It loops through the directory searching for the reference table and
    /// every index that is compatible.
    /// 
    /// # Errors
    /// 
    /// Can return multiple errors: 
    /// - Reference table not found.
    /// - Index failed to parse. 
    /// - Index couldn't be opened.
    #[inline]
    pub fn new<P: AsRef<Path>>(path: P, data: &Mmap) -> crate::Result<Self> {
        let path = path.as_ref();
        let mut indices = HashMap::new();

        let ref_tbl_path = path.join(format!("{}{}", IDX_PREFIX, REFERENCE_TABLE));
        let ref_index = if ref_tbl_path.exists() {
            Index::from_path(REFERENCE_TABLE, ref_tbl_path)?
        } else {
            return Err(ReadError::ReferenceTableNotFound.into());
        };
        
        // let mut indices: HashMap<u8, Index> = (0..REFERENCE_TABLE)
        //     .map(|index_id| (index_id, path.join(format!("{}{}", IDX_PREFIX, index_id))))
        //     .filter(|(_, path)| path.exists())
        //     .map(|(index_id, path)| Index::from_path(index_id, path))
        //     .take_while(|index| index.is_ok())
        //     .map(|index| index.unwrap())
        //     .map(|index| (index.id, index))
        //     .collect();

        // indices.iter_mut()
        //     .map(|(index_id, index)| -> crate::Result<()> {
        //         let archive_ref = ref_index.archive_refs().get(&(*index_id as u32))
        //             .ok_or(ReadError::ArchiveNotFound(REFERENCE_TABLE, *index_id as u32))?;
            
        //         if archive_ref.length != 0 {
        //             let mut buffer = Vec::with_capacity(archive_ref.length);
        //             data.read_internal(archive_ref, &mut buffer)?;
        //             let buffer = codec::decode(&buffer)?;
        //             index.archives = Archive::parse(&buffer)?;
        //         }

        //         Ok(())
        //     });

        for index_id in 0..REFERENCE_TABLE {
            let path = path.join(format!("{}{}", IDX_PREFIX, index_id));
            
            if path.exists() {
                let mut index = Index::from_path(index_id, path)?;

                let archive_ref = ref_index.archive_refs().get(&(index_id as u32))
                    .ok_or(ReadError::ArchiveNotFound(REFERENCE_TABLE, index_id as u32))?;
                
                if archive_ref.length != 0 {
                    let mut buffer = Vec::with_capacity(archive_ref.length);
                    data.read_internal(archive_ref, &mut buffer)?;
                    let buffer = codec::decode(&buffer)?;
                    index.archives = Archive::parse(&buffer)?;
                }
                
                indices.insert(index_id, index);
            }
        }

        indices.insert(REFERENCE_TABLE, ref_index);

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

impl Index {
    /// Creates an `Index` from a path.
    /// 
    /// # Panics
    /// 
    /// Panics if the file path given does not lead to valid index file with
    /// extension `.idx#` where the # must be its id.
    /// 
    /// Also panics if the given `id` does not match the id in the file extension.
    /// 
    /// # Errors
    /// 
    /// Can return multiple errors:
    /// - Index failed to parse. 
    /// - Index couldn't be opened.
    /// 
    /// # Examples
    /// 
    /// ```
    /// # use std::io::{ self, Read };
    /// # use rscache::fs::Index;
    /// # fn main() -> rscache::Result<()> {
    /// let index_id = 2;
    /// let index = Index::from_path(index_id, "./data/osrs_cache/main_file_cache.idx2")?;
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn from_path<P: AsRef<Path>>(id: u8, path: P) -> crate::Result<Self> {
        let path = path.as_ref();
        
        let index_extension = format!("idx{}", id);
        let extension = path.extension()
            .and_then(std::ffi::OsStr::to_str)
            .unwrap_or("");

        if extension != index_extension {
            panic!("Invalid index file. Expected file with extension {} but found {}.", index_extension, extension);
        }

        let mut file = File::open(path)?;
        let mut buffer = Vec::with_capacity(file.metadata()?.len() as usize);
        file.read_to_end(&mut buffer)?;

        Self::from_buffer(id, &buffer)
    }

    /// Creates an `Index` from the given buffer.
    /// 
    /// The buffer always contains the entire index file in bytes
    /// and is internally parsed.
    /// 
    /// # Errors
    /// 
    /// If the archive reference can't be parsed correctly it can return
    /// a EOF error.
    /// 
    /// # Examples
    /// 
    /// ```
    /// # use std::fs::File;
    /// # use std::io::{ self, Read };
    /// # use rscache::fs::Index;
    /// # fn main() -> rscache::Result<()> {
    /// # let index_id = 2;
    /// let mut index_file = File::open("./data/osrs_cache/main_file_cache.idx2")?;
    /// let mut index_buffer = Vec::with_capacity(index_file.metadata()?.len() as usize);
    /// 
    /// index_file.read_to_end(&mut index_buffer)?;
    /// let index = Index::from_buffer(index_id, &index_buffer)?;
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn from_buffer(id: u8, buffer: &[u8]) -> crate::Result<Self> {
        let mut archive_refs = HashMap::new();

        for (archive_id, archive_data) in buffer.chunks_exact(ARCHIVE_REF_LEN).enumerate() {
            let archive_id = archive_id as u32;

            let archive_ref = match ArchiveRef::from_buffer(archive_id, id, archive_data) {
                Ok(archive) => archive,
                Err(_) => return Err(ParseError::Archive(archive_id).into())
            };
            
            archive_refs.insert(archive_id, archive_ref);
        }

        Ok(Self { id, archive_refs, archives: Vec::new() })
    }

    #[inline]
    pub const fn id(&self) -> u8 {
        self.id
    }

    #[inline]
    pub(crate) const fn archive_refs(&self) -> &HashMap<u32, ArchiveRef> {
        &self.archive_refs
    }

    #[inline]
    pub(crate) const fn archives(&self) -> &Vec<Archive> {
        &self.archives
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_path_correct_extension() -> crate::Result<()> {
        let index2 = Index::from_path(2, "./data/osrs_cache/main_file_cache.idx2")?;
        let index15 = Index::from_path(15, "./data/osrs_cache/main_file_cache.idx15")?;
        let index255 = Index::from_path(255, "./data/osrs_cache/main_file_cache.idx255")?;

        assert_eq!(index2.id, 2);
        assert_eq!(index15.id, 15);
        assert_eq!(index255.id, 255);

        Ok(())
    }

    #[test]
    #[should_panic]
    fn from_path_incorrect_extension() {
        Index::from_path(2, "./data/osrs_cache/main_file_cache.idx1").unwrap();
    }
}