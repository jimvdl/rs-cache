use std::collections::HashMap;

use crate::{
    cache::REFERENCE_TABLE,
    Definition,
    arc::{ Archive, ArchiveFileGroup },
    CacheCore,
    codec,
};

pub const ID_BLOCK_SIZE: usize = 256;

macro_rules! impl_rs3_loader {
   ($ldr:ident, $def:ty, archive_id: $arc_id:expr) => {
        impl $ldr {
            #[inline]
            pub fn new<C: CacheCore>(cache: &C) -> crate::Result<Self> {
                Loader::new(cache)
            }

            #[inline]
            pub fn load(&self, id: u32) -> Option<&$def> {
                Loader::load(self, id)
            }
        }

        impl Loader for $ldr {
            type Definition = $def;

            #[inline]
            fn new<C: CacheCore>(cache: &C) -> crate::Result<Self> {
                let map = crate::util::rs3::parse_defs(cache, $arc_id)?;

                Ok(Self(map))
            }

            #[inline]
            fn load(&self, id: u32) -> Option<&Self::Definition> {
                self.0.get(&id)
            }
        }

        impl_iter_for_loader!($ldr, $def);
   };
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
/// # use rscache::{ Rs3Cache, util, def::rs3::ItemDefinition };
/// # fn main() -> rscache::Result<()> {
/// # let cache = Rs3Cache::new("./data/rs3_cache")?;
/// let archive_id = 19; // Archive containing item definitions.
/// let item_defs: HashMap<u32, ItemDefinition> = util::rs3::parse_defs(&cache, archive_id)?;
/// # Ok(())
/// # }
/// ```
#[inline]
pub fn parse_defs<D: Definition, C: CacheCore>(cache: &C, archive_id: u32) -> crate::Result<HashMap<u32, D>> {
    let buffer = cache.read(REFERENCE_TABLE, archive_id)?;
    let buffer = codec::decode(&buffer)?;

    let archives = Archive::parse(&buffer)?;
    let mut definitions = std::collections::HashMap::new();
    let mut base_id = 0;

    for archive in &archives {
        let buffer = cache.read(archive_id as u8, archive.id as u32)?;
        let buffer = codec::decode(&buffer)?;

        let archive_group = ArchiveFileGroup::parse(&buffer, archive.entry_count)?;

        for archive_file in archive_group {
            let id = base_id + archive.valid_ids[archive_file.id as usize] as usize;
            definitions.insert(id as u32, D::new(id as u32, &archive_file.data)?);
        }

        base_id += ID_BLOCK_SIZE;
    }

    Ok(definitions)
}