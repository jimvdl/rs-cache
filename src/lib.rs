//! A high-level immutable API for the RuneScape cache.
//! 
//! This crate provides convenient access to the binary file system of the [Oldschool RuneScape] and [RuneScape 3] caches.
//!
//! The library's API is mainly focussed around reading bytes easily.
//! Therefore it offers a higher level of abstraction then most other libraries. Most cache API's expose a
//! wide variety of internal types to let the user tinker around with the cache in unusual ways.
//! To avoid undefined behavior most internal types are kept private.
//! The goal of this crate is to provide a simple interface for basic reading of valuable data.
//!
//! Note that this crate is still evolving, both OSRS & RS3 are not fully supported/implemented and
//! will probably contain bugs or miss vital features. If this is the case for you then consider [opening
//! an issue].
//! 
//! # Safety
//! 
//! This crate internally uses [memmap] and this is safe because: the RuneScape cache is a read-only binary files ystem 
//! which is never modified by any process, and should never be modified. To ensure the main file is never moved while the
//! cache has memory mapped to it a file handle is kept internally to make access more safe. It is not possible to prevent 
//! parallel access to a certain file and prevent modifications. Therefore file-backed mapped memory is inherently unsafe.
//!
//! # Features
//!
//! The cache's protocol defaults to OSRS. In order to use the RS3 protocol you can enable the _**rs3**_ feature flag.
//! A lot of types derive [serde]'s `Serialize` and `Deserialize`. To enable (de)serialization on
//! most types use the _**serde-derive**_ feature flag.
//!
//! # Quick Start
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
//!
//! # Loaders
//!
//! In order to get [definitions](crate::definition) you can look at the [loaders](crate::loader) this library provides.
//! The loaders use the cache as a dependency to parse in their data and cache the relevant definitions internally.
//! The loader module also tells you how to make a loader if this crate doesn't (yet) provide it. 
//! 
//! Note: Some loaders cache these definitions lazily because of either the size of the data or the
//! performance. The map loader for example is both slow and large so caching is by default lazy.
//! Lazy loaders require mutability.
//!
//! [Oldschool RuneScape]: https://oldschool.runescape.com/
//! [RuneScape 3]: https://www.runescape.com/
//! [opening an issue]: https://github.com/jimvdl/rs-cache/issues/new
//! [serde]: https://crates.io/crates/serde
//! [memmap]: https://crates.io/crates/memmap

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
    clippy::no_effect
)]

// TODO: add rust-version to [package]
// TODO: run cargo outdated when it is fixed for stable 1.56

#[macro_use]
pub mod util;
mod archive;
pub mod checksum;
pub mod codec;
pub mod definition;
pub mod error;
pub mod extension;
mod index;
pub mod loader;
pub mod parse;
mod sector;

#[doc(inline)]
pub use error::{CacheError, Result};

pub(crate) const MAIN_DATA: &str = "main_file_cache.dat2";
pub(crate) const REFERENCE_TABLE: u8 = 255;

use std::{fs::File, io::Write, path::Path};

use crc::crc32;
use memmap::Mmap;
use nom::{combinator::cond, number::complete::be_u32};
#[cfg(feature = "rs3")]
use whirlpool::{Digest, Whirlpool};

use crate::{
    archive::ArchiveRef,
    checksum::{Checksum, Entry},
    error::{ParseError, ReadError},
    index::Indices,
    sector::{Sector, SectorHeaderSize, SECTOR_SIZE},
};

/// A parsed Jagex cache.
#[derive(Debug)]
pub struct Cache {
    main_file: File,
    data: Mmap,
    indices: Indices,
}

