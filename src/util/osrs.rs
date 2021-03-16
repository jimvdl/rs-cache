#[allow(unused_assignments)]
mod huffman;
#[allow(clippy::many_single_char_names, clippy::too_many_lines)]
mod isaac_rand;
/// Default xtea decipher.
pub mod xtea;

pub use huffman::Huffman;
pub use isaac_rand::IsaacRand;

use crate::{
    codec,
    def::osrs::MapDefinition,
    Store,
    Cache,
    Definition,
};

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