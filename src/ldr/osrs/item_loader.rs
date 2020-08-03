use std::collections::HashMap;

use crate::def::osrs::ItemDefinition;
use crate::{ Cache, Store, Loader, util };

/// Caches all the item definitions that were loaded.
#[derive(Clone, Eq, PartialEq, Debug, Default)]
pub struct ItemLoader {
    pub items: HashMap<u16, ItemDefinition>
}

impl ItemLoader {
    pub fn new<S: Store>(cache: &Cache<S>) -> crate::Result<Self> {
        Loader::new(cache)
    }

    pub fn load(&self, id: u16) -> Option<&ItemDefinition> {
        Loader::load(self, id)
    }
}

impl Loader<ItemDefinition> for ItemLoader {
    fn new<S: Store>(cache: &Cache<S>) -> crate::Result<Self> {
        let items = util::parse_defs(cache, 10)?;

        Ok(Self { items })
    }

    fn load(&self, id: u16) -> Option<&ItemDefinition> {
        self.items.get(&id)
    }
}