impl Cache {
    /// Constructs a new `Cache`.
    ///
    /// Each valid index is parsed and stored, and in turn all archive references as well.
    /// If an index is not present it will simply be skipped.
    /// However, the main data file and reference table both are required.
    ///
    /// # Errors
    ///
    /// If this function encounters any form of I/O or other error, a `CacheError`
    /// is returned which wraps the underlying error.
    #[inline]
    pub fn new<P: AsRef<Path>>(path: P) -> crate::Result<Self> {
        let path = path.as_ref();
        let main_file = File::open(path.join(MAIN_DATA))?;

        let data = unsafe { Mmap::map(&main_file)? };
        let indices = Indices::new(path, &data)?;

        Ok(Self { main_file, data, indices })
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
    #[inline]
    pub fn read(&self, index_id: u8, archive_id: u32) -> crate::Result<Vec<u8>> {
        let index = self
            .indices
            .get(&index_id)
            .ok_or(ReadError::IndexNotFound(index_id))?;

        let archive = index
            .archive_refs
            .get(&archive_id)
            .ok_or(ReadError::ArchiveNotFound(index_id, archive_id))?;

        let mut buffer = Vec::with_capacity(archive.length);
        self.data.read_internal(archive, &mut buffer)?;

        Ok(buffer)
    }

    pub(crate) fn read_archive(&self, archive: &ArchiveRef) -> crate::Result<Vec<u8>> {
        self.read(archive.index_id, archive.id)
    }

    /// Reads bytes from the cache into the given writer.
    ///
    /// This will not allocate a buffer but use the writer instead, see [`read`](Cache::read)
    ///
    /// # Errors
    ///
    /// Returns an `IndexNotFound` error if the specified `index_id` is not a valid `Index`.\
    /// Returns an `ArchiveNotFound` error if the specified `archive_id` is not a valid `Archive`.
    #[inline]
    pub fn read_into_writer<W: Write>(
        &self,
        index_id: u8,
        archive_id: u32,
        writer: &mut W,
    ) -> crate::Result<()> {
        let index = self
            .indices
            .get(&index_id)
            .ok_or(ReadError::IndexNotFound(index_id))?;

        let archive = index
            .archive_refs
            .get(&archive_id)
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

                    checksum.push(Entry {
                        crc: crc32::checksum_ieee(&buffer),
                        version,
                        #[cfg(feature = "rs3")]
                        hash,
                    });
                } else {
                    checksum.push(Entry::default());
                }
            };
        }

        Ok(checksum)
    }

    /// Tries to return the huffman table from the cache.
    ///
    /// This can be used to (de)compress chat messages, see [`Huffman`](crate::util::Huffman).
    #[inline]
    pub fn huffman_table(&self) -> crate::Result<Vec<u8>> {
        let index_id = 10;

        let archive = self.archive_by_name(index_id, "huffman")?;
        let buffer = self.read_archive(archive)?;
        codec::decode(&buffer)
    }

    #[inline]
    pub(crate) fn archive_by_name<T: AsRef<str>>(
        &self,
        index_id: u8,
        name: T,
    ) -> crate::Result<&ArchiveRef> {
        let index = self
            .indices
            .get(&index_id)
            .ok_or(ReadError::IndexNotFound(index_id))?;
        let hash = util::djd2::hash(&name);

        let archive = index
            .archives
            .iter()
            .find(|archive| archive.name_hash == hash)
            .ok_or_else(|| ReadError::NameNotInArchive(hash, name.as_ref().into(), index_id))?;

        let archive_ref = index
            .archive_refs
            .get(&archive.id)
            .ok_or(ReadError::ArchiveNotFound(index_id, archive.id))?;

        Ok(archive_ref)
    }

    #[inline]
    pub fn index_count(&self) -> usize {
        self.indices.len()
    }
}

pub(crate) trait ReadInternal {
    fn read_internal<W: Write>(&self, archive: &ArchiveRef, writer: &mut W) -> crate::Result<()>;
}

impl ReadInternal for Mmap {
    #[inline]
    fn read_internal<W: Write>(&self, archive: &ArchiveRef, writer: &mut W) -> crate::Result<()> {
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
                        sector
                            .header
                            .validate(archive.id, chunk, archive.index_id)?;
                        current_sector = sector.header.next;
                        writer.write_all(sector.data_block)?;
                    }
                    Err(_) => return Err(ParseError::Sector(archive.sector).into()),
                };

                remaining -= data_len;
            } else {
                if remaining == 0 {
                    break;
                }

                let data_block = &self[offset..offset + remaining + header_len];

                match Sector::new(data_block, &header_size) {
                    Ok(sector) => {
                        sector
                            .header
                            .validate(archive.id, chunk, archive.index_id)?;
                        writer.write_all(sector.data_block)?;

                        break;
                    }
                    Err(_) => return Err(ParseError::Sector(archive.sector).into()),
                };
            }

            chunk += 1;
        }

        Ok(())
    }
}
