//! A read-only, high-level, virtual file API for the RuneScape cache.
//!
//! This crate provides high performant data reads into the [Oldschool
//! RuneScape] and [RuneScape 3] cache file systems. It can read the necessary
//! data to synchronize the client's cache with the server. There are also some
//! [loaders](#loaders) that give access to definitions from the cache such as
//! items or npcs.
//!
//! For read-heavy workloads, a writer can be used to prevent continuous buffer
//! allocations. By default every read will allocate a writer with the correct
//! capacity.
//!
//! RuneScape's chat system uses huffman coding to compress messages. In order
//! to decompress them this library has a [`Huffman`] implementation.
//!
//! When a RuneScape client sends game packets the id's are encoded and can be
//! decoded with the [`IsaacRand`] implementation. These id's are encoded by the
//! client in a predictable random order which can be reversed if the server has
//! its own `IsaacRand` with the same encoder/decoder keys. These keys are sent
//! by the client on login and are user specific. It will only send encoded
//! packet id's if the packets are game packets.
//!
//! Note that this crate is still evolving; both OSRS & RS3 are not fully
//! supported/implemented and will probably contain bugs or miss core features.
//! If you require features or find bugs consider [opening an issue].
//!
//! # Safety
//!
//! In order to read bytes in a high performant way the cache uses [memmap2].
//! This can be unsafe because of its potential for _Undefined Behaviour_ when
//! the underlying file is subsequently modified, in or out of process. Using
//! `Mmap` here is safe because the RuneScape cache is a read-only binary file
//! system. The map will remain valid even after the `File` is dropped, it's
//! completely independent of the `File` used to create it. Therefore, the use
//! of unsafe is not propagated outwards. When the `Cache` is dropped memory
//! will be subsequently unmapped.
//!
//! # Features
//!
//! The cache's protocol defaults to OSRS. In order to use the RS3 protocol you
//! can enable the `rs3` feature flag. A lot of types derive [serde]'s
//! `Serialize` and `Deserialize`. The `serde-derive` feature flag can be used
//! to enable (de)serialization on any compatible types.
//!
//! # Quick Start
//!
//! For an instance that stays local to this thread you can simply use:
//! ```
//! use rscache::Cache;
//! 
//! fn main() {
//!     let cache = Cache::new("./data/osrs_cache").unwrap();
//! 
//!     let index_id = 2; // Config index.
//!     let archive_id = 10; // Archive containing item definitions.
//! 
//!     let buffer = cache.read(index_id, archive_id).unwrap();
//! }
//! ```
//! 
//! If you want to share the instance over multiple threads you can do so by
//! wrapping it in an
//! [`Arc`](https://doc.rust-lang.org/std/sync/struct.Arc.html)
//! ```
//! use rscache::Cache;
//! use std::sync::Arc;
//! 
//! fn main() {
//!     let cache = Arc::new(Cache::new("./data/osrs_cache").unwrap());
//!     
//!     let c = Arc::clone(&cache);
//!     std::thread::spawn(move || {
//!         c.read(0, 10).unwrap();
//!     });
//! 
//!     std::thread::spawn(move || {
//!         cache.read(0, 10).unwrap();
//!     });
//! }
//! ```
//! 
//! The recommended usage would be to wrap it using
//! [`once_cell`](https://docs.rs/once_cell/latest/once_cell/) making it the
//! easiest way to access cache data from anywhere and at any time. No need for
//! an `Arc` or a `Mutex` because `Cache` will always be `Send` & `Sync`.
//! ```
//! use rscache::Cache;
//! use once_cell::sync::Lazy;
//! 
//! static CACHE: Lazy<Cache> = Lazy::new(|| {
//!     Cache::new("./data/osrs_cache").unwrap()
//! });
//! 
//! fn main() {
//!     std::thread::spawn(move || {
//!         CACHE.read(0, 10).unwrap();
//!     });
//! 
//!     std::thread::spawn(move || {
//!         CACHE.read(0, 10).unwrap();
//!     });
//! }
//! ```
//!
//! # Loaders
//!
//! In order to get [definitions](crate::definition) you can look at the
//! [loaders](crate::loader) this library provides. The loaders use the cache as
//! a dependency to parse in their data and cache the relevant definitions
//! internally. The loader module also tells you how to make a loader if this
//! crate doesn't (yet) provide it.
//!
//! Note: Some loaders cache these definitions lazily because of either the size
//! of the data or the performance. The map loader for example is both slow and
//! large so caching is by default lazy. Lazy loaders require mutability.
//!
//! [Oldschool RuneScape]: https://oldschool.runescape.com/
//! [RuneScape 3]: https://www.runescape.com/
//! [opening an issue]: https://github.com/jimvdl/rs-cache/issues/new
//! [serde]: https://crates.io/crates/serde
//! [memmap2]: https://crates.io/crates/memmap2
//! [`Huffman`]: crate::util::Huffman
//! [`IsaacRand`]: crate::util::IsaacRand
#![cfg_attr(docsrs, feature(doc_cfg))]
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

