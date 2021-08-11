use std::collections::{ hash_map, HashMap };

use crate::{ Store, Loader, util, Cache };

use crate::def::rs3::ItemDefinition;

/// Loads all item definitions from the current cache.
#[derive(Clone, Eq, PartialEq, Debug, Default)]
pub struct ItemLoader {
    itms: HashMap<u32, ItemDefinition>
}

impl_rs3_loader!(ItemLoader, ItemDefinition, itms, archive_id: 19);

impl_iter_for_loader!(ItemLoader, ItemDefinition, itms);