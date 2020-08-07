use std::{ 
    fs::File,
    io::{ Read, Seek, SeekFrom, BufReader },
    cell::RefCell,
};

use crate::{
	arc::Archive,
	sec::Sector,
	error::ParseError,
};
use super::*;

pub struct FileStore {
    handle: RefCell<BufReader<File>>
}

impl Store for FileStore {
	#[inline]
    fn new(main_file: File) -> crate::Result<Self> {
        Ok(Self { handle: RefCell::new(BufReader::new(main_file)) })
    }

	#[inline]
    fn read(&self, archive: &Archive) -> crate::Result<Vec<u8>> {
        let expanded_header = archive.id > std::u16::MAX.into();
		let mut current_sector = archive.sector;
        let mut data = vec![0; archive.length];
		let mut remaining = archive.length;
		let mut data_block = vec![0; SECTOR_SIZE];
		let mut current = 0;
		let mut chunk = 0;
		let header_len = if expanded_header { SECTOR_EXPANDED_HEADER_SIZE } else { SECTOR_HEADER_SIZE };
		let data_len = if expanded_header { SECTOR_EXPANDED_DATA_SIZE } else { SECTOR_DATA_SIZE };

		loop {
			let offset = current_sector as usize * SECTOR_SIZE;
			
			if remaining >= data_len {
                self.handle.borrow_mut().seek(SeekFrom::Start(offset as u64))?;
                self.handle.borrow_mut().read_exact(&mut data_block)?;
					
				let sector = match Sector::new(&data_block, expanded_header) {
					Ok(sector) => sector,
					Err(_) => return Err(ParseError::Sector(archive.sector).into())
				};
				sector.header.validate(archive.id, chunk, archive.index_id)?;

				current_sector = sector.header.next;

				remaining -= data_len;
				data[current..current + data_len].copy_from_slice(sector.data_block);
				current += data_len;
			} else {
				if remaining == 0 { break; }

				let mut data_block = vec![0; remaining + header_len];

                self.handle.borrow_mut().seek(SeekFrom::Start(offset as u64))?;
				self.handle.borrow_mut().read_exact(&mut data_block)?;
				
				let sector = match Sector::new(&data_block, expanded_header) {
					Ok(sector) => sector,
					Err(_) => return Err(ParseError::Sector(archive.sector).into())
				};
				sector.header.validate(archive.id, chunk, archive.index_id)?;

				data[current..current + remaining].copy_from_slice(&data_block);
				break;
			}

			chunk += 1;
		}

		Ok(data)
    }
}