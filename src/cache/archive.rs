use crate::CacheError;

use std::io::Read;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Archive {
    pub sector: u32,
    pub length: usize
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct ArchiveData {
    pub id: u16,
    pub identifier: i32,
    crc: u32,
    revision: u32
}

impl Archive {
    pub const fn new(sector: u32, length: usize) -> Self {
		Self { sector, length }
	}
}

impl ArchiveData {
    pub fn decode<R: Read>(reader: &mut R) -> Result<Vec<Self>, CacheError> {
        let mut buffer = [0; 1];
        reader.read_exact(&mut buffer)?;
        let protocol = buffer[0];

        if protocol >= 6 {
            reader.take(4);
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

        Ok(archives)
    }
}