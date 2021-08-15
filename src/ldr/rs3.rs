use std::collections::{ hash_map, HashMap };

use crate::{ Store, Loader, Cache };

use crate::def::rs3::ItemDefinition;

/// Loads all item definitions from the current cache.
#[derive(Clone, Eq, PartialEq, Debug, Default)]
pub struct ItemLoader(HashMap<u32, ItemDefinition>);

impl_rs3_loader!(ItemLoader, ItemDefinition, archive_id: 19);