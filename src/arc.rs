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
    pub sector: u32,
    pub length: usize
}

/// Represents archive metadata.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct ArchiveData {
    pub id: u16,
    pub hash: i32,
    pub crc: u32,
    pub revision: u32,
    pub entry_count: usize
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

// TODO: id could be u16 or u24 depending on protocol (rs3)
#[inline]
pub fn parse_archive_data(buffer: &[u8]) -> crate::Result<Vec<ArchiveData>> {
    let (buffer, protocol) = be_u8(buffer)?;
    let (buffer, _) = cond(protocol >= 6, be_u32)(buffer)?;
    let (buffer, identified) = parse_identified(buffer)?;
    let (buffer, archive_count) = parse_usize_from_u16(buffer)?;
    let (buffer, ids) = many_m_n(0, archive_count, be_i16)(buffer)?;
    let (buffer, hashes) = parse_identifiers(buffer, identified, archive_count)?;
    let (buffer, crcs) = many_m_n(0, archive_count, be_u32)(buffer)?;
    let (buffer, revisions) = many_m_n(0, archive_count, be_u32)(buffer)?;
    let (_, entry_counts) = many_m_n(0, archive_count, be_u16)(buffer)?;

    let mut archives = Vec::with_capacity(archive_count);
    let mut last_archive_id = 0;
    let archive_data = izip!(&ids, &hashes, &crcs, &revisions, &entry_counts);
    let mut last_archive_id = 0;
    for (id, hash, crc, revision, entry_count) in archive_data {
        last_archive_id += id;

        archives.push(ArchiveData { 
            id: last_archive_id as u16, 
            hash: *hash, 
            crc: *crc, 
            revision: *revision, 
            entry_count: *entry_count as usize 
        });
    }
    
    Ok(archives)
}

fn parse_identified(buffer: &[u8]) -> crate::Result<(&[u8], bool)> {
    let (buffer, identified) = be_u8(buffer)?;

    Ok((buffer, (1 & identified) != 0))
}

fn parse_identifiers(buffer: &[u8], identified: bool, archive_count: usize) -> crate::Result<(&[u8], Vec<i32>)> {
    let (buffer, taken) = cond(identified, take(archive_count * 4))(buffer)?;
    let (_, mut identifiers) = many0(be_i32)(match taken {
        Some(taken) => taken,
        None => &[]
    })?;
    
    if identifiers.len() != archive_count {
        identifiers = vec![0; archive_count]; 
    }
    
    Ok((buffer, identifiers))
}

fn parse_usize_from_u16(buffer: &[u8]) -> crate::Result<(&[u8], usize)> {
    let (buffer, value) = be_u16(buffer)?;

    Ok((buffer, value as usize))
}