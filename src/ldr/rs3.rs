//! # Example
//! 
//! ```
//! use rscache::Cache;
//! use rscache::ldr::rs3::ItemLoader;
//! 
//! # fn main() -> rscache::Result<()> {
//! let cache = Cache::new("./data/rs3_cache")?;
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
use crate::def::rs3::ItemDefinition;

/// Loads all item definitions from the current cache.
#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Debug, Default)]
pub struct ItemLoader(HashMap<u32, ItemDefinition>);

impl_rs3_loader!(ItemLoader, ItemDefinition, archive_id: 19);