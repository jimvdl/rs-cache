use std::{ io::Read, fs::File };

use crate::arc::Archive;

use super::*;

pub struct MemoryStore {
    data: Vec<u8>
}

impl Store for MemoryStore {
	#[inline]
    fn new(mut main_file: File) -> crate::Result<Self> {
        let mut buffer = Vec::new();
        main_file.read_to_end(&mut buffer)?;
        
        Ok(Self { data: buffer })
    }

	#[inline]
    fn read(&self, archive: &Archive) -> Vec<u8> {
        let mut sector = archive.sector;
        let mut data = vec![0; archive.length];
		let mut remaining = archive.length;
		let mut current = 0;

		loop {
			let offset = sector as usize * SECTOR_SIZE;
			
			if remaining >= SECTOR_DATA_SIZE {
				let data_block = &self.data[offset..offset + SECTOR_SIZE];
				
				sector =  u32::from(data_block[4]) << 16 
				        | u32::from(data_block[5]) << 8 
				        | u32::from(data_block[6]);
				remaining -= SECTOR_DATA_SIZE;

				let data_block = &data_block[SECTOR_HEADER_SIZE..];
				data[current..current + SECTOR_DATA_SIZE].copy_from_slice(data_block);
				
				current += SECTOR_DATA_SIZE;
			} else {
				if remaining == 0 {
					break;
				}

				let offset = offset + SECTOR_HEADER_SIZE;
				let data_block = &self.data[offset..offset + remaining];

				data[current..current + remaining].copy_from_slice(data_block);
				break;
			}
		}

		data
    }
}