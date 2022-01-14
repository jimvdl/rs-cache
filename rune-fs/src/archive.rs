use std::{
    io,
    slice::{Iter, IterMut},
};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "serde")]
use serde_big_array::big_array;
#[cfg(feature = "serde")]
big_array! { BigArray; }

use itertools::izip;
use nom::{
    bytes::complete::take,
    combinator::cond,
    multi::{many0, many_m_n},
    number::complete::{be_i32, be_u16, be_u24, be_u32, be_u8},
};

use crate::parse::be_u32_smart;
use crate::sector::{
    SectorHeaderSize, SECTOR_DATA_SIZE, SECTOR_EXPANDED_DATA_SIZE, SECTOR_EXPANDED_HEADER_SIZE,
    SECTOR_HEADER_SIZE,
};

pub const ARCHIVE_REF_LEN: usize = 6;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ArchiveRef {
    pub id: u32,
    pub index_id: u8,
    pub sector: usize,
    pub length: usize,
}

impl ArchiveRef {
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

    pub(crate) fn data_blocks(&self) -> DataBlocks {
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

pub(crate) struct DataBlocks {
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

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Archive {
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

impl Archive {
    pub fn parse(buffer: &[u8]) -> crate::Result<Vec<Self>> {
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

            archives.push(Self {
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
        Ok(archives)
    }
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ArchiveFileData {
    pub id: u32,
    pub data: Vec<u8>,
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ArchiveFileGroup(Vec<ArchiveFileData>);

impl ArchiveFileGroup {
    pub fn parse(buffer: &[u8], entry_count: usize) -> io::Result<Self> {
        let chunks = buffer[buffer.len() - 1] as usize;
        let mut data = Vec::with_capacity(chunks);
        let mut cached_chunks = Vec::new();
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

        Ok(Self(data))
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

impl IntoIterator for ArchiveFileGroup {
    type Item = ArchiveFileData;
    type IntoIter = std::vec::IntoIter<ArchiveFileData>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

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
