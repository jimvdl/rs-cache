use std::collections::HashMap;

use crate::def::osrs::NpcDefinition;
use crate::{ Cache, Store, Loader, util };

/// Caches all the npc definitions that were loaded.
#[derive(Clone, Eq, PartialEq, Debug, Default)]
pub struct NpcLoader {
    pub npcs: HashMap<u16, NpcDefinition>
}

impl<S: Store> Loader<NpcDefinition, S> for NpcLoader {
    fn new(cache: &Cache<S>) -> crate::Result<Self> {
        let npcs = util::parse_defs(cache, 9)?;

        Ok(Self { npcs })
    }

    fn load(&self, id: u16) -> Option<&NpcDefinition> {
        self.npcs.get(&id)
    }
}