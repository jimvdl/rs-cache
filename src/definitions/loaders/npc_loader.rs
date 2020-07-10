use std::collections::HashMap;

use super::super::npc_def::NpcDefinition;
use crate::{
    Cache, CacheError,
    LinkedListExt,
    codec,
    cache::archive::{ Archive, ArchiveData }
};

pub struct NpcLoader {
    pub npcs: HashMap<u16, NpcDefinition>
}

impl NpcLoader {
    #[inline]
    pub fn new(cache: &Cache) -> Result<Self, CacheError> {    
        let index_id = 2;
        let archive_id = 9;
        
        let mut buffer = &cache.read(255, index_id)?.to_vec()[..];
        let mut buffer = &codec::decode(&mut buffer)?[..];
        
        let archives = ArchiveData::decode(&mut buffer)?;
        let entry_count = archives[archive_id - 1].entry_count();
        
        let mut buffer = &cache.read(index_id as u8, archive_id as u16)?.to_vec()[..];
        let buffer = codec::decode(&mut buffer)?;
        
        let item_data = Archive::decode(&buffer, entry_count)?;
        let mut npcs = HashMap::new();
        
        for (npc_id, npc_buffer) in item_data {
            npcs.insert(npc_id, NpcDefinition::new(npc_id, &npc_buffer)?);
        }
        
        Ok(Self { npcs })
    }

    #[inline]
    pub fn load(&self, id: u16) -> Option<&NpcDefinition> {
        self.npcs.get(&id)
    }
}