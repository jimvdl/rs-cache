use std::{ 
    fs::File,
    io::{ Read, Seek, SeekFrom, BufReader },
    cell::RefCell,
};

use crate::arc::Archive;

use super::*;

pub struct FileStore {
    handle: RefCell<BufReader<File>>
}

impl Store for FileStore {
    fn new(main_file: File) -> crate::Result<Self> {
        Ok(Self { handle: RefCell::new(BufReader::new(main_file)) })
    }

    fn read(&self, archive: &Archive) -> Vec<u8> {
        let expanded_header = archive.id > std::u16::MAX.into();
		let mut sector = archive.sector;
        let mut data = vec![0; archive.length];
		let mut remaining = archive.length;
		let mut data_block = vec![0; SECTOR_SIZE];
		let mut current = 0;
		let header_len = if expanded_header { SECTOR_EXPANDED_HEADER_SIZE } else { SECTOR_HEADER_SIZE };
		let data_len = if expanded_header { SECTOR_EXPANDED_DATA_SIZE } else { SECTOR_DATA_SIZE };

		loop {
			let offset = sector as usize * SECTOR_SIZE;
			
			if remaining >= data_len {
                self.handle.borrow_mut().seek(SeekFrom::Start(offset as u64)).unwrap();
                self.handle.borrow_mut().read_exact(&mut data_block).unwrap();
						
				sector = if expanded_header {
				    u32::from(data_block[6]) << 16 
				  | u32::from(data_block[7]) << 8 
				  | u32::from(data_block[8])
				} else {
				    u32::from(data_block[4]) << 16 
                  | u32::from(data_block[5]) << 8 
				  | u32::from(data_block[6])
				};

				remaining -= data_len;
				data[current..current + data_len].copy_from_slice(&data_block[header_len..]);
				current += data_len;
			} else {
				if remaining == 0 {
					break;
				}

                let offset = offset + header_len;
				let mut data_block = vec![0; remaining];

                self.handle.borrow_mut().seek(SeekFrom::Start(offset as u64)).unwrap();
                self.handle.borrow_mut().read_exact(&mut data_block).unwrap();

				data[current..current + remaining].copy_from_slice(&data_block);
				break;
			}
		}

		data
    }
}