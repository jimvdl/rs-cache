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

use crate::{ Loader, Cache };
// use crate::codec;
use crate::def::{
    // Definition,
    FetchDefinition,
    osrs::{
        ItemDefinition,
        NpcDefinition,
        ObjectDefinition,
        MapDefinition,
    }
};

/// Loads all item definitions from the current cache.
#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Debug, Default)]
pub struct ItemLoader(HashMap<u32, ItemDefinition>);

/// Loads all npc definitions from the current cache.
#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Debug, Default)]
pub struct NpcLoader(HashMap<u32, NpcDefinition>);

/// Loads all object definitions from the current cache.
#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Debug, Default)]
pub struct ObjectLoader(HashMap<u32, ObjectDefinition>);

// /// Loads all object definitions from the current cache.
// #[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Debug, Default)]
// pub struct MapLoader(HashMap<u32, MapDefinition>);

impl_osrs_loader!(ItemLoader, ItemDefinition, index_id: 2, archive_id: 10);
impl_osrs_loader!(NpcLoader, NpcDefinition, index_id: 2, archive_id: 9);
impl_osrs_loader!(ObjectLoader, ObjectDefinition, index_id: 2, archive_id: 6);

// impl MapLoader {
//     #[inline]
//     pub fn new(cache: &Cache) -> crate::Result<Self> {
//         Loader::new(cache)
//     }

//     #[inline]
//     pub fn load(&self, id: u32) -> Option<&MapDefinition> {
//         Loader::load(self, id)
//     }
// }

// impl Loader for MapLoader {
//     type Definition = MapDefinition;

//     #[inline]
//     fn new(cache: &Cache) -> crate::Result<Self> {
//         for region_id in 0..32768 {
//             let x = region_id >> 8;
//             let y = region_id & 0xFF;

//             if let Ok(map_archive) = cache.archive_by_name(5, format!("m{}_{}", x, y)) {
//                 let buffer = cache.read_archive(map_archive)?;
//                 let buffer = codec::decode(&buffer)?;
                
//                 let map_def = Self::Definition::new(region_id, &buffer)?;
//             }
//         }
        
//         todo!()
//     }

//     #[inline]
//     fn load(&self, id: u32) -> Option<&Self::Definition> {
//         self.0.get(&id)
//     }
// }

// impl_iter_for_loader!(MapLoader, MapDefinition);