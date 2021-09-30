#[allow(unused_assignments)]
mod huffman;
#[allow(clippy::many_single_char_names, clippy::too_many_lines)]
mod isaac_rand;

pub use huffman::Huffman;
pub use isaac_rand::IsaacRand;

use crate::{
    codec,
    def::osrs::MapDefinition,
    Cache,
    Definition,
};

macro_rules! impl_osrs_loader {
   ($ldr:ident, $def:ty, index_id: $idx_id:expr $(, archive_id: $arc_id:expr)?) => {
        impl $ldr {
            #[inline]
            pub fn new(cache: &Cache) -> crate::Result<Self> {
                Loader::new(cache)
            }

            #[inline]
            pub fn load(&self, id: u32) -> Option<&$def> {
                Loader::load(self, id)
            }
        }

        impl Loader for $ldr {
            type Definition = $def;

            #[allow(unreachable_code)]
            #[inline]
            fn new(cache: &Cache) -> crate::Result<Self> {            
                $(
                    let map = Self::Definition::fetch_from_archive(cache, $idx_id, $arc_id)?;

                    return Ok(Self(map));
                )?

                let map = Self::Definition::fetch_from_index(cache, $idx_id)?;

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

/// Loads the [MapDefinition](../../def/osrs/struct.MapDefinition.html) belonging to a given region.
/// 
/// Returns `None` if the given region id doesn't have a corresponding map definition.
#[inline]
pub fn load_map_def(cache: &Cache, region_id: u32) -> crate::Result<Option<MapDefinition>> {
    let x = region_id >> 8;
    let y = region_id & 0xFF;

    if let Ok(map_archive) = cache.archive_by_name(5, format!("m{}_{}", x, y)) {
        let buffer = cache.read_archive(map_archive)?;
        let buffer = codec::decode(&buffer)?;
        
        return Ok(Some(MapDefinition::new(region_id, &buffer)?))
    }

    Ok(None)
}