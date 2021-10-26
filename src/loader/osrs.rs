//! # Example
//!
//! ```
//! use rscache::Cache;
//! use rscache::ldr::osrs::ItemLoader;
//!
//! # fn main() -> rscache::Result<()> {
//! let cache = Cache::new("./data/osrs_cache")?;
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

use std::collections::{
    hash_map::{self, Entry},
    HashMap,
};

#[cfg(feature = "serde-derive")]
use serde::{Deserialize, Serialize};

use crate::{
    codec,
    definition::osrs::{
        Definition, FetchDefinition, ItemDefinition, LocationDefinition, MapDefinition,
        NpcDefinition, ObjectDefinition,
    },
    Cache,
};

/// Loads all item definitions from the current cache.
#[derive(Clone, Eq, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde-derive", derive(Serialize, Deserialize))]
pub struct ItemLoader(HashMap<u16, ItemDefinition>);

impl_osrs_loader!(ItemLoader, ItemDefinition, index_id: 2, archive_id: 10);

/// Loads all npc definitions from the current cache.
#[derive(Clone, Eq, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde-derive", derive(Serialize, Deserialize))]
pub struct NpcLoader(HashMap<u16, NpcDefinition>);

impl_osrs_loader!(NpcLoader, NpcDefinition, index_id: 2, archive_id: 9);

/// Loads all object definitions from the current cache.
#[derive(Clone, Eq, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde-derive", derive(Serialize, Deserialize))]
pub struct ObjectLoader(HashMap<u16, ObjectDefinition>);

impl_osrs_loader!(ObjectLoader, ObjectDefinition, index_id: 2, archive_id: 6);

#[derive(Debug)]
pub struct MapLoader<'cache> {
    cache: &'cache Cache,
    maps: HashMap<u16, MapDefinition>,
}

impl<'cache> MapLoader<'cache> {
    #[inline]
    pub fn new(cache: &'cache Cache) -> Self {
        Self {
            cache,
            maps: HashMap::new(),
        }
    }

    #[inline]
    pub fn load(&mut self, id: u16) -> crate::Result<&MapDefinition> {
        if let Entry::Vacant(entry) = self.maps.entry(id) {
            let x = id >> 8;
            let y = id & 0xFF;

            let map_archive = self.cache.archive_by_name(5, format!("m{}_{}", x, y))?;
            let buffer = self.cache.read_archive(map_archive)?;
            let buffer = codec::decode(&buffer)?;

            entry.insert(MapDefinition::new(id, &buffer)?);
        }

        Ok(&self.maps[&id])
    }
}

#[derive(Debug)]
pub struct LocationLoader<'cache> {
    cache: &'cache Cache,
    locations: HashMap<u16, LocationDefinition>,
}

impl<'cache> LocationLoader<'cache> {
    #[inline]
    pub fn new(cache: &'cache Cache) -> Self {
        Self {
            cache,
            locations: HashMap::new(),
        }
    }

    #[inline]
    pub fn load(&mut self, id: u16, keys: &[u32; 4]) -> crate::Result<&LocationDefinition> {
        if let Entry::Vacant(entry) = self.locations.entry(id) {
            let x = id >> 8;
            let y = id & 0xFF;

            let loc_archive = self.cache.archive_by_name(5, format!("l{}_{}", x, y))?;
            let buffer = self.cache.read_archive(loc_archive)?;
            let buffer = codec::decode_with_keys(&buffer, keys)?;

            entry.insert(LocationDefinition::new(id, &buffer)?);
        }

        Ok(&self.locations[&id])
    }
}
