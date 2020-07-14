use std::collections::HashMap;

use super::archive::Archive;

pub const INDEX_LENGTH: usize = 6;

#[derive(Clone, Debug, Default)]
pub struct Index {
	archives: HashMap<u16, Archive>,
}

impl Index {
    pub fn new(buffer: &[u8]) -> Self {
		let mut archives = HashMap::new();

		for (id, archive_metadata) in buffer.chunks_exact(INDEX_LENGTH).enumerate() {
			let archive = parse_archive(archive_metadata);
			archives.insert(id as u16, archive);
		}

        Self { archives }
	}

	pub fn archive(&self, archive_id: u16) -> Option<&Archive> {
		self.archives.get(&archive_id)
	}
}

fn parse_archive(buffer: &[u8]) -> Archive {
	let length = (buffer[0] as usize) << 16 | (buffer[1] as usize) << 8 | (buffer[2] as usize);
	let sector = u32::from(buffer[3]) << 16 | u32::from(buffer[4]) << 8 | u32::from(buffer[5]);

	Archive { sector, length }
}