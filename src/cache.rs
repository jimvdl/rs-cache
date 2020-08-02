use std::{ 
    path::Path,
    collections::HashMap,
};

use crate::{ 
    store::Store, 
    cksm::{ Checksum, Entry },
    idx::Index,
    arc::{ self, Archive },
    error::ReadError, 
    util,
    codec,
    idx,
};

use crc::crc32;

pub const MAIN_DATA: &str = "main_file_cache.dat2";
pub const MAIN_MUSIC_DATA: &str = "main_file_cache.dat2m";
pub const IDX_PREFIX: &str = "main_file_cache.idx";

pub trait CacheCore: CacheRead + Sized {
    fn new<P: AsRef<Path>>(path: P) -> crate::Result<Self>;
}

pub trait CacheRead {
    fn read(&self, index_id: u8, archive_id: u32) -> crate::Result<Vec<u8>>;
}

pub struct Cache<T: Store> {
	store: T,
	indices: HashMap<u8, Index>
}

impl<T: Store> Cache<T> {
    pub fn new<P: AsRef<Path>>(path: P) -> crate::Result<Self> {
        CacheCore::new(path)
    }

    pub fn read(&self, index_id: u8, archive_id: u32) -> crate::Result<Vec<u8>> {
        CacheRead::read(self, index_id, archive_id)
    }

    pub fn create_checksum(&self) -> crate::Result<Checksum> {
        let mut checksum = Checksum::new();

        for index_id in 0..self.index_count() as u32 {
            if index_id == 16 {
                checksum.push(Entry { crc: 0, revision: 0 });
                continue;
            }

            if let Ok(buffer) = &self.read(255, index_id) {	
                let buffer = buffer.to_vec();

                if !buffer.is_empty() {
                    let mut buf = buffer[..].as_ref();
                    let data = codec::decode(&mut buf)?;

                    checksum.push(Entry { 
                        crc: crc32::checksum_ieee(&buffer), 
                        revision: idx::version(&data),
                    });
                }
            };
        }

        Ok(checksum)
    }

    pub fn huffman_table(&self) -> crate::Result<Vec<u8>> {
        let index_id = 10;

        let archive = self.archive_by_name(index_id, "huffman")?;
        let mut buffer: &[u8] = &self.store.read(&archive);
		
		Ok(codec::decode(&mut buffer)?)
    }

    fn archive_by_name(&self, index_id: u8, name: &str) -> crate::Result<Archive> {
        let index = match self.indices.get(&index_id) {
            Some(index) => index,
            None => return Err(ReadError::IndexNotFound(index_id).into())
        };
        let identifier = util::djd2::hash(name);

        let mut buffer: &[u8] = &self.read(255, index_id as u32)?;
        let data = &codec::decode(&mut buffer)?[..];

        let archives = arc::parse(data)?;

        for archive_data in archives {
            if archive_data.identifier == identifier {
                match index.archive(archive_data.id as u32) {
                    Some(archive) => return Ok(*archive),
                    None => return Err(
                        ReadError::ArchiveNotFound(index_id, archive_data.id as u32).into()
                    ),
                }
            }
        }

        Err(ReadError::NameNotInArchive(identifier, name.to_owned(), index_id).into())
    }

    pub fn index_count(&self) -> usize {
        self.indices.len()
    }
}

impl<T: Store> CacheCore for Cache<T> {
    fn new<P: AsRef<Path>>(path: P) -> crate::Result<Self> {
        let path = path.as_ref();

        let store = util::load_store(path)?;
        let indices = util::load_indices(path)?;

        Ok(Self { store, indices })
    }
}

impl<T: Store> CacheRead for Cache<T> {
    fn read(&self, index_id: u8, archive_id: u32) -> crate::Result<Vec<u8>> {
        let index = match self.indices.get(&index_id) {
            Some(index) => index,
            None => return Err(ReadError::IndexNotFound(index_id).into())
        };

        let archive = match index.archive(archive_id) {
            Some(archive) => archive,
            None => return Err(ReadError::ArchiveNotFound(index_id, archive_id).into())
        };

        Ok(self.store.read(archive))
    }
}