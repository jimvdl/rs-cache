mod main_data;
mod index;
mod archive;

use main_data::MainData;
use index::{ Index };

use crate::{
    errors::ReadError,
    Checksum, CacheError,
    checksum::Entry,
    Container,
    LinkedListExt
};

use crc::crc32;

use std::{
    path::Path,
    io::{ self, Read },
    fs::File,
    collections::{ LinkedList, HashMap },
};

pub const MAIN_FILE_CACHE_DAT: &str = "main_file_cache.dat2";
pub const MAIN_FILE_CACHE_IDX: &str = "main_file_cache.idx";

pub struct Cache {
    main_data: MainData,
	indices: HashMap<u8, Index>
}

impl Cache {
    #[inline]
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, CacheError> {
        let path = path.as_ref();

        let main_data = load_main_data(path)?;
        let indices = load_indices(path)?;

        Ok(Self { main_data, indices })
    }

    #[inline]
    pub fn read(&self, index_id: u8, archive_id: u8) -> Result<LinkedList<&[u8]>, ReadError> {
        let index = match self.indices.get(&index_id) {
            Some(index) => index,
            None => return Err(ReadError::IndexNotFound(index_id))
        };

        let archive = match index.get_archive(archive_id) {
            Some(archive) => archive,
            None => return Err(ReadError::ArchiveNotFound(archive_id, index_id))
        };

        Ok(self.main_data.read(archive.sector, archive.length))
    }

    #[inline]
    pub fn create_checksum(&self) -> Result<Checksum, CacheError> {
        let mut checksum = Checksum::new();
        let ref_table = match self.indices.get(&255) {
            Some(ref_table) => ref_table,
            None => return Err(ReadError::IndexNotFound(255).into()),
        };

        for archive_id in 0..ref_table.archive_count() as u8 {
            if archive_id == 16 {
                checksum.push(Entry { crc: 0, revision: 0 });
                continue;
            }

            if let Ok(buffer) = &self.read(255, archive_id) {	
                let buffer = buffer.to_vec();

                if !buffer.is_empty() {
                    let mut buf = buffer[..].as_ref();
                    let container = Container::decode(&mut buf)?;
                    let container_data = container.data();

                    checksum.push(Entry { 
                        crc: crc32::checksum_ieee(&buffer), 
                        revision: get_index_version(container_data)?
                    });
                }
            };
        }

        Ok(checksum)
    }

    #[inline]
    pub fn huffman_table(&self) -> &Vec<u8> {
        unimplemented!()
    }

    #[inline]
    pub fn index_count(&self) -> usize {
        self.indices.len()
    }
}

fn load_main_data(path: &Path) -> io::Result<MainData> {
	let mut main_file = File::open(path.join(MAIN_FILE_CACHE_DAT))?;
	let mut buffer = Vec::new();
	main_file.read_to_end(&mut buffer)?;

	Ok(MainData::new(buffer))
}

fn load_indices(path: &Path) -> Result<HashMap<u8, Index>, CacheError> {
	let mut indices = HashMap::new();

	for index_id in 0..=255 {
		let path = path.join(format!("{}{}", MAIN_FILE_CACHE_IDX, index_id));

		if path.exists() {
			let mut index = File::open(path)?;
			let mut index_buffer = Vec::new();

			index.read_to_end(&mut index_buffer)?;
			indices.insert(index_id, Index::new(&index_buffer));
		}
	}

	Ok(indices)
}

fn get_index_version(buffer: &[u8]) -> io::Result<u32> {
    let format = buffer[0];

    let version = if format >= 6 {
        u32::from_be_bytes([buffer[1], buffer[2], buffer[3], buffer[4]])
    } else {
        0
    };

    Ok(version)
}