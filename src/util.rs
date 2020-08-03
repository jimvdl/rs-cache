use std::{ 
    path::Path, 
    collections::HashMap,
    fs::File,
    io::Read
};

use crate::{
    store::Store,
    cache::{ MAIN_DATA, IDX_PREFIX, REFERENCE_TABLE},
    idx::Index,
    Definition,
    Cache,
    codec,
    arc,
};

#[macro_export]
macro_rules! impl_loader {
   ($ldr:ident, $def:ty, $defs_field:ident, $arc_id:expr) => {
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
                let $defs_field = util::parse_defs(cache, $arc_id)?;

                Ok($ldr { $defs_field })
            }

            #[inline]
            fn load(&self, id: u16) -> Option<&$def> {
                self.$defs_field.get(&id)
            }
        }
   };
}

pub mod djd2 {
    #[inline]
    pub fn hash(string: &str) -> i32 {
        let mut hash = 0;

        for index in 0..string.len() {
            hash = string.chars()
                .nth(index).unwrap_or_else(|| panic!("index {} not valid in str len {}", index, string.len())) as i32 + ((hash << 5) - hash);
        }
        
        hash
    }
}

#[inline]
pub fn load_store<T: Store, P: AsRef<Path>>(path: P) -> crate::Result<T> {
    let path = path.as_ref();
    let main_file = File::open(path.join(MAIN_DATA))?;
    
    T::new(main_file)
}

#[inline]
pub fn load_indices<P: AsRef<Path>>(path: P) -> crate::Result<HashMap<u8, Index>> {
    let path = path.as_ref();
	let mut indices = HashMap::new();

	for index_id in 0..=REFERENCE_TABLE {
		let path = path.join(format!("{}{}", IDX_PREFIX, index_id));

		if path.exists() {
			let mut index = File::open(path)?;
			let mut index_buffer = Vec::new();

			index.read_to_end(&mut index_buffer)?;
			indices.insert(index_id, Index::new(&index_buffer));
		}
	}

	Ok(indices)
}

#[inline]
pub fn parse_defs<T: Definition, S: Store>(cache: &Cache<S>, archive_id: u32) -> crate::Result<HashMap<u16, T>> {
    let buffer = cache.read(REFERENCE_TABLE, 2)?;
    let buffer = codec::decode(&buffer)?;
    
    let archives = arc::parse(&buffer)?;
    let entry_count = archives[archive_id as usize - 1].entry_count;
    
    let buffer = cache.read(2, archive_id)?;
    let buffer = codec::decode(&buffer)?;

    let archive_data = arc::decode(&buffer, entry_count)?;

    let mut definitions = HashMap::new();
    for (id, buffer) in archive_data {
        definitions.insert(id, T::new(id, &buffer)?);
    }

    Ok(definitions)
}