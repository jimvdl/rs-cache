//! Main cache implementation and traits.

use std::{
    path::Path,
    io::Write,
    fs::File,
};

use nom::{
    combinator::cond,
    number::complete::{
        be_u32,
    },
};
use memmap::Mmap;
use crc::crc32;
use whirlpool::{ Whirlpool, Digest };

use crate::{ 
    cksm::{ Checksum, Entry },
    idx::Indices,
    arc::ArchiveRef,
    error::{ ReadError, ParseError }, 
    util,
    codec,
    sec::{
        Sector,
        SectorHeaderSize,
        SECTOR_SIZE,
    },
};

/// Main data name.
pub const MAIN_DATA: &str = "main_file_cache.dat2";
/// Main music data name.
pub const MAIN_MUSIC_DATA: &str = "main_file_cache.dat2m";
/// Reference table id.
pub const REFERENCE_TABLE: u8 = 255;

/// Reads bytes from the cache into the given writer.
/// 
/// # Errors
/// 
/// Returns an `IndexNotFound` error if the specified `index_id` is not a valid `Index`.\
/// Returns an `ArchiveNotFound` error if the specified `archive_id` is not a valid `Archive`.
/// 
/// # Examples
/// 
/// ```
/// use std::io::BufWriter;
/// 
/// # use rscache::Cache;
/// use rscache::ReadIntoWriter;
/// 
/// # fn main() -> rscache::Result<()> {
/// let cache = Cache::new("./data/osrs_cache")?;
/// 
/// let index_id = 2; // Config index
/// let archive_id = 10; // Random archive.
/// 
/// let mut writer = BufWriter::new(Vec::new());
/// cache.read_into_writer(index_id, archive_id, &mut writer)?;
/// # Ok(())
/// # }
/// ```
pub trait ReadIntoWriter {
    fn read_into_writer<W: Write>(
        &self, 
        index_id: u8, 
        archive_id: u32, 
        writer: &mut W
    ) -> crate::Result<()>;
}

/// Main cache struct providing basic utilities.
#[derive(Debug)]
pub struct Cache {
    data: Mmap,
    indices: Indices,
}

impl Cache {
    /// Constructs a new `Cache`.
    ///
    /// # Errors
    /// 
    /// If this function encounters any form of I/O or other error, a `CacheError`
    /// is returned which wraps the underlying error.
    /// 
    /// # Examples
    ///
    /// ```
    /// use rscache::Cache;
    /// # fn main() -> rscache::Result<()> {
    /// 
    /// let cache = Cache::new("./data/osrs_cache")?;
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn new<P: AsRef<Path>>(path: P) -> crate::Result<Self> {
        let path = path.as_ref();
        let main_file = File::open(path.join(MAIN_DATA))?;

        let data = unsafe { Mmap::map(&main_file)? };
        let indices = Indices::new(path, &data)?;

        Ok(Self { data, indices })
    }

    /// Reads from the internal data.
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
    /// 
    /// ```
    /// # use rscache::Cache;
    /// # fn main() -> rscache::Result<()> {
    /// let cache = Cache::new("./data/osrs_cache")?;
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
        let index = self.indices.get(&index_id)
            .ok_or(ReadError::IndexNotFound(index_id))?;

        let archive = index.archive_refs().get(&archive_id)
            .ok_or(ReadError::ArchiveNotFound(index_id, archive_id))?;

        let mut buffer = Vec::with_capacity(archive.length);
        self.data.read_internal(archive, &mut buffer)?;

