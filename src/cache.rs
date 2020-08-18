//! Main cache implementation and traits.

use std::{ 
    path::Path,
    collections::HashMap,
};

use nom::{
    combinator::cond,
	number::complete::{
		be_u32,
    },
};

use crate::{ 
    store::Store, 
    cksm::{ Checksum, Entry },
    idx::Index,
    arc::{ self, Archive },
    error::ReadError, 
    util,
    codec,
};

use crc::crc32;

pub const MAIN_DATA: &str = "main_file_cache.dat2";
pub const MAIN_MUSIC_DATA: &str = "main_file_cache.dat2m";
pub const IDX_PREFIX: &str = "main_file_cache.idx";
pub const REFERENCE_TABLE: u8 = 255;

/// The core of a cache.
pub trait CacheCore: CacheRead + Sized {
    fn new<P: AsRef<Path>>(path: P) -> crate::Result<Self>;
}

/// The read functionality of a cache.
pub trait CacheRead {
    fn read(&self, index_id: u8, archive_id: u32) -> crate::Result<Vec<u8>>;
}

/// Main cache struct providing basic utilities.
pub struct Cache<S: Store> {
	store: S,
	indices: HashMap<u8, Index>
}

impl<S: Store> Cache<S> {
    /// Constructs a new `Cache<S>` with the given store.
    ///
    /// # Errors
    /// 
    /// If this function encounters any form of I/O or other error, a `CacheError`
    /// is returned which wrapps the underlying error.
    /// 
    /// # Examples
    ///
    /// ```
    /// use rscache::{ Cache, store::MemoryStore };
    /// # fn main() -> rscache::Result<()> {
    /// 
    /// let cache: Cache<MemoryStore> = Cache::new("./data/cache")?;
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn new<P: AsRef<Path>>(path: P) -> crate::Result<Self> {
        CacheCore::new(path)
    }

    /// Reads from the internal store.
    /// 
    /// A lookup is performed on the specified index to find the sector id and the total length
    /// of the buffer that needs to be read from the `main_file_cache.dat2`.
    /// 
    /// If the lookup is successfull the data is gathered into a `Vec<u8>`.
    /// 
    /// # Errors
    /// 
    /// Returns an `IndexNotFound` error if the specified `index_id` is not a valid `Index`.\
    /// Returns an `ArchiveNotFound` error if the specified `archive_id` is not a valid `Archive`.
    /// 
    /// # Examples
    /// ```
    /// # use rscache::{ Cache, store::MemoryStore };
    /// # fn main() -> rscache::Result<()> {
    /// # let path = "./data/cache";
    /// let cache: Cache<MemoryStore> = Cache::new(path)?;
    /// 
    /// let index_id = 2; // Config index
    /// let archive_id = 10; // Random archive.
    /// 
    /// let buffer = cache.read(index_id, archive_id)?;
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn read(&self, index_id: u8, archive_id: u32) -> crate::Result<Vec<u8>> {
        CacheRead::read(self, index_id, archive_id)
    }

    /// Creates a `Checksum` which can be used to validate the cache data
    /// that the client received during the update protocol.
    /// 
    /// NOTE: The RuneScape client doesn't have a valid crc for index 16.
    /// This checksum sets the crc and revision for index 16 both to 0.
    /// The crc for index 16 should be skipped.
    /// 
    /// # Errors
    /// 
    /// Returns an error when a buffer read from the reference
    /// table could not be decoded / decompressed.
    /// 
    /// # Examples
    /// 
    /// ```
    /// # use rscache::{ Cache, store::MemoryStore };
    /// # fn main() -> rscache::Result<()> {
    /// # let path = "./data/cache";
    /// # let cache: Cache<MemoryStore> = Cache::new(path)?;
    /// let checksum = cache.create_checksum()?;
    /// #    Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn create_checksum(&self) -> crate::Result<Checksum> {
        let mut checksum = Checksum::new();

        for index_id in 0..self.index_count() as u32 {
            if index_id == 16 {
                checksum.push(Entry { crc: 0, revision: 0 });
                continue;
            }

            if let Ok(buffer) = self.read(REFERENCE_TABLE, index_id) {	
                if !buffer.is_empty() {
                    let data = codec::decode(&buffer)?;

                    let (_, version) = cond(data[0] >= 6, be_u32)(&data[1..5])?;
                    let version = if let Some(version) = version { version } else { 0 };

                    checksum.push(Entry { 
                        crc: crc32::checksum_ieee(&buffer), 
                        revision: version,
                    });
                }
            };
        }

