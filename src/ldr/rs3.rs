use std::collections::HashMap;

use crate::{ Store, Loader, util, Cache };

use crate::def::rs3::ItemDefinition;

/// Loads all item definitions from the current cache.
#[derive(Clone, Eq, PartialEq, Debug, Default)]
pub struct ItemLoader {
    pub items: HashMap<u32, ItemDefinition>
}

impl_rs3_loader!(ItemLoader, ItemDefinition, items, archive_id: 19);