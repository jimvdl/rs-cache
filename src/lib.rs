//! An immutable, high-level API for the RuneScape cache file system.
//!
//! This crate provides high performant data reads into the [Oldschool RuneScape] and [RuneScape 3] cache file systems.
//! It can read the necessary data to synchronize the client's cache with the server. There are also some
//! [loaders](#loaders) that give access to definitions from the cache such as items or npcs.
//!
//! For read-heavy workloads, a writer can be used to prevent continuous buffer allocations.
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
//! after the `File` is dropped, it's completely independent of the `File` used to create it. Therefore, the use of unsafe is
//! not propagated outwards. When the `Cache` is dropped memory will be subsequently unmapped.
//!
//! # Features
//!
//! The cache's protocol defaults to OSRS. In order to use the RS3 protocol you can enable the `rs3` feature flag.
//! A lot of types derive [serde]'s `Serialize` and `Deserialize`. The `serde-derive` feature flag can be used to
//! enable (de)serialization on any compatible types.
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

#![deny(
    clippy::all,
    clippy::correctness,
    clippy::suspicious,
    clippy::style,
    clippy::complexity,
    clippy::perf
)]

#[macro_use]
pub mod util;
pub mod checksum;
pub mod definition;
pub mod error;
pub mod extension;
pub mod loader;

#[doc(inline)]
pub use error::Error;
use error::Result;

// use crate::error::ReadError;
use runefs::error::{ReadError, Error as RuneFsError};
use runefs::{ArchiveRef, Dat2, Indices, MAIN_DATA};
use std::{io::Write, path::Path};

/// A parsed Jagex cache.
#[derive(Debug)]
pub struct Cache {
    data: Dat2,
    pub(crate) indices: Indices,
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
        Ok(Self {
            data: Dat2::new(path.as_ref().join(MAIN_DATA))?,
            indices: Indices::new(path)?,
        })
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
            .ok_or(RuneFsError::Read(ReadError::IndexNotFound(index_id)))?;

        let archive = index
            .archive_refs
            .get(&archive_id)
            .ok_or(RuneFsError::Read(ReadError::ArchiveNotFound {
                idx: index_id,
                arc: archive_id,
            }))?;

        let mut buffer = Vec::with_capacity(archive.length);
        self.data.read(archive, &mut buffer)?;

        assert_eq!(buffer.len(), archive.length);

        Ok(buffer)
    }

    pub(crate) fn read_archive(&self, archive: &ArchiveRef) -> crate::Result<Vec<u8>> {
        self.read(archive.index_id, archive.id)
    }

    /// Reads bytes from the cache into the given writer.
    ///
    /// For read-heavy workloads it is recommended to use this version of read to prevent
    /// multiple buffer allocations, instead it will not allocate a buffer but use the writer
    /// instead, see [`read`](Cache::read).
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
            .ok_or(RuneFsError::Read(ReadError::IndexNotFound(index_id)))?;

        let archive = index
            .archive_refs
            .get(&archive_id)
            .ok_or(RuneFsError::Read(ReadError::ArchiveNotFound {
                idx: index_id,
                arc: archive_id,
            }))?;
        Ok(self.data.read(archive, writer)?)
    }

    /// Tries to return the huffman table from the cache.
    ///
    /// This can be used to decompress chat messages, see [`Huffman`](crate::util::Huffman).
    #[inline]
    pub fn huffman_table(&self) -> crate::Result<Vec<u8>> {
        let index_id = 10;

        let archive = self.archive_by_name(index_id, "huffman")?;
        let buffer = self.read_archive(archive)?;
        Ok(runefs::codec::decode(&buffer)?)
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
            .ok_or(RuneFsError::Read(ReadError::IndexNotFound(index_id)))?;
        let hash = util::djd2::hash(&name);

        let archive = index
            .archives
            .iter()
            .find(|archive| archive.name_hash == hash)
            .ok_or_else(|| crate::error::NameHashMismatch {
                hash,
                name: name.as_ref().into(),
                idx: index_id,
            })?;

        let archive_ref = index
            .archive_refs
            .get(&archive.id)
            .ok_or(RuneFsError::Read(ReadError::ArchiveNotFound {
                idx: index_id,
                arc: archive.id,
            }))?;

        Ok(archive_ref)
    }
}

#[cfg(test)]
fn is_normal<T: Send + Sync + Sized + Unpin>() {}
#[test]
fn normal_types() {
    is_normal::<Cache>();
}
