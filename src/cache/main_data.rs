use std::collections::LinkedList;

pub const SECTOR_HEADER_SIZE: usize = 8;
pub const SECTOR_DATA_SIZE: usize = 512;
pub const SECTOR_SIZE: usize = SECTOR_HEADER_SIZE + SECTOR_DATA_SIZE;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct MainData {
	data: Vec<u8>,
}

impl MainData {
    pub fn new(buffer: Vec<u8>) -> Self {
        Self { data: buffer }
    }
	
	pub fn read(&self, mut sector: u32, size: usize) -> LinkedList<&[u8]> {
		let mut data = LinkedList::new();
		let mut remaining = size;

		loop {
			let offset = sector as usize * SECTOR_SIZE;
			
			if remaining >= SECTOR_DATA_SIZE {
				let data_block = &self.data[offset..offset + SECTOR_SIZE];
				data.push_back(&data_block[SECTOR_HEADER_SIZE..]);
				
				sector =  u32::from(data_block[4]) << 16 
						| u32::from(data_block[5]) << 8 
						| u32::from(data_block[6]);
				remaining -= SECTOR_DATA_SIZE;
			} else {
				if remaining == 0 {
					break;
				}

				let offset = offset + SECTOR_HEADER_SIZE;
				let data_block = &self.data[offset..offset + remaining];

				data.push_back(data_block);
				break;
			}
		}

		data
    }
}