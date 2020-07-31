mod main_data;
mod index;
pub mod archive;

use main_data::MainData;
use index::{ Index };
use archive::Archive;

use crate::{
    error::ReadError,
    Checksum,
    checksum::Entry,
    codec
};

use crc::crc32;

use std::{
    path::Path,
    io::{ self, Read },
    fs::File,
    collections::HashMap,
};

type IndexId = u8;
type ArchiveId = u16;

pub const MAIN_FILE_CACHE_DAT: &str = "main_file_cache.dat2";
pub const MAIN_FILE_CACHE_IDX: &str = "main_file_cache.idx";

/// Main struct which provides basic cache utilities and interactions.
#[derive(Clone, Debug, Default)]
pub struct Cache {
    main_data: MainData,
	indices: HashMap<IndexId, Index>
}

impl Cache {
    /// Constructs a new `Cache`.
    ///
    /// The cache loads every file into memory.
    ///
    /// # Errors
    /// 
    /// If this function encounters any form of I/O or other error, a `CacheError`
    /// is returned which wrapps the underlying error.
    /// 
    /// # Examples
    ///
    /// ```
    /// use rscache::Cache;
    /// 
    /// let cache = Cache::new("path/to/cache");
    /// ```
    #[inline]
    pub fn new<P: AsRef<Path>>(path: P) -> crate::Result<Self> {
        let path = path.as_ref();

        let main_data = load_main_data(path)?;
        let indices = load_indices(path)?;

        Ok(Self { main_data, indices })
    }

    /// Reads from the internal `main_file_cache.dat2` buffer.
    /// 
    /// A lookup is performed on the specified index to find the sector id and the total length
    /// of the buffer that needs to be read from the `main_file_cache.dat2` buffer.
    /// 
    /// If the lookup is successfull the data is gathered into a `LinkedList<&[u8]>`.
    /// 
    /// # Errors
    /// 
    /// Returns an `IndexNotFound` error if the specified `index_id` is not a valid `Index`.\
    /// Returns an `ArchiveNotFound` error if the specified `archive_id` is not a valid `Archive`.
    /// 
    /// # Examples
    /// ```
    /// # use rscache::Cache;
    /// # fn main() -> rscache::Result<()> {
    /// # let path = "./data/cache";
    /// let cache = Cache::new(path)?;
    /// 
    /// let index_id = 2; // Config index
    /// let archive_id = 10; // Random archive.
    /// 
    /// let buffer = cache.read(index_id, archive_id)?;
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn read(&self, index_id: IndexId, archive_id: ArchiveId) -> Result<Vec<u8>, ReadError> {
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
    /// # use rscache::Cache;
    /// # fn main() -> rscache::Result<()> {
    /// # let path = "./data/cache";
    /// # let cache = Cache::new(path)?;
    /// let checksum = cache.create_checksum()?;
    /// #    Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn create_checksum(&self) -> crate::Result<Checksum> {
        let mut checksum = Checksum::new();

        for index_id in 0..self.index_count() as u16 {
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
                        revision: index::version(&data),
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
    /// # use rscache::Cache;
    /// # struct Huffman;
    /// # impl Huffman {
    /// #   pub fn new(buffer: Vec<u8>) -> Self { Self {} }
    /// # }
    /// # fn main() -> rscache::Result<()> {
    /// # let cache = Cache::new("./data/cache")?;
    /// let huffman_table = cache.huffman_table()?;
    /// let huffman = Huffman::new(huffman_table);
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn huffman_table(&self) -> crate::Result<Vec<u8>> {
        let index_id = 10;

        let archive = self.archive_by_name(index_id, "huffman")?;
        let buffer = &self.main_data.read(archive.sector, archive.length);
		
		Ok(codec::decode(&mut buffer.as_slice())?)
    }

	fn archive_by_name(&self, index_id: IndexId, name: &str) -> crate::Result<Archive> {
        let index = match self.indices.get(&index_id) {
            Some(index) => index,
            None => return Err(ReadError::IndexNotFound(index_id).into())
        };
        let identifier = crate::djd2::hash(name);

        let mut buffer = &self.read(255, index_id as u16)?.to_vec()[..];
        let data = &codec::decode(&mut buffer)?[..];

        let archives = archive::parse(data)?;

        for archive_data in archives {
            if archive_data.identifier == identifier {
                match index.archive(archive_data.id) {
                    Some(archive) => return Ok(*archive),
                    None => return Err(
                        ReadError::ArchiveNotFound(index_id, archive_data.id).into()
                    ),
                }
            }
        }

        Err(ReadError::NameNotInArchive(identifier, name.to_owned(), index_id).into())
    }

    /// Simply returns the index count, by getting the `len()` of 
    /// the underlying `indices` vector.
    /// 
    /// # Examples
    /// 
    /// ```
    /// # use rscache::Cache;
    /// # fn main() -> rscache::Result<()> {
    /// # let cache = Cache::new("./data/cache")?;
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

fn load_main_data(path: &Path) -> io::Result<MainData> {
	let mut main_file = File::open(path.join(MAIN_FILE_CACHE_DAT))?;
	let mut buffer = Vec::new();
	main_file.read_to_end(&mut buffer)?;

	Ok(MainData::new(buffer))
}

fn load_indices(path: &Path) -> crate::Result<HashMap<IndexId, Index>> {
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