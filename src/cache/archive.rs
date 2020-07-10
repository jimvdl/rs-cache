use std::{
    io,
    io::Read,
    collections::HashMap,
};

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
    pub fn decode<R: Read>(reader: &mut R) -> Result<Vec<Self>, CacheError> {
        let mut buffer = [0; 1];
        reader.read_exact(&mut buffer)?;
        let protocol = buffer[0];

        if protocol >= 6 {
            let mut buffer = [0; 4];
			reader.read_exact(&mut buffer)?;
        }
        
        let mut buffer = [0; 1];
        reader.read_exact(&mut buffer)?;
        let identified = (1 & buffer[0]) != 0;
        
        let mut buffer = [0; 2];
        reader.read_exact(&mut buffer)?;
        let archive_count = u16::from_be_bytes(buffer) as usize;
        let mut archives = vec![Self::default(); archive_count];
        
        for archive_data in archives.iter_mut().take(archive_count) {
            let mut buffer = [0; 2];
            reader.read_exact(&mut buffer)?;
            let archive_id = u16::from_be_bytes(buffer);

            archive_data.id = archive_id;
        }

        if identified {
            for archive_data in archives.iter_mut().take(archive_count) {
                let mut buffer = [0; 4];
                reader.read_exact(&mut buffer)?;

                archive_data.identifier = i32::from_be_bytes(buffer);
            }
        }

        for archive_data in archives.iter_mut().take(archive_count) {
            let mut buffer = [0; 4];
            reader.read_exact(&mut buffer)?;

            archive_data.crc = u32::from_be_bytes(buffer);
        }

        for archive_data in archives.iter_mut().take(archive_count) {
            let mut buffer = [0; 4];
            reader.read_exact(&mut buffer)?;

            archive_data.revision = u32::from_be_bytes(buffer);
        }

        for archive_data in archives.iter_mut().take(archive_count) {
            let mut buffer = [0; 2];
			reader.read_exact(&mut buffer)?;

            archive_data.entry_count = u16::from_be_bytes(buffer) as usize;
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