use std::collections::{
    hash_map::{self, Entry},
    HashMap,
};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use runefs::codec;
use crate::{
    definition::osrs::{
        Definition, FetchDefinition, ItemDefinition, LocationDefinition, MapDefinition,
        NpcDefinition, ObjectDefinition,
    },
    Cache,
};

/// Loads all item definitions from the current cache.
#[derive(Clone, Eq, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ItemLoader(HashMap<u16, ItemDefinition>);

impl_osrs_loader!(ItemLoader, ItemDefinition, index_id: 2, archive_id: 10);

/// Loads all npc definitions from the current cache.
#[derive(Clone, Eq, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct NpcLoader(HashMap<u16, NpcDefinition>);

impl_osrs_loader!(NpcLoader, NpcDefinition, index_id: 2, archive_id: 9);

/// Loads all object definitions from the current cache.
#[derive(Clone, Eq, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ObjectLoader(HashMap<u16, ObjectDefinition>);

impl_osrs_loader!(ObjectLoader, ObjectDefinition, index_id: 2, archive_id: 6);

/// Loads maps definitions lazily from the current cache.
#[derive(Debug)]
pub struct MapLoader<'cache> {
    cache: &'cache Cache,
    maps: HashMap<u16, MapDefinition>,
}

impl<'cache> MapLoader<'cache> {
    /// Make a new `MapLoader`.
    /// 
    /// This takes a `Cache` by references with a `'cache` lifetime.
    /// All the map definitions are loaded lazily where the `&'cache Cache` is used
    /// to cache them internally on load.
    pub fn new(cache: &'cache Cache) -> Self {
        Self {
            cache,
            maps: HashMap::new(),
        }
    }

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

/// Loads location definitions lazily from the current cache.
#[derive(Debug)]
pub struct LocationLoader<'cache> {
    cache: &'cache Cache,
    locations: HashMap<u16, LocationDefinition>,
}

impl<'cache> LocationLoader<'cache> {
    /// Make a new `LocationLoader`.
    /// 
    /// This takes a `Cache` by references with a `'cache` lifetime.
    /// All the location definitions are loaded lazily where the `&'cache Cache` is used
    /// to cache them internally on load.
    pub fn new(cache: &'cache Cache) -> Self {
        Self {
            cache,
            locations: HashMap::new(),
        }
    }

    /// Loads the location data for a particular region.
    /// 
    /// Also takes a `keys: [u32; 4]` because the location archive is encrypted
    /// with XTEA. The buffer is automatically decoded with the given keys. 
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
