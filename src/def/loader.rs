mod item_loader;
mod npc_loader;
mod obj_loader;

pub use item_loader::ItemLoader;
pub use npc_loader::NpcLoader;
pub use obj_loader::ObjectLoader;

use std::{ collections::HashMap, marker::Sized };

use crate::{ Cache, LinkedListExt, codec, cache::archive };
use super::Definition;

/// Internal trait to supply loaders with basic functionality.
pub trait Loader<T: Definition>: super::internal::Sealed {
    fn new(cache: &Cache) -> crate::Result<Self> where Self: Sized;
    fn load(&self, id: u16) -> Option<&T>;
}

fn parse_defs<T: Definition>(cache: &Cache, archive_id: u16) -> crate::Result<HashMap<u16, T>> {
    let buffer = cache.read(255, 2)?.to_vec();
    let buffer = codec::decode(&mut buffer.as_slice())?;
    
    let archives = archive::parse(&buffer)?;
    let entry_count = archives[archive_id as usize - 1].entry_count;
    
    let buffer = cache.read(2, archive_id)?.to_vec();
    let buffer = codec::decode(&mut buffer.as_slice())?;

    let archive_data = archive::decode(&buffer, entry_count)?;

    let mut definitions = HashMap::new();
    for (id, buffer) in archive_data {
        definitions.insert(id, T::new(id, &buffer)?);
    }

    Ok(definitions)
}