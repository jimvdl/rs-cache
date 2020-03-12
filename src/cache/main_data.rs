use std::collections::LinkedList;

pub const SECTOR_HEADER_SIZE: usize = 8;
pub const SECTOR_DATA_SIZE: usize = 512;
pub const SECTOR_SIZE: usize = SECTOR_HEADER_SIZE + SECTOR_DATA_SIZE;

pub struct MainData {
	pub data: Vec<u8>,
}

impl MainData {
    pub fn new(buffer: Vec<u8>) -> Self {
        Self { data: buffer }
    }
	
	pub fn read(&self, mut sector: u32, size: usize) -> LinkedList<&[u8]> {
		let mut data = LinkedList::new();
		let mut remaining = size;

		loop {
			let current = sector as usize * SECTOR_SIZE;

			if remaining >= SECTOR_SIZE {
				let data_block = &self.data[current..current + SECTOR_SIZE];
				data.push_back(&data_block[SECTOR_HEADER_SIZE..]);

				sector = ((data_block[4] as u32).overflowing_shl(16)).0 | ((data_block[5] as u32).overflowing_shl(8)).0 | data_block[6] as u32;
				remaining -= SECTOR_DATA_SIZE;
			} else {
				if remaining == 0 {
					break;
				}

				data.push_back(&self.data[current + SECTOR_HEADER_SIZE..current + SECTOR_HEADER_SIZE + remaining]);
				break;
			}
		}

		data
    }
}