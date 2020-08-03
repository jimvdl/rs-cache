use std::collections::HashMap;

use crate::arc::Archive;

pub const IDX_LENGTH: usize = 6;

#[derive(Clone, Debug, Default)]
pub struct Index {
	archives: HashMap<u32, Archive>,
}

impl Index {
	#[inline]
    pub fn new(buffer: &[u8]) -> Self {
		let mut archives = HashMap::new();

		for (id, archive_metadata) in buffer.chunks_exact(IDX_LENGTH).enumerate() {
			let id = id as u32;

			let archive = parse_archive(id, archive_metadata);
			archives.insert(id, archive);
		}

        Self { archives }
	}

	#[inline]
	pub fn archive(&self, archive_id: u32) -> Option<&Archive> {
		self.archives.get(&archive_id)
	}
}

fn parse_archive(id: u32, buffer: &[u8]) -> Archive {
	let length = (buffer[0] as usize) << 16 | (buffer[1] as usize) << 8 | (buffer[2] as usize);
	let sector = u32::from(buffer[3]) << 16 | u32::from(buffer[4]) << 8 | u32::from(buffer[5]);

	Archive { id, sector, length }
}

#[inline]
pub fn version(buffer: &[u8]) -> u32 {
    let format = buffer[0];

    if format >= 6 {
        u32::from_be_bytes([buffer[1], buffer[2], buffer[3], buffer[4]])
    } else {
        0
    }
}