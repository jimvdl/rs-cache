use std::collections::HashMap;

use super::super::NpcDefinition;
use crate::{ Cache, Loader };
use crate::def::loader;

/// Caches all the npc definitions that were loaded.
#[derive(Clone, Eq, PartialEq, Debug, Default)]
pub struct NpcLoader {
    pub npcs: HashMap<u16, NpcDefinition>
}

impl Loader<NpcDefinition> for NpcLoader {
    fn new(cache: &Cache) -> crate::Result<Self> {
        let npcs = loader::parse_defs(cache, 9)?;

        Ok(Self { npcs })
    }

    fn load(&self, id: u16) -> Option<&NpcDefinition> {
        self.npcs.get(&id)
    }
}