        Ok(checksum)
    }

    /// Constructs a buffer which contains the huffman table.
    /// 
    /// # Errors
    /// 
    /// Returns an error if the huffman archive could not be found or 
    /// if the decode / decompression of the huffman table failed.
    /// 
    /// # Examples
    /// ```
    /// # use rscache::{ Cache, store::MemoryStore };
    /// # struct Huffman;
    /// # impl Huffman {
    /// #   pub fn new(buffer: Vec<u8>) -> Self { Self {} }
    /// # }
    /// # fn main() -> rscache::Result<()> {
    /// # let cache: Cache<MemoryStore> = Cache::new("./data/cache")?;
    /// let huffman_table = cache.huffman_table()?;
    /// let huffman = Huffman::new(huffman_table);
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn huffman_table(&self) -> crate::Result<Vec<u8>> {
        let index_id = 10;

        let archive = self.archive_by_name(index_id, "huffman")?;
        let buffer = self.store.read(&archive)?;
		
		Ok(codec::decode(&buffer)?)
    }

    /// Searches for the archive which contains the given name hash in the given
    /// index_id.
    /// 
    /// # Errors
    /// 
    /// Panics if the string couldn't be hashed by the djd2 hasher.
    /// 
    /// Returns an `IndexNotFound` error if the specified `index_id` is not a valid `Index`.\
    /// Returns an `ArchiveNotFound` error if the specified `archive_id` is not a valid `Archive`.\
    /// Returns an `NameNotInArchive` error if the `name` hash is not present in this archive.
    /// 
    /// # Examples
    /// ```
    /// # use rscache::{ Cache, store::MemoryStore, codec };
    /// # fn main() -> rscache::Result<()> {
    /// # let path = "./data/cache";
    /// # let cache: Cache<MemoryStore> = Cache::new(path)?;
    /// let index_id = 10;
    /// let archive = cache.archive_by_name(index_id, "huffman")?;
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn archive_by_name(&self, index_id: u8, name: &str) -> crate::Result<Archive> {
        let index = match self.indices.get(&index_id) {
            Some(index) => index,
            None => return Err(ReadError::IndexNotFound(index_id).into())
        };
        let hash = util::djd2::hash(name);

        let buffer = self.read(REFERENCE_TABLE, index_id as u32)?;
        let data = codec::decode(&buffer)?;

        let archives = arc::parse_archive_data(&data)?;

        for archive_data in archives {
            if archive_data.hash == hash {
                match index.archives.get(&(archive_data.id as u32)) {
                    Some(archive) => return Ok(*archive),
                    None => return Err(
                        ReadError::ArchiveNotFound(index_id, archive_data.id as u32).into()
                    ),
                }
            }
        }

        Err(ReadError::NameNotInArchive(hash, name.to_owned(), index_id).into())
    }

    /// Simply returns the index count, by getting the `len()` of 
    /// the underlying `indices` vector.
    /// 
    /// # Examples
    /// 
    /// ```
    /// # use rscache::{ Cache, store::MemoryStore };
    /// # fn main() -> rscache::Result<()> {
    /// # let cache: Cache<MemoryStore> = Cache::new("./data/cache")?;
    /// for index in 0..cache.index_count() {
    ///     // ...
    /// }
    /// 
    /// assert_eq!(22, cache.index_count());
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn index_count(&self) -> usize {
        self.indices.len()
    }
}

impl<S: Store> CacheCore for Cache<S> {
    #[inline]
    fn new<P: AsRef<Path>>(path: P) -> crate::Result<Self> {
        let path = path.as_ref();

        let store = util::load_store(path)?;
        let indices = util::load_indices(path)?;

        Ok(Self { store, indices })
    }
}

impl<S: Store> CacheRead for Cache<S> {
    #[inline]
    fn read(&self, index_id: u8, archive_id: u32) -> crate::Result<Vec<u8>> {
        let index = match self.indices.get(&index_id) {
            Some(index) => index,
            None => return Err(ReadError::IndexNotFound(index_id).into())
        };

        let archive = match index.archives.get(&archive_id) {
            Some(archive) => archive,
            None => return Err(ReadError::ArchiveNotFound(index_id, archive_id).into())
        };

        Ok(self.store.read(archive)?)
    }
}