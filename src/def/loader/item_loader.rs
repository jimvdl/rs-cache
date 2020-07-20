use std::collections::HashMap;

use super::super::ItemDefinition;
use crate::{ Cache, Loader };
use crate::def::loader;

/// Caches all the item definitions that were loaded.
#[derive(Clone, Eq, PartialEq, Debug, Default)]
pub struct ItemLoader {
    pub items: HashMap<u16, ItemDefinition>
}

impl Loader<ItemDefinition> for ItemLoader {
    fn new(cache: &Cache) -> crate::Result<Self> {
        let items = loader::parse_defs(cache, 10)?;

        Ok(Self { items })
    }

    fn load(&self, id: u16) -> Option<&ItemDefinition> {
        self.items.get(&id)
    }
}