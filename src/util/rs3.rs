use std::collections::HashMap;

use crate::{
    store::Store,
    cache::REFERENCE_TABLE,
    Definition,
    Cache,
    codec,
    arc,
};

#[macro_use]
macro_rules! impl_rs3_loader {
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
                let $defs_field = util::rs3::parse_defs(cache, $arc_id)?;

                Ok($ldr { $defs_field })
            }

            #[inline]
            fn load(&self, id: u16) -> Option<&$def> {
                self.$defs_field.get(&id)
            }
        }
   };
}

#[inline]
pub fn parse_defs<T: Definition, S: Store>(cache: &Cache<S>, archive_id: u32) -> crate::Result<HashMap<u16, T>> {
    let buffer = cache.read(REFERENCE_TABLE, archive_id)?;
    let buffer = codec::decode(&buffer)?;

    let archives = arc::parse_archive_data(&buffer)?;
    let mut definitions = std::collections::HashMap::new();
    let mut base_id = 0;

    for archive in &archives {
        let buffer = cache.read(archive_id as u8, archive.id as u32)?;
        let buffer = codec::decode(&buffer)?;

        let archive_data = arc::parse_content(&buffer, archive.entry_count)?;

        for (index, data) in archive_data {
            let id = base_id + archive.valid_ids[index as usize] as usize;
            definitions.insert(id as u16, T::new(id as u16, &data)?);
        }

        base_id += 256;
    }

    Ok(definitions)
}