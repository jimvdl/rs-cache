//! [Oldschool RuneScape](https://oldschool.runescape.com/) 
//! & [RuneScape 3](https://www.runescape.com/) cache api for basic
//! and simple cache interactions.
//! 
//! # Features
//! 
//! Currently rs-cache offers limited support for OSRS & RS3 with the features listed below.
//! This crate also contains tools to help you with implementing your own cache
//! if the currently supplied cache is insufficient for a specific use-case.
//! 
//! Note: this crate is still a work in progress and might contain bugs and is still
//! incomplete.
//! 
//! The following features are currently provided:
//! - Reading from the cache.
//! - Huffman buffer access.
//! - Checksum with simple-to-use validation.
//! - Compression and decompression:
//!   - [Gzip](https://crates.io/crates/libflate)
//!   - [Bzip2](https://crates.io/crates/bzip2)
//! - OSRS Loaders
//!   - [`ItemLoader`](ldr/osrs/struct.ItemLoader.html)
//!   - [`NpcLoader`](ldr/osrs/struct.NpcLoader.html)
//!   - [`ObjectLoader`](ldr/osrs/struct.ObjectLoader.html)
//! - RS3 Loaders
//!   - [`ItemLoader`](ldr/rs3/struct.ItemLoader.html)
//! - Utilities
//!   - Huffman decompressor.
//!   - Isaac randomizer.
//!   - Xtea decipher.
//! 
//! Features to be implemented in the future: 
//! - Writing data to the cache.
//! - LZMA compression.
//! 
//! # Quick Start
//! 
//! The quickest and easiest way to get started is by using 
//! [`OsrsCache`](type.OsrsCache.html) or [`Rs3Cache`](type.Rs3Cache.html).
//! (they work the same but use different reading methods)
//! 
//! ```
//! use rscache::OsrsCache;
//! 
//! # fn main() -> rscache::Result<()> {
//! let cache = OsrsCache::new("./data/osrs_cache")?;
//! 
//! let index_id = 2; // Config index.
//! let archive_id = 10; // Archive containing item definitions.
//! 
//! let buffer: Vec<u8> = cache.read(index_id, archive_id)?;
//! 
//! # Ok(())
//! # }
//! ```
//! 
//! # Cache interchangeability
//! 
//! The internal storage and reading functionalities can be changed
//! by using the generic [`Cache`](struct.Cache.html) struct and chosing
//! a store implementation that fits a specific use-case.
//! 
//! In the below example the [`FileStore`](struct.FileStore.html) holds a 
//! handle to the main data file while the [`MemoryStore`](struct.MemoryStore.html) 
//! parses the entire main data file into memory. If the main file is too large 
//! for the `MemoryStore` you can opt into a `FileStore` to do reading through disk I/O.
//! 
//! The type [`OsrsCache`](type.OsrsCache.html) is a type alias for `Cache<MemoryStore>`.
//! 
//! ```
//! use rscache::{ Cache, store::FileStore };
//! 
//! # fn main() -> rscache::Result<()> {
//! let cache = Cache::<FileStore>::new("./data/osrs_cache")?;
//! 
//! let index_id = 2; // Config index.
//! let archive_id = 10; // Archive containing item definitions.
//! 
//! let buffer: Vec<u8> = cache.read(index_id, archive_id)?;
//! 
//! # Ok(())
//! # }
//! ```
//! 
//! # Building a custom cache
//! 
//! This crate supplies traits and helper functions to help implement 
//! your own cache when the default cache doesn't do exactly what you need.
//! 
//! See the [custom_cache](https://github.com/jimvdl/rs-cache/tree/master/examples) 
//! example to help you get started.

#![deny(clippy::all, clippy::nursery)]

#![warn(
    clippy::clone_on_ref_ptr, 
    clippy::redundant_clone, 
    clippy::default_trait_access, 
    clippy::expl_impl_clone_on_copy,
    clippy::explicit_into_iter_loop, 
    clippy::explicit_iter_loop, 
    clippy::filter_map, 
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
    clippy::pub_enum_variant_names, 
    clippy::unused_self, 
    clippy::similar_names, 
    clippy::single_match_else, 
    clippy::too_many_lines, 
    clippy::type_repetition_in_bounds,
    clippy::unseparated_literal_suffix, 
    clippy::used_underscore_binding
)]

#[macro_use]
pub mod util;
pub mod cache;
pub mod cksm;
pub mod idx;
pub mod arc;
pub mod ext;
pub mod error;
pub mod store;
pub mod codec;
pub mod def;
pub mod ldr;
pub mod sec;

/// Type alias for `Cache<MemoryStore>`.
#[cfg(feature = "osrs")]
pub type OsrsCache = Cache<MemoryStore>;
/// Type alias for `Cache<FileStore>`.
#[cfg(feature = "rs3")]
pub type Rs3Cache = Cache<FileStore>;

#[doc(inline)]
pub use error::Result;
#[doc(inline)]
pub use cache::{ Cache, CacheCore, CacheRead };
#[doc(inline)]
pub use cksm::Checksum;
#[doc(inline)]
pub use store::{ Store, FileStore, MemoryStore };
#[doc(inline)]
pub use ldr::Loader;
#[doc(inline)]
pub use def::Definition;