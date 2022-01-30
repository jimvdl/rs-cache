use std::slice::{Iter, IterMut};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use nom::number::complete::be_u24;

use crate::sector::{
    SectorHeaderSize, SECTOR_DATA_SIZE, SECTOR_EXPANDED_DATA_SIZE, SECTOR_EXPANDED_HEADER_SIZE,
    SECTOR_HEADER_SIZE,
};

pub const ARCHIVE_REF_LEN: usize = 6;

/// A reference to an archive segment.
/// 
/// Archives are not stored in a contiguous fashion.
/// An `ArchiveRef` is _basically_ a wide pointer, like `&str` or `&[u8]`. When an `Archive` is fetched from the
/// [`Dat2`](crate::Dat2) file it knows where the memory starts and its length. Each block of memory belong to the archive
/// is viewed as a [`Sector`](crate::Sector), which contains a pointer to the next sector. Once this chain is 
/// exhausted you are left with all of the archive data.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct ArchiveRef {
    pub id: u32,
    pub index_id: u8,
    pub sector: usize,
    pub length: usize,
}

impl ArchiveRef {
    /// Transforms an [`Index`](crate::Index) sub-buffer into an archive reference.
    /// 
    /// # Errors
    /// 
    /// Will fail if the buffer is not exactly 6 bytes in length.
    pub fn from_buffer(id: u32, index_id: u8, buffer: &[u8]) -> crate::Result<Self> {
        let (buffer, len) = be_u24(buffer)?;
        let (_, sec) = be_u24(buffer)?;

        Ok(Self {
            id,
            index_id,
            sector: sec as usize,
            length: len as usize,
        })
    }

    /// Generate a data block iterator from this archive reference.
    pub fn data_blocks(&self) -> DataBlocks {
        let (header_len, data_len) = match SectorHeaderSize::from(self) {
            SectorHeaderSize::Normal => (SECTOR_HEADER_SIZE, SECTOR_DATA_SIZE),
            SectorHeaderSize::Expanded => (SECTOR_EXPANDED_HEADER_SIZE, SECTOR_EXPANDED_DATA_SIZE),
        };

        let n = self.length / data_len;
        let rem = self.length % data_len;
        let n = if rem > 0 { n + 1 } else { n };

        DataBlocks {
            count: n,
            remainder: rem,
            header_len,
            data_len,
        }
    }
}

/// Iterator to walk the archive reference chain.
/// 
/// When reading an `Archive` from [`Dat2`](crate::Dat2) it needs to know
/// where to read and what the total data length is.
/// By calling [`ArchiveRef::data_blocks`](ArchiveRef::data_blocks) you get a
/// easy iterable struct that does all of the calculations for you.
pub struct DataBlocks {
    count: usize,
    remainder: usize,
    header_len: usize,
    data_len: usize,
}

impl Iterator for DataBlocks {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count == 0 {
            return None;
        }

        let n = if self.count == 1 && self.remainder != 0 {
            self.remainder
        } else {
            self.data_len
        };

        self.count -= 1;
        Some(self.header_len + n)
    }
}

/// Metadata on every archive.
/// 
/// # Example
/// 
/// TODO
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct ArchiveMetadata {
    pub id: u32,
    pub name_hash: i32,
    pub crc: u32,
    pub hash: i32,
    #[cfg_attr(feature = "serde", serde(with = "BigArray"))]
    pub whirlpool: [u8; 64],
    pub version: u32,
    pub entry_count: usize,
    pub valid_ids: Vec<u32>,
}

/// Holds an archive file id with its data.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct ArchiveFileData {
    pub id: u32,
    pub data: Vec<u8>,
}

/// Holds all of the archive files that belong to a single archive.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct ArchiveFileGroup(Vec<ArchiveFileData>);

impl ArchiveFileGroup {
    /// Format a raw buffer into a list of `ArchiveFileData`'s.
    /// 
    /// # Panics
    /// 
    /// Whenever the buffer has a wrong format no files can be constructed.
    pub fn from_buffer(buffer: &[u8], entry_count: usize) -> Self {
        let chunks = buffer[buffer.len() - 1] as usize;
        let mut data = Vec::with_capacity(chunks);
        let mut cached_chunks = Vec::with_capacity(chunks);
        let mut read_ptr = buffer.len() - 1 - chunks * entry_count * 4;

        for _ in 0..chunks {
            let mut chunk_size = 0;

            for entry_id in 0..entry_count {
                let mut bytes = [0; 4];
                bytes.copy_from_slice(&buffer[read_ptr..read_ptr + 4]);
                let delta = i32::from_be_bytes(bytes);
                read_ptr += 4;
                chunk_size += delta;

                cached_chunks.push((entry_id as u32, chunk_size as usize));
            }
        }
        read_ptr = 0;
        for (entry_id, chunk_size) in cached_chunks {
            let buf = buffer[read_ptr..read_ptr + chunk_size].to_vec();

            data.push(ArchiveFileData {
                id: entry_id,
                data: buf,
            });
            read_ptr += chunk_size;
        }

        Self(data)
    }

    #[inline]
    pub fn iter(&self) -> Iter<'_, ArchiveFileData> {
        self.0.iter()
    }

    #[inline]
    pub fn iter_mut(&mut self) -> IterMut<'_, ArchiveFileData> {
        self.0.iter_mut()
    }
}

#[cfg(feature = "rs3")]
#[cfg_attr(docsrs, doc(cfg(feature = "rs3")))]
impl IntoIterator for ArchiveFileGroup {
    type Item = ArchiveFileData;
    type IntoIter = std::vec::IntoIter<ArchiveFileData>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

#[cfg(feature = "rs3")]
#[cfg_attr(docsrs, doc(cfg(feature = "rs3")))]
impl<'a> IntoIterator for &'a ArchiveFileGroup {
    type Item = &'a ArchiveFileData;
    type IntoIter = Iter<'a, ArchiveFileData>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}
#[test]
fn parse_archive() -> crate::Result<()> {
    let buffer = &[0, 0, 77, 0, 1, 196];
    let archive = ArchiveRef::from_buffer(10, 255, buffer)?;

    assert_eq!(
        archive,
        ArchiveRef {
            id: 10,
            index_id: 255,
            sector: 452,
            length: 77
        }
    );

    Ok(())
}
