use std::collections::HashMap;

use super::super::ObjectDefinition;
use crate::{
    Cache, CacheError,
    LinkedListExt,
    codec,
    cache::archive,
};

/// Caches all the object definitions that were loaded.
#[derive(Clone, Eq, PartialEq, Debug, Default)]
pub struct ObjectLoader {
    pub objects: HashMap<u16, ObjectDefinition>
}

impl ObjectLoader {
    /// Constructs a new `ObjectLoader`.
    ///
    /// It loads all the object definitions and caches them.
    ///
    /// # Errors
    /// 
    /// If this function encounters any errors it will be wrapped
    /// in a `CacheError`. (`ReadError`s or I/O errors)
    /// 
    /// # Examples
    ///
    /// ```
    /// # use rscache::{ Cache, CacheError };
    /// use rscache::ObjectLoader;
    /// # fn main() -> Result<(), CacheError> {
    /// # let path = "./data/cache";
    /// # let cache = Cache::new(path)?;
    /// 
    /// let obj_loader = ObjectLoader::new(&cache)?;
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn new(cache: &Cache) -> Result<Self, CacheError> {    
        let index_id = 2;
        let archive_id = 6;
        
        let mut buffer = &cache.read(255, index_id)?.to_vec()[..];
        let buffer = &codec::decode(&mut buffer)?[..];
        
        let archives = archive::parse(buffer)?;
        let entry_count = archives[archive_id - 1].entry_count;
        
        let mut buffer = &cache.read(index_id as u8, archive_id as u16)?.to_vec()[..];
        let buffer = codec::decode(&mut buffer)?;
        
        let obj_data = archive::decode(&buffer, entry_count)?;
        let mut objects = HashMap::new();
        
        for (obj_id, obj_buffer) in obj_data {
            objects.insert(obj_id, ObjectDefinition::new(obj_id, &obj_buffer)?);
        }
        
        Ok(Self { objects })
    }

    /// Retrieves the `ObjectDefinition` for the given object `id`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use rscache::{ Cache, CacheError };
    /// # use rscache::ObjectLoader;
    /// # fn main() -> Result<(), CacheError> {
    /// # let path = "./data/cache";
    /// # let cache = Cache::new(path)?;
    /// # let obj_loader = ObjectLoader::new(&cache)?;
    /// // law rift id = 25034
    /// let obj_def = obj_loader.load(25034);
    /// 
    /// match obj_def {
    ///     Some(obj_def) => { 
    ///         assert_eq!("Law rift", obj_def.name);
    ///         assert_eq!(2178, obj_def.animation_id);
    ///         assert!(!obj_def.hollow);
    ///         assert!(!obj_def.obstruct_ground);
    ///     },
    ///     None => (),
    /// }
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn load(&self, id: u16) -> Option<&ObjectDefinition> {
        self.objects.get(&id)
    }
}