use std::{
    collections::{hash_map, HashMap},
    fs::File,
    io::Read,
    path::Path,
    slice::Iter,
};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "serde")]
use serde_big_array::big_array;
#[cfg(feature = "serde")]
big_array! { BigArray; }

use crate::{
    archive::{ArchiveRef, ARCHIVE_REF_LEN},
    error::{ParseError, ReadError},
    Dat2, REFERENCE_TABLE_ID,
};
use itertools::izip;
use nom::{
    bytes::complete::take,
    combinator::cond,
    multi::{many0, many_m_n},
    number::complete::{be_i32, be_u16, be_u32, be_u8},
};

use crate::parse::be_u32_smart;
use crate::codec::{Buffer, Decoded};

pub const IDX_PREFIX: &str = "main_file_cache.idx";

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, Default)]
pub struct Indices(pub HashMap<u8, Index>);

impl Indices {
    pub fn new<P: AsRef<Path>>(path: P) -> crate::Result<Self> {
        let path = path.as_ref();
        let mut indices = HashMap::new();

        let ref_index = Index::from_path(
            REFERENCE_TABLE_ID,
            path.join(format!("{}{}", IDX_PREFIX, REFERENCE_TABLE_ID)),
        )?;

        let data = Dat2::new(path.join(crate::MAIN_DATA))?;

        for index_id in 0..REFERENCE_TABLE_ID {
            let path = path.join(format!("{}{}", IDX_PREFIX, index_id));

            if !path.exists() {
                continue;
            }
            let mut index = Index::from_path(index_id, path)?;

            let archive_ref = ref_index.archive_refs.get(&(index_id as u32)).ok_or(
                ReadError::ArchiveNotFound {
                    idx: REFERENCE_TABLE_ID,
                    arc: index_id as u32,
                },
            )?;

            if archive_ref.length != 0 {
                let buffer = data.read(archive_ref)?.decode()?;
                index.metadata = IndexMetadata::try_from(buffer)?;
            }

            indices.insert(index_id, index);
        }

        indices.insert(REFERENCE_TABLE_ID, ref_index);

        Ok(Self(indices))
    }

    pub fn get(&self, key: &u8) -> Option<&Index> {
        self.0.get(key)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, Default)]
pub struct Index {
    pub id: u8,
    pub archive_refs: HashMap<u32, ArchiveRef>,
    pub metadata: IndexMetadata,
}

impl Index {
    pub fn from_path<P: AsRef<Path>>(id: u8, path: P) -> crate::Result<Self> {
        let path = path.as_ref();
        let index_extension = format!("idx{}", id);
        let extension = path
            .extension()
            .and_then(std::ffi::OsStr::to_str)
            .unwrap_or("");

        if extension != index_extension {
            panic!("index extension missmatch: expected {index_extension} but found {extension}");
        }

        let mut file = File::open(path)?;
        let mut buffer = Vec::with_capacity(file.metadata()?.len() as usize);
        file.read_to_end(&mut buffer)?;

        Self::from_buffer(id, &buffer)
    }

    pub fn from_buffer(id: u8, buffer: &[u8]) -> crate::Result<Self> {
        let mut archive_refs = HashMap::new();

        for (archive_id, archive_data) in buffer.chunks_exact(ARCHIVE_REF_LEN).enumerate() {
            let archive_id = archive_id as u32;

            let archive_ref = match ArchiveRef::from_buffer(archive_id, id, archive_data) {
                Ok(archive) => archive,
                Err(_) => return Err(ParseError::Archive(archive_id).into()),
            };
            archive_refs.insert(archive_id, archive_ref);
        }

        Ok(Self {
            id,
            archive_refs,
            metadata: IndexMetadata::default(),
        })
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

/// All of the index metadata read from the reference index.
// TODO: figure out a way to allocate this less error prone
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Default, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct IndexMetadata(Vec<ArchiveMetadata>);

impl IndexMetadata {
    #[inline]
    pub fn iter(&self) -> Iter<'_, ArchiveMetadata> {
        self.0.iter()
    }
}

impl std::convert::TryFrom<&[u8]> for IndexMetadata {
    type Error = crate::error::Error;

