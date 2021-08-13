use std::{ io::Read, fs::File };

use crate::{
	arc::Archive,
	sec::{ Sector, SectorHeaderSize },
	error::ParseError,
	sec::{
		SECTOR_SIZE,
		SECTOR_DATA_SIZE,
		SECTOR_HEADER_SIZE
	},
};

use super::Store;

/// Cache inner reading from memory.
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
    fn read(&self, archive: &Archive) -> crate::Result<Vec<u8>> {
		let mut current_sector = archive.sector;
        let mut data = vec![0; archive.length];
		let mut remaining = archive.length;
		let mut current = 0;
		let mut chunk = 0;

		loop {
			let offset = current_sector as usize * SECTOR_SIZE;
			
			if remaining >= SECTOR_DATA_SIZE {
				let data_block = &self.data[offset..offset + SECTOR_SIZE];
				
				match Sector::new(data_block, &SectorHeaderSize::Normal) {
					Ok(sector) => {
						sector.header.validate(archive.id, chunk, archive.index_id)?;

						current_sector = sector.header.next;
						data[current..current + SECTOR_DATA_SIZE].copy_from_slice(sector.data_block);
					},
					Err(_) => return Err(ParseError::Sector(archive.sector).into())
				};
				
				remaining -= SECTOR_DATA_SIZE;
				current += SECTOR_DATA_SIZE;
			} else {
				if remaining == 0 { break; }
				
				let data_block = &self.data[offset..offset + SECTOR_HEADER_SIZE + remaining];
				
				match Sector::new(data_block, &SectorHeaderSize::Normal) {
					Ok(sector) => {
						sector.header.validate(archive.id, chunk, archive.index_id)?;
						data[current..current + remaining].copy_from_slice(sector.data_block);

						break;
					},
					Err(_) => return Err(ParseError::Sector(archive.sector).into())
				};
			}
			
			chunk += 1;
		}

		Ok(data)
    }
}