use std::{
    io,
    collections::HashMap,
};

use nom::{
    multi::many0,
    combinator::cond,
    bytes::complete::take,
	number::complete::{
		be_u16,
		be_u32,
        be_u8,
        be_i32
	},
};
use itertools::izip;

use crate::CacheError;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Archive {
    pub sector: u32,
    pub length: usize
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct ArchiveData {
    id: u16,
    identifier: i32,
    crc: u32,
    revision: u32,
    entry_count: usize
}

impl Archive {
    pub const fn new(sector: u32, length: usize) -> Self {
		Self { sector, length }
    }
    
    pub fn decode(buffer: &[u8], entry_count: usize) -> io::Result<HashMap<u16, Vec<u8>>> {
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
}

impl ArchiveData {
    pub fn decode(buffer: &[u8]) -> Result<Vec<Self>, CacheError> {
        let (buffer, protocol) = be_u8(buffer)?;
        let (buffer, _) = cond(protocol >= 6, be_u32)(buffer)?;
        let (buffer, identified) = be_u8(buffer)?;
        let (buffer, archive_count) = be_u16(buffer)?;

        let archive_count = archive_count as usize;
        let identified = (1 & identified) != 0;

        let (buffer, taken) = take(archive_count * 2)(buffer)?;
        let (_, ids) = many0(be_u16)(taken)?;

        let (buffer, taken) = cond(identified, take(archive_count * 4))(buffer)?;
        let (_, mut identifiers) = many0(be_i32)(match taken {
            Some(taken) => taken,
            None => &[]
        })?;
        if identifiers.len() != archive_count {
            identifiers = vec![0; archive_count]; 
        }

        let (buffer, taken) = take(archive_count * 4)(buffer)?;
        let (_, crcs) = many0(be_u32)(taken)?;

        let (buffer, taken) = take(archive_count * 4)(buffer)?;
        let (_, revisions) = many0(be_u32)(taken)?;

        let (_, taken) = take(archive_count * 2)(buffer)?;
        let (_, entry_counts) = many0(be_u16)(taken)?;

        let mut archives = Vec::with_capacity(archive_count);
        let archive_data = izip!(&ids, &identifiers, &crcs, &revisions, &entry_counts);
        for (id, identifier, crc, revision, entry_count) in archive_data {
            archives.push(Self { 
                id: *id, 
                identifier: *identifier, 
                crc: *crc, 
                revision: *revision, 
                entry_count: *entry_count as usize 
            });
        }
        
        Ok(archives)
    }

    #[inline]
    pub const fn id(&self) -> u16 {
        self.id
    }

    #[inline]
    pub const fn identifier(&self) -> i32 {
        self.identifier
    }

    #[inline]
    pub const fn entry_count(&self) -> usize {
        self.entry_count
    }
}