    fn try_from(buffer: &[u8]) -> Result<Self, Self::Error> {
        let (buffer, protocol) = be_u8(buffer)?;
        let (buffer, _) = cond(protocol >= 6, be_u32)(buffer)?;
        let (buffer, identified, whirlpool, codec, hash) = parse_identified(buffer)?;
        let (buffer, archive_count) = parse_archive_count(buffer, protocol)?;
        let (buffer, ids) = parse_ids(buffer, protocol, archive_count)?;
        let (buffer, name_hashes) = parse_hashes(buffer, identified, archive_count)?;
        let (buffer, crcs) = many_m_n(0, archive_count, be_u32)(buffer)?;
        let (buffer, hashes) = parse_hashes(buffer, hash, archive_count)?;
        let (buffer, whirlpools) = parse_whirlpools(buffer, whirlpool, archive_count)?;
        // skip for now
        //let (buffer, compressed, decompressed) = parse_codec(buffer, codec, archive_count)?;
        let (buffer, _) = cond(codec, many_m_n(0, archive_count * 8, be_u8))(buffer)?;
        let (buffer, versions) = many_m_n(0, archive_count, be_u32)(buffer)?;
        let (buffer, entry_counts) = parse_entry_counts(buffer, protocol, archive_count)?;
        let (_, valid_ids) = parse_valid_ids(buffer, protocol, &entry_counts)?;
        let mut archives = Vec::with_capacity(archive_count);
        let mut last_archive_id = 0;
        let archive_data = izip!(
            ids,
            name_hashes,
            crcs,
            hashes,
            whirlpools,
            versions,
            entry_counts,
            valid_ids
        );
        for (id, name_hash, crc, hash, whirlpool, version, entry_count, valid_ids) in archive_data {
            last_archive_id += id as i32;

            archives.push(ArchiveMetadata {
                id: last_archive_id as u32,
                name_hash,
                crc,
                hash,
                whirlpool,
                version,
                entry_count: entry_count as usize,
                valid_ids,
            });
        }
        Ok(Self(archives))
    }
}

impl std::convert::TryFrom<Buffer<Decoded>> for IndexMetadata {
    type Error = crate::error::Error;

