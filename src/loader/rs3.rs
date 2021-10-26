//! # Example
//!
//! ```
//! use rscache::Cache;
//! use rscache::loader::rs3::ItemLoader;
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

use std::collections::{hash_map, HashMap};

#[cfg(feature = "serde-derive")]
use serde::{Deserialize, Serialize};

use crate::{
    definition::rs3::{FetchDefinition, ItemDefinition},
    Cache,
};

/// Loads all item definitions from the current cache.
#[derive(Clone, Eq, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde-derive", derive(Serialize, Deserialize))]
pub struct ItemLoader(HashMap<u32, ItemDefinition>);

impl_rs3_loader!(ItemLoader, ItemDefinition, index_id: 19);
