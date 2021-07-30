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

use std::collections::HashMap;

use crate::{ Store, Loader, Cache };

use crate::util::osrs::{
    parse_defs,
    parse_defs_from_archive
};

use crate::def::osrs::{
    ItemDefinition,
    NpcDefinition,
    ObjectDefinition,
    ModelDefinition,
};

/// Loads all item definitions from the current cache.
#[derive(Clone, Eq, PartialEq, Debug, Default)]
pub struct ItemLoader {
    pub items: HashMap<u32, ItemDefinition>
}

/// Loads all npc definitions from the current cache.
#[derive(Clone, Eq, PartialEq, Debug, Default)]
pub struct NpcLoader {
    pub npcs: HashMap<u32, NpcDefinition>
}

/// Loads all object definitions from the current cache.
#[derive(Clone, Eq, PartialEq, Debug, Default)]
pub struct ObjectLoader {
    pub objs: HashMap<u32, ObjectDefinition>
}

/// Loads all model definitions from the current cache.
#[derive(Clone, Eq, PartialEq, Debug, Default)]
pub struct ModelLoader {
    pub mdls: HashMap<u32, ModelDefinition>
}

// TODO: make all fields private and impl iter on all loaders

impl_osrs_loader!(ItemLoader, ItemDefinition, items, index_id: 2, archive_id: 10);
impl_osrs_loader!(NpcLoader, NpcDefinition, npcs, index_id: 2, archive_id: 9);
impl_osrs_loader!(ObjectLoader, ObjectDefinition, objs, index_id: 2, archive_id: 6);
impl_osrs_loader!(ModelLoader, ModelDefinition, mdls, index_id: 7);

// impl_osrs_loader!(ItemLoader, ItemDefinition, items, parse_defs_from_archive, 2, 10);
// impl_osrs_loader!(NpcLoader, NpcDefinition, npcs, parse_defs_from_archive, 2, 9);
// impl_osrs_loader!(ObjectLoader, ObjectDefinition, objs, parse_defs_from_archive, 2, 6);
// impl_osrs_loader!(ModelLoader, ModelDefinition, mdls, parse_defs, 7);