    fn try_from(buffer: Buffer<Decoded>) -> Result<Self, Self::Error> {
        Self::try_from(buffer.as_slice())
    }
}

impl std::ops::Index<usize> for IndexMetadata {
    type Output = ArchiveMetadata;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl IntoIterator for IndexMetadata {
    type Item = ArchiveMetadata;
    type IntoIter = std::vec::IntoIter<ArchiveMetadata>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> IntoIterator for &'a IndexMetadata {
    type Item = &'a ArchiveMetadata;
    type IntoIter = Iter<'a, ArchiveMetadata>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
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

fn parse_identified(buffer: &[u8]) -> crate::Result<(&[u8], bool, bool, bool, bool)> {
    let (buffer, identified) = be_u8(buffer)?;

    let whirlpool = (2 & identified) != 0;
    let codec = (identified & 4) != 0;
    let hash = (identified & 8) != 0;
    let identified = (1 & identified) != 0;

    Ok((buffer, identified, whirlpool, codec, hash))
}

fn parse_hashes(
    buffer: &[u8],
    hash: bool,
    archive_count: usize,
) -> crate::Result<(&[u8], Vec<i32>)> {
    let (buffer, taken) = cond(hash, take(archive_count * 4))(buffer)?;
    let (_, mut hashes) = many0(be_i32)(taken.unwrap_or(&[]))?;

    if hashes.len() != archive_count {
        hashes = vec![0; archive_count * 4];
    }

    Ok((buffer, hashes))
}

fn parse_whirlpools(
    buffer: &[u8],
    whirlpool: bool,
    archive_count: usize,
) -> crate::Result<(&[u8], Vec<[u8; 64]>)> {
    let (buffer, taken) = cond(whirlpool, take(archive_count * 64))(buffer)?;
    let mut whirlpools = vec![[0; 64]; archive_count];

    for (index, chunk) in taken.unwrap_or(&[]).chunks_exact(64).enumerate() {
        whirlpools[index].copy_from_slice(chunk);
    }
    if whirlpools.len() != archive_count {
        whirlpools = vec![[0; 64]; archive_count];
    }

    Ok((buffer, whirlpools))
}

// fn parse_codec(buffer: &[u8], codec: bool, archive_count: usize) -> crate::Result<(&[u8], Vec<u32>, Vec<u32>)> {
//     todo!()
// }

fn parse_valid_ids<'a>(
    mut buffer: &'a [u8],
    protocol: u8,
    entry_counts: &[usize],
) -> crate::Result<(&'a [u8], Vec<Vec<u32>>)> {
    let mut result = Vec::with_capacity(entry_counts.len());

    for entry_count in entry_counts {
        let (buf, id_modifiers) = if protocol >= 7 {
            many_m_n(0, *entry_count as usize, be_u32_smart)(buffer)?
        } else {
            let (buf, result) = many_m_n(0, *entry_count as usize, be_u16)(buffer)?;
            let result = result.iter().map(|&id_mod| id_mod as u32).collect();

            (buf, result)
        };
        buffer = buf;

        let mut ids = Vec::with_capacity(id_modifiers.len());
        let mut id = 0_u32;
        for current_id in id_modifiers {
            id += current_id as u32;
            ids.push(id);
        }

        result.push(ids);
    }

    Ok((buffer, result))
}

fn parse_archive_count(buffer: &[u8], protocol: u8) -> crate::Result<(&[u8], usize)> {
    let (buffer, value) = if protocol >= 7 {
        be_u32_smart(buffer)?
    } else {
        let (buf, res) = be_u16(buffer)?;
        (buf, res as u32)
    };

    Ok((buffer, value as usize))
}

fn parse_ids(
    buffer: &[u8],
    protocol: u8,
    archive_count: usize,
) -> crate::Result<(&[u8], Vec<u32>)> {
    let (buffer, ids) = if protocol >= 7 {
        many_m_n(0, archive_count, be_u32_smart)(buffer)?
    } else {
        let (buf, res) = many_m_n(0, archive_count, be_u16)(buffer)?;
        let res = res.iter().map(|&ec| ec as u32).collect();
        (buf, res)
    };

    Ok((buffer, ids))
}

fn parse_entry_counts(
    buffer: &[u8],
    protocol: u8,
    archive_count: usize,
) -> crate::Result<(&[u8], Vec<usize>)> {
    let (buffer, entry_counts) = if protocol >= 7 {
        many_m_n(0, archive_count, be_u32_smart)(buffer)?
    } else {
        let (buf, res) = many_m_n(0, archive_count, be_u16)(buffer)?;
        let res = res.iter().map(|&ec| ec as u32).collect();

        (buf, res)
    };

    let entry_counts: Vec<usize> = entry_counts
        .iter()
        .map(|&entry_count| entry_count as usize)
        .collect();

    Ok((buffer, entry_counts))
}

#[test]
fn from_path_correct_extension() -> crate::Result<()> {
    let index2 = Index::from_path(2, "../data/osrs_cache/main_file_cache.idx2")?;
    let index15 = Index::from_path(15, "../data/osrs_cache/main_file_cache.idx15")?;
    let index255 = Index::from_path(255, "../data/osrs_cache/main_file_cache.idx255")?;

    assert_eq!(index2.id, 2);
    assert_eq!(index15.id, 15);
    assert_eq!(index255.id, 255);

    Ok(())
}

#[test]
#[should_panic]
fn from_path_incorrect_extension() {
    Index::from_path(2, "../data/osrs_cache/main_file_cache.idx1").unwrap();
}
