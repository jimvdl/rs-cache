//! [Oldschool RuneScape](https://oldschool.runescape.com/) 
//! & [RuneScape 3](https://www.runescape.com/) cache api for basic
//! and simple cache interactions.
//! 
//! # Features
//! 
//! Currently rs-cache offers limited support for OSRS & RS3 with the features listed below.
//! 
//! Note: the public api of this crate is still evolving, it might contain bugs or miss features.
//! 
//! The following features are currently provided:
//! - Reading from the cache.
//! - Huffman buffer access.
//! - Checksum with simple-to-use validation.
//! - Compression and decompression:
//!   - [Gzip](https://crates.io/crates/flate2)
//!   - [Bzip2](https://crates.io/crates/bzip2)
//!   - [LZMA](https://crates.io/crates/lzma_rs)
//! - OSRS Loaders
//!   - [`ItemLoader`](ldr/osrs/struct.ItemLoader.html)
//!   - [`NpcLoader`](ldr/osrs/struct.NpcLoader.html)
//!   - [`ObjectLoader`](ldr/osrs/struct.ObjectLoader.html)
//! - RS3 Loaders
//!   - [`ItemLoader`](ldr/rs3/struct.ItemLoader.html)
//! - Utilities
//!   - [`Huffman`](util/struct.Huffman.html) decompressor.
//!   - [`Isaac`](util/struct.IsaacRand.html) randomizer.
//!   - Xtea cipher.
//! 
//! Feature to be implemented in the future: 
//! - Writing data to the cache.
//! 
//! # Quick Start
//! 
//! The quickest and easiest way to get started is by using the
//! [`Cache`](struct.Cache.html).
//! 
//! ```
//! use rscache::Cache;
//! 
//! # fn main() -> rscache::Result<()> {
//! let cache = Cache::new("./data/osrs_cache")?;
//! 
//! let index_id = 2; // Config index.
//! let archive_id = 10; // Archive containing item definitions.
//! 
//! let buffer: Vec<u8> = cache.read(index_id, archive_id)?;
//! # Ok(())
//! # }
//! ```

#![deny(clippy::all, clippy::nursery)]

#![warn(
    clippy::clone_on_ref_ptr, 
    clippy::redundant_clone, 
    clippy::default_trait_access, 
    clippy::expl_impl_clone_on_copy,
    clippy::explicit_into_iter_loop, 
    clippy::explicit_iter_loop, 
    clippy::manual_filter_map, 
    clippy::filter_map_next, 
    clippy::manual_find_map, 
    clippy::get_unwrap,
    clippy::items_after_statements, 
    clippy::large_digit_groups, 
    clippy::map_flatten, 
    clippy::match_same_arms, 
    clippy::maybe_infinite_iter, 
    clippy::mem_forget,
    clippy::missing_inline_in_public_items, 
    clippy::multiple_inherent_impl, 
    clippy::mut_mut, 
    clippy::needless_continue,
    clippy::needless_pass_by_value, 
    clippy::map_unwrap_or, 
    clippy::unused_self, 
    clippy::similar_names, 
    clippy::single_match_else, 
    clippy::too_many_lines, 
    clippy::type_repetition_in_bounds,
    clippy::unseparated_literal_suffix, 
    clippy::used_underscore_binding,
    clippy::should_implement_trait,
    clippy::no_effect,
)]

// TODO: determine what belongs in public api
// TODO: make serde a feature
// TODO: add rust-version to [package]
// TODO: add rs3 feature, you can drop in replace using use as syntax (use std::sync::Arc as SyncRc)
// TODO: update min rust version badge + remove docs badge and license badge
// TODO: document how to make your own loader in ldr.rs
// TODO: document unsafe memmap
// TODO: maybe check load function names on map and location loader to reflect that they need mut for lazy caching.
// TODO: remove custom loader test and make it documentation instead

#[macro_use]
pub mod util;
pub mod checksum;
pub mod extension;
pub mod parse;
pub mod error;
pub mod codec;
pub mod definition;
pub mod loader;
mod index;
mod archive;
mod sector;

#[doc(inline)]
pub use error::{ Result, CacheError };

/// Main data name.
pub(crate) const MAIN_DATA: &str = "main_file_cache.dat2";
/// Reference table id.
pub(crate) const REFERENCE_TABLE: u8 = 255;

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
#[cfg(feature = "rs3")]
use whirlpool::{ Whirlpool, Digest };

use crate::{ 
    checksum::{ Checksum, Entry },
    index::Indices,
    archive::ArchiveRef,
    error::{ ReadError, ParseError }, 
    sector::{
        Sector,
        SectorHeaderSize,
        SECTOR_SIZE,
    },
};

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

        let archive = index.archive_refs.get(&archive_id)
            .ok_or(ReadError::ArchiveNotFound(index_id, archive_id))?;

        let mut buffer = Vec::with_capacity(archive.length);
        self.data.read_internal(archive, &mut buffer)?;

        Ok(buffer)
    }

    #[inline]
    pub fn read_archive(&self, archive: &ArchiveRef) -> crate::Result<Vec<u8>> {
        self.read(archive.index_id, archive.id)
    }

    // FIXME
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
    #[inline]
    pub fn read_into_writer<W: Write>(
        &self, 
        index_id: u8,
        archive_id: u32, 
        writer: &mut W
    ) -> crate::Result<()> {
        let index = self.indices.get(&index_id)
            .ok_or(ReadError::IndexNotFound(index_id))?;

        let archive = index.archive_refs.get(&archive_id)
            .ok_or(ReadError::ArchiveNotFound(index_id, archive_id))?;
            
        self.data.read_internal(archive, writer)
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

                    #[cfg(feature = "rs3")]
                    let hash = {
                        let mut hasher = Whirlpool::new();
                        hasher.update(&buffer);
                        hasher.finalize().as_slice().to_vec()
                    };

                    checksum.push(
                        Entry { 
                            crc: crc32::checksum_ieee(&buffer), 
                            version,
                            #[cfg(feature = "rs3")]
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
        
        let archive = index.archives.iter()
            .find(|archive| archive.name_hash == hash)
            .ok_or_else(|| ReadError::NameNotInArchive(hash, name.as_ref().into(), index_id))?;

        let archive_ref = index.archive_refs
            .get(&archive.id)
            .ok_or(ReadError::ArchiveNotFound(index_id, archive.id))?;

        Ok(archive_ref)
    }

    #[inline]
    pub fn index_count(&self) -> usize {
        self.indices.len()
    }

    #[inline]
    pub const fn indices(&self) -> &Indices {
        &self.indices
    }
}

pub(crate) trait ReadInternal {
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