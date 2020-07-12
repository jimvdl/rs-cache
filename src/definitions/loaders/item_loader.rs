use std::collections::HashMap;

use super::super::item_def::ItemDefinition;
use crate::{
    Cache, CacheError,
    LinkedListExt,
    codec,
    cache::archive::{ Archive, ArchiveData },
};

/// Caches all the item definitions that were loaded.
#[derive(Clone, Eq, PartialEq, Debug, Default)]
pub struct ItemLoader {
    pub items: HashMap<u16, ItemDefinition>
}

impl ItemLoader {
    /// Constructs a new `ItemLoader`.
    ///
    /// It loads all the item definitions and caches them.
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
    /// use rscache::ItemLoader;
    /// # fn main() -> Result<(), CacheError> {
    /// # let path = "./data/cache";
    /// # let cache = Cache::new(path)?;
    /// 
    /// let item_loader = ItemLoader::new(&cache)?;
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn new(cache: &Cache) -> Result<Self, CacheError> {    
        let index_id = 2;
        let archive_id = 10;
        
        let mut buffer = &cache.read(255, index_id)?.to_vec()[..];
        let buffer = &codec::decode(&mut buffer)?[..];
        
        let archives = ArchiveData::decode(buffer)?;
        let entry_count = archives[archive_id - 1].entry_count();
        
        let mut buffer = &cache.read(index_id as u8, archive_id as u16)?.to_vec()[..];
        let buffer = codec::decode(&mut buffer)?;

        let item_data = Archive::decode(&buffer, entry_count)?;
        let mut items = HashMap::new();

        for (item_id, item_buffer) in item_data {
            items.insert(item_id, ItemDefinition::new(item_id, &item_buffer)?);
        }

        Ok(Self { items })
    }

    /// Retrieves the `ItemDefinition` for the given item `id`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use rscache::{ Cache, CacheError };
    /// # use rscache::ItemLoader;
    /// # fn main() -> Result<(), CacheError> {
    /// # let path = "./data/cache";
    /// # let cache = Cache::new(path)?;
    /// # let item_loader = ItemLoader::new(&cache)?;
    /// // blue partyhat id = 1042
    /// let blue_partyhat = item_loader.load(1042);
    /// 
    /// match blue_partyhat {
    ///     Some(blue_partyhat) => {
    ///         assert_eq!("Blue partyhat", blue_partyhat.name);
    ///         assert!(!blue_partyhat.stackable);
    ///         assert!(!blue_partyhat.members_only);
    ///     },
    ///     None => (),
    /// }
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn load(&self, id: u16) -> Option<&ItemDefinition> {
        self.items.get(&id)
    }
}