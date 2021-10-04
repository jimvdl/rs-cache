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

use std::collections::{ hash_map, HashMap };

use serde::{ Serialize, Deserialize };

use crate::{
    Cache,
    codec,
    def::osrs::{
        Definition,
        FetchDefinition,
        ItemDefinition,
        NpcDefinition,
        ObjectDefinition,
        MapDefinition,
    },
};

/// The core of each Loader tasked with loading certain definitions.
pub trait Loader: Sized {
    type Definition: Definition;

    fn new(cache: &Cache) -> crate::Result<Self>;
    fn load(&self, id: u16) -> Option<&Self::Definition>;
}

/// Loads all item definitions from the current cache.
#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Debug, Default)]
pub struct ItemLoader(HashMap<u16, ItemDefinition>);

/// Loads all npc definitions from the current cache.
#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Debug, Default)]
pub struct NpcLoader(HashMap<u16, NpcDefinition>);

/// Loads all object definitions from the current cache.
#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Debug, Default)]
pub struct ObjectLoader(HashMap<u16, ObjectDefinition>);

// /// Loads all object definitions from the current cache.
// #[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Debug, Default)]
// pub struct MapLoader(pub HashMap<u32, MapDefinition>);

impl_osrs_loader!(ItemLoader, ItemDefinition, index_id: 2, archive_id: 10);
impl_osrs_loader!(NpcLoader, NpcDefinition, index_id: 2, archive_id: 9);
impl_osrs_loader!(ObjectLoader, ObjectDefinition, index_id: 2, archive_id: 6);

// impl MapLoader {
//     #[inline]
//     pub fn new() -> Self {
//         Self(HashMap::new())
//     }

//     #[inline]
//     pub fn load(&mut self, cache: &Cache, id: u32) -> crate::Result<&MapDefinition> {
//         let map_def = self.0.entry(id).or_insert(MapDefinition::load_internal(cache, id)?);

//         Ok(map_def)
//     }
// }

// impl_iter_for_loader!(MapLoader, MapDefinition);