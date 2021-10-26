#[allow(clippy::too_many_lines)]
mod item_def;

pub use item_def::*;

use std::collections::HashMap;

use crate::{
    Cache,
    codec,
    REFERENCE_TABLE,
    arc::{ Archive, ArchiveFileGroup },
};

pub const ID_BLOCK_SIZE: usize = 256;

/// Marker trait for definitions.
pub trait Definition: Sized {
    fn new(id: u32, buffer: &[u8]) -> crate::Result<Self>;
}

pub trait FetchDefinition: Definition {
    // FIXME
    #[inline]
    fn fetch_from_index<D>(cache: &Cache, index_id: u8) -> crate::Result<HashMap<u32, D>> 
    where
        D: Definition,
    {
        let buffer = cache.read(REFERENCE_TABLE, index_id as u32)?;
        let buffer = codec::decode(&buffer)?;

        let archives = Archive::parse(&buffer)?;
        let mut definitions = std::collections::HashMap::new();
        let mut base_id = 0;

        for archive in &archives {
            let buffer = cache.read(index_id, archive.id as u32)?;
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
}

impl<D: Definition> FetchDefinition for D {}