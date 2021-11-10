//! An immutable, high-level API for the RuneScape cache file system.
//! 
//! This crate provides high performant data reads into the [Oldschool RuneScape] and [RuneScape 3] file systems. 
//! It can read the necessary data to syncronize the client's cache with the server. There are also some 
//! [loaders](#loaders) that give access to definitions from the cache such as items or npcs. 
//! 
//! For read-heavy workloads, a writer can be used to prevent continous buffer allocations.
//! By default every read will allocate a writer with the correct capacity.
//! 
//! RuneScape's chat system uses huffman coding to compress messages. In order to decompress them this library has
//! a [`Huffman`] implementation.
//! 
//! When a RuneScape client sends game packets the id's are encoded and can be decoded with the [`IsaacRand`]
//! implementation. These id's are encoded by the client in a predictable random order which can be reversed if
//! the server has its own `IsaacRand` with the same encoder/decoder keys. These keys are sent by the client
//! on login and are user specific. It will only send encoded packet id's if the packets are game packets.
//! 
//! Note that this crate is still evolving; both OSRS & RS3 are not fully supported/implemented and
//! will probably contain bugs or miss core features. If you require features or find bugs consider [opening
//! an issue].
//! 
//! # Safety
//! 
//! In order to read bytes in a high performant way the cache uses [memmap2]. This can be unsafe because of its potential for
//! _Undefined Behaviour_ when the underlying file is subsequently modified, in or out of process. 
//! Using `Mmap` here is safe because the RuneScape cache is a read-only binary file system. The map will remain valid even
//! after the `File` is dropped, it's completely independent of the `File` used to create it. When the `Cache` is dropped 
//! memory will be subsequently unmapped.
//!
//! # Features
//!
//! The cache's protocol defaults to OSRS. In order to use the RS3 protocol you can enable the `rs3` feature flag.
//! A lot of types derive [serde]'s `Serialize` and `Deserialize`. To enable (de)serialization on any compatible
//! types use the `serde-derive` feature flag.
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
//! [memmap2]: https://crates.io/crates/memmap2
//! [`Huffman`]: crate::util::Huffman
//! [`IsaacRand`]: crate::util::IsaacRand

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
const CRC: Crc<u32> = Crc::<u32>::new(&CRC_32_ISO_HDLC);

use std::{fs::File, io::Write, path::Path};

use crc::{Crc, CRC_32_ISO_HDLC};
use memmap2::Mmap;
use nom::{combinator::cond, number::complete::be_u32};
#[cfg(feature = "rs3")]
use whirlpool::{Digest, Whirlpool};

use crate::{
    archive::ArchiveRef,
    checksum::{Checksum, Entry},
    error::{ParseError, ReadError},
    index::Indices,
    sector::{Sector, SECTOR_SIZE},
};

/// A parsed Jagex cache.
#[derive(Debug)]
pub struct Cache {
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

        assert_eq!(buffer.len(), archive.length);

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

                    let mut digest = CRC.digest();
                    digest.update(&buffer);

                    checksum.push(Entry {
                        crc: digest.finalize(),
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
        let mut current_sector = archive.sector;
        let (header_size, chunks) = archive.chunks();

        for (chunk, data_len) in chunks.enumerate() {
            let offset = current_sector * SECTOR_SIZE;

            let data_block = &self[offset..offset + data_len];
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
        }

        Ok(())
    }
}