        Ok(buffer)
    }

    #[inline]
    pub fn read_archive(&self, archive: &ArchiveRef) -> crate::Result<Vec<u8>> {
        self.read(archive.index_id, archive.id)
    }

    /// Creates a `Checksum` which can be used to validate the cache data
    /// that the client received during the update protocol.
    /// 
    /// NOTE: The RuneScape client doesn't have a valid crc for index 16.
    /// This checksum sets the crc and version for index 16 both to 0.
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
    /// # let cache = Cache::new("./data/osrs_cache")?;
    /// let checksum = cache.create_checksum()?;
    /// #    Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn create_checksum(&self) -> crate::Result<Checksum> {
        let mut checksum = Checksum::new(self.index_count());

        for index_id in 0..self.index_count() as u32 {
            if let Ok(buffer) = self.read(REFERENCE_TABLE, index_id) {	
                if !buffer.is_empty() && index_id != 47 {
                    let data = codec::decode(&buffer)?;
                    
                    let (_, version) = cond(data[0] >= 6, be_u32)(&data[1..5])?;
                    let version = version.unwrap_or(0);

                    let mut hasher = Whirlpool::new();
                    hasher.update(&buffer);
                    let hash = hasher.finalize().as_slice().to_vec();

                    checksum.push(
                        Entry { 
                            crc: crc32::checksum_ieee(&buffer), 
                            version,
                            hash
                        }
                    );
                } else {
                    checksum.push(Entry::default());
                }
            };
        }

        Ok(checksum)
    }

    #[inline]
    pub fn huffman_table(&self) -> crate::Result<Vec<u8>> {
        let index_id = 10;

        let archive = self.archive_by_name(index_id, "huffman")?;
        let buffer = self.read_archive(archive)?;
        
        codec::decode(&buffer)
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
    /// # use rscache::{ Cache, codec };
    /// # fn main() -> rscache::Result<()> {
    /// # let cache = Cache::new("./data/osrs_cache")?;
    /// let index_id = 10;
    /// let archive = cache.archive_by_name(index_id, "huffman")?;
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn archive_by_name<T: AsRef<str>>(&self, index_id: u8, name: T) -> crate::Result<&ArchiveRef> {
        let index = self.indices.get(&index_id)
            .ok_or(ReadError::IndexNotFound(index_id))?;
        
        let hash = util::djd2::hash(&name);
        
        let archive = index.archives().iter()
            .find(|archive| archive.name_hash == hash)
            .ok_or_else(|| ReadError::NameNotInArchive(hash, name.as_ref().into(), index_id))?;

        let archive_ref = index.archive_refs()
            .get(&archive.id)
            .ok_or(ReadError::ArchiveNotFound(index_id, archive.id))?;

        Ok(archive_ref)
    }

    /// Simply returns the index count, by getting the `len()` of 
    /// the underlying `indices` vector.
    /// 
    /// # Examples
    /// 
    /// ```
    /// # use rscache::Cache;
    /// # fn main() -> rscache::Result<()> {
    /// # let cache = Cache::new("./data/osrs_cache")?;
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

    #[inline]
    pub const fn indices(&self) -> &Indices {
        &self.indices
    }
}

impl ReadIntoWriter for Cache {
    #[inline]
    fn read_into_writer<W: Write>(
        &self, 
        index_id: u8,
        archive_id: u32, 
        writer: &mut W
    ) -> crate::Result<()> {
        let index = self.indices.get(&index_id)
            .ok_or(ReadError::IndexNotFound(index_id))?;

        let archive = index.archive_refs().get(&archive_id)
            .ok_or(ReadError::ArchiveNotFound(index_id, archive_id))?;
            
        self.data.read_internal(archive, writer)
    }
}

pub trait ReadInternal {
    fn read_internal<W: Write>(
        &self, 
        archive: 
        &ArchiveRef, 
        writer: &mut W
    ) -> crate::Result<()>;
}

impl ReadInternal for Mmap {
    #[inline]
    fn read_internal<W: Write>(
        &self, 
        archive: &ArchiveRef, 
        writer: &mut W
    ) -> crate::Result<()> {
        let header_size = SectorHeaderSize::from_archive(archive);
        let (header_len, data_len) = header_size.clone().into();
        let mut current_sector = archive.sector;
        let mut remaining = archive.length;
        let mut chunk = 0;

        loop {
            let offset = current_sector as usize * SECTOR_SIZE;
            
            if remaining >= data_len {
                let data_block = &self[offset..offset + SECTOR_SIZE];
                
                match Sector::new(data_block, &header_size) {
                    Ok(sector) => {
                        sector.header.validate(archive.id, chunk, archive.index_id)?;
                        current_sector = sector.header.next;
                        writer.write_all(sector.data_block)?;
                    },
                    Err(_) => return Err(ParseError::Sector(archive.sector).into())
                };

                remaining -= data_len;
            } else {
                if remaining == 0 { break; }

                let data_block = &self[offset..offset + remaining + header_len];

                match Sector::new(data_block, &header_size) {
                    Ok(sector) => {
                        sector.header.validate(archive.id, chunk, archive.index_id)?;
                        writer.write_all(sector.data_block)?;

                        break;
                    },
                    Err(_) => return Err(ParseError::Sector(archive.sector).into())
                };
            }

            chunk += 1;
        }

        Ok(())
    }
}