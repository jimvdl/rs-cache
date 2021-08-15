//! # Example
//! 
//! ```
//! use rscache::OsrsCache;
//! use rscache::ldr::osrs::ItemLoader;
//! 
//! # fn main() -> rscache::Result<()> {
//! let cache = OsrsCache::new("./data/osrs_cache")?;
//! let item_ldr = ItemLoader::new(&cache)?;
//! 
//! if let Some(def) = item_ldr.load(1042) {
//!     assert_eq!("Blue partyhat", def.name);
//!     assert!(!def.stackable);
//!     assert!(!def.members_only);
//! }
//! # Ok(())
//! # }
//! ```
//! 
//! # Errors
//! 
//! Every loader returns a `CacheError` with an inner error.

use std::collections::{ hash_map, HashMap };

use crate::{ Store, Loader, Cache };

use crate::def::osrs::{
    ItemDefinition,
    NpcDefinition,
    ObjectDefinition,
};

/// Loads all item definitions from the current cache.
#[derive(Clone, Eq, PartialEq, Debug, Default)]
pub struct ItemLoader(HashMap<u32, ItemDefinition>);

/// Loads all npc definitions from the current cache.
#[derive(Clone, Eq, PartialEq, Debug, Default)]
pub struct NpcLoader(HashMap<u32, NpcDefinition>);

/// Loads all object definitions from the current cache.
#[derive(Clone, Eq, PartialEq, Debug, Default)]
pub struct ObjectLoader(HashMap<u32, ObjectDefinition>);

impl_osrs_loader!(ItemLoader, ItemDefinition, index_id: 2, archive_id: 10);
impl_osrs_loader!(NpcLoader, NpcDefinition, index_id: 2, archive_id: 9);
impl_osrs_loader!(ObjectLoader, ObjectDefinition, index_id: 2, archive_id: 6);