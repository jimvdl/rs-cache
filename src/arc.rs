//! Archives with parsing and decoding.

use std::{
    io,
    collections::HashMap,
};

use nom::{
    multi::{ many0, many_m_n },
    combinator::cond,
    bytes::complete::take,
	number::complete::{
        be_u8,
        be_u16,
        be_u32,
        be_i16,
        be_i32
    },
};
use itertools::izip;

/// Represents an archive contained in an index.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Archive {
    pub id: u32,
    pub index_id: u8,
    pub sector: usize,
    pub length: usize
}

/// Represents archive metadata.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct ArchiveData {
    pub id: u16,
    pub name_hash: i32,
    pub crc: u32,
    pub hash: i32,
    pub whirlpool: [u8; 64],
    pub revision: u32,
    pub entry_count: usize,
    pub valid_ids: Vec<u16>,
}

#[inline]
pub fn parse_content(buffer: &[u8], entry_count: usize) -> io::Result<HashMap<u16, Vec<u8>>> {
    let chunks = buffer[buffer.len() - 1] as usize;
    let mut data = HashMap::new();
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

            cached_chunks.push((entry_id as u16, chunk_size as usize));
        }
    }
    
    read_ptr = 0;
    for (entry_id, chunk_size) in cached_chunks {
        let buf = buffer[read_ptr..read_ptr + chunk_size].to_vec();

        data.insert(entry_id, buf);
        read_ptr += chunk_size;
    }

    Ok(data)
}

#[inline]
pub fn parse_archive_data(buffer: &[u8]) -> crate::Result<Vec<ArchiveData>> {
    let (buffer, protocol) = be_u8(buffer)?;
    let (buffer, _) = cond(protocol >= 6, be_u32)(buffer)?;
    let (buffer, identified, whirlpool, codec, hash) = parse_identified(buffer)?;
    let (buffer, archive_count) = parse_archive_count(buffer)?;
    let (buffer, ids) = many_m_n(0, archive_count, be_i16)(buffer)?;
    let (buffer, name_hashes) = parse_hashes(buffer, identified, archive_count)?;
    let (buffer, crcs) = many_m_n(0, archive_count, be_u32)(buffer)?;
    let (buffer, hashes) = parse_hashes(buffer, hash, archive_count)?;
    let (buffer, whirlpools) = parse_whirlpools(buffer, whirlpool, archive_count)?;
    // skip for now
    //let (buffer, compressed, decompressed) = parse_codec(buffer, codec, archive_count)?;
    let (buffer, _) = cond(codec, many_m_n(0, archive_count * 8, be_u8))(buffer)?;
    let (buffer, revisions) = many_m_n(0, archive_count, be_u32)(buffer)?;
    let (buffer, entry_counts) = many_m_n(0, archive_count, be_u16)(buffer)?;
    let entry_counts: Vec<usize> = entry_counts.iter().map(|&entry_count| entry_count as usize).collect();
    let (_, valid_ids) = parse_valid_ids(buffer, &entry_counts)?;

    let mut archives = Vec::with_capacity(archive_count);
    let mut last_archive_id = 0;
    let archive_data = izip!(ids, name_hashes, crcs, hashes, whirlpools, revisions, entry_counts, valid_ids);
    for (id, name_hash, crc, hash, whirlpool, revision, entry_count, valid_ids) in archive_data {
        last_archive_id += id;
        
        archives.push(ArchiveData { 
            id: last_archive_id as u16, 
            name_hash,
            crc,
            hash,
            whirlpool,
            revision, 
            entry_count: entry_count as usize,
            valid_ids,
        });
    }
    
    Ok(archives)
}

fn parse_identified(buffer: &[u8]) -> crate::Result<(&[u8], bool, bool, bool, bool)> {
    let (buffer, identified) = be_u8(buffer)?;

    let whirlpool = (2 & identified) != 0;
    let codec = (identified & 4) != 0;
    let hash = (identified & 8) != 0;
    let identified = (1 & identified) != 0;

    Ok((buffer, identified, whirlpool, codec, hash))
}

fn parse_hashes(buffer: &[u8], hash: bool, archive_count: usize) -> crate::Result<(&[u8], Vec<i32>)> {
    let (buffer, taken) = cond(hash, take(archive_count * 4))(buffer)?;
    let (_, mut hashes) = many0(be_i32)(taken.unwrap_or(&[]))?;

    if hashes.len() != archive_count {
        hashes = vec![0; archive_count * 4]; 
    }

    Ok((buffer, hashes))
}

fn parse_whirlpools(buffer: &[u8], whirlpool: bool, archive_count: usize) -> crate::Result<(&[u8], Vec<[u8; 64]>)> {
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

fn parse_valid_ids<'a>(buffer: &'a [u8], entry_counts: &[usize]) -> crate::Result<(&'a [u8], Vec<Vec<u16>>)> {
    let mut result = Vec::with_capacity(entry_counts.len());
    let count: usize = entry_counts.iter().sum();
    let (buffer, mut taken) = take(count * 2)(buffer)?;

    for entry_count in entry_counts {
        let (buf, id_modifiers) = many_m_n(0, *entry_count as usize, be_u16)(taken)?;
        taken = buf;

        let mut ids = Vec::with_capacity(id_modifiers.len());
        let mut id = 0;
        for current_id in id_modifiers {
            id += current_id;
            ids.push(id);
        }

        result.push(ids);
    }

    Ok((buffer, result))
}

fn parse_archive_count(buffer: &[u8]) -> crate::Result<(&[u8], usize)> {
    let (buffer, value) = be_u16(buffer)?;

    Ok((buffer, value as usize))
}