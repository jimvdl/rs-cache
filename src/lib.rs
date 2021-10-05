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
    clippy::used_underscore_binding
)]

// TODO: determine what belongs in public api

#[macro_use]
pub mod util;
pub mod cksm;
pub mod ext;
pub mod parse;
pub mod error;
pub mod codec;
pub mod def;
pub mod ldr;
mod cache;
mod idx;
mod arc;
mod sec;

#[doc(inline)]
pub use error::{ Result, CacheError };
#[doc(inline)]
pub use cache::{ Cache, ReadIntoWriter };

/// Core architecture.
pub mod fs {
    #[doc(inline)]
    pub use crate::cache::*;
    #[doc(inline)]
    pub use crate::arc::*;
    #[doc(inline)]
    pub use crate::idx::*;
    #[doc(inline)]
    pub use crate::sec::*;
}