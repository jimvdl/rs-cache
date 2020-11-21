//! # Example
//! 
//! ```
//! use rscache::OsrsCache;
//! use rscache::ldr::osrs::ItemLoader;
//! 
//! # fn main() -> rscache::Result<()> {
//! let cache = OsrsCache::new("./data/cache")?;
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

use std::collections::HashMap;

use crate::{ Store, Loader, util, Cache };

use crate::def::osrs::{
    ItemDefinition,
    NpcDefinition,
    ObjectDefinition,
};

/// Loads all item definitions from the current cache.
#[derive(Clone, Eq, PartialEq, Debug, Default)]
pub struct ItemLoader {
    pub items: HashMap<u16, ItemDefinition>
}

/// Loads all npc definitions from the current cache.
#[derive(Clone, Eq, PartialEq, Debug, Default)]
pub struct NpcLoader {
    pub npcs: HashMap<u16, NpcDefinition>
}

/// Loads all object definitions from the current cache.
#[derive(Clone, Eq, PartialEq, Debug, Default)]
pub struct ObjectLoader {
    pub objs: HashMap<u16, ObjectDefinition>
}

impl_loader!(ItemLoader, ItemDefinition, items, archive_id: 10);
impl_loader!(NpcLoader, NpcDefinition, npcs, archive_id: 9);
impl_loader!(ObjectLoader, ObjectDefinition, objs, archive_id: 6);