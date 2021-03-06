#[allow(unused_assignments)]
mod huffman;
#[allow(clippy::many_single_char_names, clippy::too_many_lines)]
mod isaac_rand;
/// Default xtea decipher.
pub mod xtea;

pub use huffman::Huffman;
pub use isaac_rand::IsaacRand;

use std::collections::HashMap;

use crate::{
    codec,
    def::osrs::MapDefinition,
    cache::REFERENCE_TABLE,
    Store,
    Cache,
    Definition,
    arc,
};

#[macro_use]
macro_rules! impl_osrs_loader {
   ($ldr:ident, $def:ty, $defs_field:ident, archive_id: $arc_id:expr) => {
        impl $ldr {
            #[inline]
            pub fn new<S: Store>(cache: &Cache<S>) -> crate::Result<Self> {
                Loader::new(cache)
            }

            #[inline]
            pub fn load(&self, id: u16) -> Option<&$def> {
                Loader::load(self, id)
            }
        }

        impl Loader<$def> for $ldr {
            #[inline]
            fn new<S: Store>(cache: &Cache<S>) -> crate::Result<$ldr> {
                let $defs_field = util::osrs::parse_defs(cache, $arc_id)?;

                Ok($ldr { $defs_field })
            }

            #[inline]
            fn load(&self, id: u16) -> Option<&$def> {
                self.$defs_field.get(&id)
            }
        }
   };
}

/// Loads the [MapDefinition](../../def/osrs/struct.MapDefinition.html) belonging to a given region.
/// 
/// Returns `None` if the given region id doesn't have a corresponding map definition.
#[inline]
pub fn load_map_def<S: Store>(cache: &Cache<S>, region_id: u16) -> crate::Result<Option<MapDefinition>> {
    let x = region_id as u32 >> 8;
    let y = region_id as u32 & 0xFF;

    if let Ok(map_archive) = cache.archive_by_name(5, format!("m{}_{}", x, y)) {
        let buffer = cache.read_archive(&map_archive)?;
        let buffer = codec::decode(&buffer)?;
        
        return Ok(Some(MapDefinition::new(region_id, &buffer)?))
    }

    Ok(None)
}

/// Parses all definitions read from the passed `Cache<S>` from `archive_id`.
/// 
/// # Errors
/// 
/// Can return multiple errors: if reading, decoding or parsing definition buffers fail.
/// 
/// # Examples
/// 
/// ```
/// # use std::collections::HashMap;
/// # use rscache::{ OsrsCache, util, def::osrs::ItemDefinition };
/// # fn main() -> rscache::Result<()> {
/// # let cache = OsrsCache::new("./data/osrs_cache")?;
/// let archive_id = 10; // Archive containing item definitions.
/// let item_defs: HashMap<u16, ItemDefinition> = util::osrs::parse_defs(&cache, archive_id)?;
/// # Ok(())
/// # }
/// ```
#[inline]
pub fn parse_defs<T: Definition, S: Store>(cache: &Cache<S>, archive_id: u32) -> crate::Result<HashMap<u16, T>> {
    let buffer = cache.read(REFERENCE_TABLE, 2)?;
    let buffer = codec::decode(&buffer)?;
    
    let archives = arc::parse_archive_data(&buffer)?;
    let entry_count = archives[archive_id as usize - 1].entry_count;
    
    let buffer = cache.read(2, archive_id)?;
    let buffer = codec::decode(&buffer)?;

    let archive_data = arc::parse_content(&buffer, entry_count)?;

    let mut definitions = HashMap::new();
    for (id, buffer) in archive_data {
        definitions.insert(id, T::new(id, &buffer)?);
    }

    Ok(definitions)
}