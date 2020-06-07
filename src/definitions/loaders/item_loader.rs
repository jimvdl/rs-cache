use std::collections::HashMap;
use std::io;

use super::super::item_def::ItemDefinition;
use crate::{
    Cache, CacheError,
    LinkedListExt,
    codec,
    cache::archive::ArchiveData
};

/// Caches all the item definitions that were loaded.
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
        let mut buffer = &codec::decode(&mut buffer)?[..];
        
        let archives = ArchiveData::decode(&mut buffer)?;
        let entry_count = archives[archive_id - 1].entry_count();
        
        let mut buffer = &cache.read(2, 10)?.to_vec()[..];
        let buffer = codec::decode(&mut buffer)?;

        let items = decode_item_data(&buffer, entry_count)?;
        
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
    ///         assert_eq!(false, blue_partyhat.stackable);
    ///         assert_eq!(false, blue_partyhat.members_only);
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

fn decode_item_data(buffer: &[u8], entry_count: usize) -> io::Result<HashMap<u16, ItemDefinition>> {
    let chunks = buffer[buffer.len() - 1] as usize;
    let mut items = HashMap::new();
    let mut cached_chunks = Vec::new();
    let mut read_ptr = buffer.len() - 1 - chunks * entry_count * 4;

    for _ in 0..chunks {
        let mut chunk_size = 0;

        for entry_id in 0..entry_count {
            let mut bytes = [0; 4];
            bytes.copy_from_slice(&buffer[read_ptr..read_ptr + 4]);
            let delta = i32::from_be_bytes(bytes);
            
            read_ptr += 4;
            chunk_size += delta;

            cached_chunks.push((entry_id as u16, chunk_size as usize));
        }
    }

    read_ptr = 0;
    for (entry_id, chunk_size) in cached_chunks {
        let buf = buffer[read_ptr..read_ptr + chunk_size].to_vec();
        let item_def = ItemDefinition::new(entry_id, &buf)?;

        items.insert(entry_id, item_def);
        read_ptr += chunk_size;
    }

    Ok(items)
}