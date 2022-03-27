use std::collections::{hash_map, HashMap};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{
    definition::rs3::{FetchDefinition, ItemDefinition},
    Cache,
};

/// Loads all item definitions from the current cache.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Eq, PartialEq, Debug, Default)]
pub struct ItemLoader(HashMap<u32, ItemDefinition>);

impl_rs3_loader!(ItemLoader, ItemDefinition, index_id: 19);