use checksum::Checksum;
#[cfg(feature = "rs3")]
use checksum::{RsaChecksum, RsaKeys};
use runefs::codec::{Buffer, Decoded, Encoded};
use runefs::error::{Error as RuneFsError, ReadError};
use runefs::{ArchiveRef, Dat2, Indices, MAIN_DATA};
use std::{io::Write, path::Path};

/// A complete virtual representation of the RuneScape cache file system.
#[derive(Debug)]
pub struct Cache {
    pub(crate) data: Dat2,
    pub(crate) indices: Indices,
}

impl Cache {
    /// Creates a high level virtual memory map over the cache directory.
    ///
    /// All files are isolated on allocation by keeping them as in-memory files.
    ///
    /// # Errors
    ///
    /// The bulk of the errors which might occur are mostely I/O related due to
    /// acquiring file handles.
    ///
    /// Other errors might include protocol changes in newer caches. Any error
    /// unrelated to I/O at this stage should be considered a bug.
    pub fn new<P: AsRef<Path>>(path: P) -> crate::Result<Self> {
        Ok(Self {
            data: Dat2::new(path.as_ref().join(MAIN_DATA))?,
            indices: Indices::new(path)?,
        })
    }

    /// Generate a checksum based on the current cache.
    ///
    /// The `Checksum` acts as a validator for individual cache files. Any
    /// RuneScape client will request a list of crc's to check the validity of
    /// all of the file data that was transferred.
    pub fn checksum(&self) -> crate::Result<Checksum> {
        Checksum::new(self)
    }

    /// Generate a checksum based on the current cache with RSA encryption.
    ///
    /// `RsaChecksum` wraps a regular `Checksum` with the added benefit of
    /// encrypting the whirlpool hash into the checksum buffer.
    #[cfg(feature = "rs3")]
    #[cfg_attr(docsrs, doc(cfg(feature = "rs3")))]
    pub fn checksum_with<'a>(&self, keys: RsaKeys<'a>) -> crate::Result<RsaChecksum<'a>> {
        RsaChecksum::with_keys(self, keys)
    }

    /// Retrieves and constructs data corresponding to the given index and
    /// archive.
    ///
    /// # Errors
    ///
    /// When trying to retrieve data from an index or an archive that does not
    /// exist the `IndexNotFound` or `ArchiveNotFound` errors are returned,
    /// respectively.
    ///
    /// Any other errors such as sector validation failures or failed parsers
    /// should be considered a bug.
    pub fn read(&self, index_id: u8, archive_id: u32) -> crate::Result<Buffer<Encoded>> {
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

        let buffer = self.data.read(archive)?;

        assert_eq!(buffer.len(), archive.length);

        Ok(buffer)
    }

    pub(crate) fn read_archive(&self, archive: &ArchiveRef) -> crate::Result<Buffer<Encoded>> {
        self.read(archive.index_id, archive.id)
    }

    /// Retrieves and writes data corresponding to the given index and archive
    /// into `W`.
    ///
    /// # Errors
    ///
    /// See the error section on [`read`](Cache::read) for more details.
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
        Ok(self.data.read_into_writer(archive, writer)?)
    }

    /// Retrieves the huffman table.
    ///
    /// Required when decompressing chat messages, see
    /// [`Huffman`](crate::util::Huffman).
    pub fn huffman_table(&self) -> crate::Result<Buffer<Decoded>> {
        let index_id = 10;

        let archive = self.archive_by_name(index_id, "huffman")?;
        let buffer = self.read_archive(archive)?;

        assert_eq!(buffer.len(), archive.length);

        Ok(buffer.decode()?)
    }

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
            .metadata
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
