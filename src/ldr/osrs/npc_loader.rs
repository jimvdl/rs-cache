use std::collections::HashMap;

use crate::def::osrs::NpcDefinition;
use crate::{ Cache, Store, Loader, util };

/// Caches all the npc definitions that were loaded.
#[derive(Clone, Eq, PartialEq, Debug, Default)]
pub struct NpcLoader {
    pub npcs: HashMap<u16, NpcDefinition>
}

impl NpcLoader {
    pub fn new<S: Store>(cache: &Cache<S>) -> crate::Result<Self> {
        Loader::new(cache)
    }

    pub fn load(&self, id: u16) -> Option<&NpcDefinition> {
        Loader::load(self, id)
    }
}

impl Loader<NpcDefinition> for NpcLoader {
    fn new<S: Store>(cache: &Cache<S>) -> crate::Result<Self> {
        let npcs = util::parse_defs(cache, 9)?;

        Ok(Self { npcs })
    }

    fn load(&self, id: u16) -> Option<&NpcDefinition> {
        self.npcs.get(&id)
    }
}