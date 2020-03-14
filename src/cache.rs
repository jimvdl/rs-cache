mod main_data;
mod index;
mod archive;

use main_data::MainData;
use index::{ Index };
use archive::{ Archive, ArchiveData };

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

#[derive(Clone, Debug, Default)]
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

        let archive = match index.archive(archive_id) {
            Some(archive) => archive,
            None => return Err(ReadError::ArchiveNotFound(index_id, archive_id))
        };

        Ok(self.main_data.read(archive.sector, archive.length))
    }

    #[inline]
    pub fn create_checksum(&self) -> Result<Checksum, CacheError> {
        let mut checksum = Checksum::new();

        for index_id in 0..self.index_count() as u8 {
            if index_id == 16 {
                checksum.push(Entry { crc: 0, revision: 0 });
                continue;
            }

            if let Ok(buffer) = &self.read(255, index_id) {	
                let buffer = buffer.to_vec();

                if !buffer.is_empty() {
                    let mut buf = buffer[..].as_ref();
                    let container = Container::decompress(&mut buf)?;
                    let container_data = container.data();

                    checksum.push(Entry { 
                        crc: crc32::checksum_ieee(&buffer), 
                        revision: index_version(container_data)?
                    });
                }
            };
        }

        Ok(checksum)
    }

    #[inline]
    pub fn huffman_table(&self) -> Result<Vec<u8>, CacheError> {
        let index_id = 10;
        let archive = self.archive_by_name(index_id, "huffman")?;

        let mut buffer = &self.main_data.read(archive.sector, archive.length).to_vec()[..];
		let container = Container::decompress(&mut buffer)?;
		
		Ok(container.data().to_vec())
    }

    #[inline]
	pub fn archive_by_name(&self, index_id: u8, name: &str) -> Result<Archive, CacheError> {
        let index = match self.indices.get(&index_id) {
            Some(index) => index,
            None => return Err(ReadError::IndexNotFound(index_id).into())
        };
        let identifier = djd2::hash(name);

        let mut buffer = &self.read(255, index_id)?.to_vec()[..];
        let container = Container::decompress(&mut buffer)?;

        let archives = ArchiveData::decode(&mut container.data())?;

        for archive_data in archives {
            if archive_data.identifier == identifier {
                match index.archive(archive_data.id as u8) {
                    Some(archive) => return Ok(*archive),
                    None => return Err(ReadError::ArchiveNotFound(index_id, archive_data.id as u8).into()),
                }
            }
        }

        Err(ReadError::ArchiveNotFound(index_id, 0).into())
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

fn index_version(buffer: &[u8]) -> io::Result<u32> {
    let format = buffer[0];

    let version = if format >= 6 {
        u32::from_be_bytes([buffer[1], buffer[2], buffer[3], buffer[4]])
    } else {
        0
    };

    Ok(version)
}

mod djd2 {
    pub fn hash(string: &str) -> i32 {
        let mut hash = 0;
        for index in 0..string.len() {
            hash = string.chars().nth(index).unwrap() as i32 + ((hash << 5) - hash);
        }
        hash
    }
}