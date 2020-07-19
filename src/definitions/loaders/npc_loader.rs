use std::collections::HashMap;

use super::super::NpcDefinition;
use crate::{
    Cache,
    LinkedListExt,
    codec,
    cache::archive,
};

/// Caches all the npc definitions that were loaded.
#[derive(Clone, Eq, PartialEq, Debug, Default)]
pub struct NpcLoader {
    pub npcs: HashMap<u16, NpcDefinition>
}

impl NpcLoader {
    /// Constructs a new `NpcLoader`.
    ///
    /// It loads all the npc definitions and caches them.
    ///
    /// # Errors
    /// 
    /// If this function encounters any errors it will be wrapped
    /// in a `CacheError`. (`ReadError`s or I/O errors)
    /// 
    /// # Examples
    ///
    /// ```
    /// # use rscache::Cache;
    /// use rscache::NpcLoader;
    /// # fn main() -> rscache::Result<()> {
    /// # let path = "./data/cache";
    /// # let cache = Cache::new(path)?;
    /// 
    /// let npc_loader = NpcLoader::new(&cache)?;
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn new(cache: &Cache) -> crate::Result<Self> {    
        let index_id = 2;
        let archive_id = 9;
        
        let mut buffer = &cache.read(255, index_id)?.to_vec()[..];
        let buffer = &codec::decode(&mut buffer)?[..];
        
        let archives = archive::parse(buffer)?;
        let entry_count = archives[archive_id - 1].entry_count;
        
        let mut buffer = &cache.read(index_id as u8, archive_id as u16)?.to_vec()[..];
        let buffer = codec::decode(&mut buffer)?;
        
        let npc_data = archive::decode(&buffer, entry_count)?;
        let mut npcs = HashMap::new();
        
        for (npc_id, npc_buffer) in npc_data {
            npcs.insert(npc_id, NpcDefinition::new(npc_id, &npc_buffer)?);
        }
        
        Ok(Self { npcs })
    }

    /// Retrieves the `NpcDefinition` for the given npc `id`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use rscache::Cache;
    /// # use rscache::NpcLoader;
    /// # fn main() -> rscache::Result<()> {
    /// # let path = "./data/cache";
    /// # let cache = Cache::new(path)?;
    /// # let npc_loader = NpcLoader::new(&cache)?;
    /// // wise old man id = 2108
    /// let npc_def = npc_loader.load(2108);
    /// 
    /// match npc_def {
    ///     Some(npc_def) => {
    ///         assert_eq!("Wise Old Man", npc_def.name);
    ///         assert!(npc_def.interactable);
    ///     },
    ///     None => (),
    /// }
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn load(&self, id: u16) -> Option<&NpcDefinition> {
        self.npcs.get(&id)
    }
}