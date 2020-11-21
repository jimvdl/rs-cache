//! Helpful utility functions, macros and structs.

/// All OSRS specific utilities.
pub mod osrs;

use std::{ 
    path::Path, 
    collections::HashMap,
    fs::File,
    io::{ BufReader, self, Read }
};

use crate::{
    store::Store,
    cache::{ MAIN_DATA, IDX_PREFIX, REFERENCE_TABLE},
    idx::Index,
    Definition,
    Cache,
    codec,
    arc,
    ext::ReadExt,
    def,
};

#[macro_use]
macro_rules! impl_loader {
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

/// djd2 module for hashing strings
pub mod djd2 {

    /// Hashes the string
    /// 
    /// # Errors
    /// 
    /// Can panic if `nth(n)` returns `None` if n >= strings iter length.
    /// 
    /// # Examples
    /// 
    /// ```
    /// let hash = rscache::util::djd2::hash("huffman");
    /// assert_eq!(hash, 1258058669);
    /// ``` 
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

/// Loads the given store.
/// 
/// This will load the main cache file and open the chosen store
/// with it.
/// 
/// # Errors
/// 
/// Returns an `std::io::Error` if the path is incorrect.
/// 
/// # Examples 
/// 
/// ```
/// # use std::{ fs::File, path::Path, collections::HashMap };
/// # use rscache::arc::Archive;
/// use rscache::{ Store, util };
/// 
/// # fn main() -> rscache::Result<()> {
/// let store: CustomStore = util::load_store("./data/cache")?;
/// # Ok(())
/// # }
/// 
/// 
/// struct CustomStore;
/// 
/// impl Store for CustomStore {
///     fn new(mut main_file: File) -> rscache::Result<Self> {
///         // snip
/// 
///         Ok(Self {  })
///     }
/// # fn read(&self, archive: &Archive) -> rscache::Result<Vec<u8>> {
/// # unimplemented!()
/// # }
/// }
/// ```
#[inline]
pub fn load_store<S: Store, P: AsRef<Path>>(path: P) -> crate::Result<S> {
    let path = path.as_ref();
    let main_file = File::open(path.join(MAIN_DATA))?;
    
    S::new(main_file)
}

/// Loads all indices present in the cache folder.
/// 
/// The `u8` in `HashMap<u8, Index>` represents the id of the index.
/// 
/// # Errors
/// 
/// Can return multiple errors: if the index couldnt be parsed or the index 
/// couldn't be opened.
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
			indices.insert(index_id, Index::new(index_id, &index_buffer)?);
		}
	}

	Ok(indices)
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
/// # let cache = OsrsCache::new("./data/cache")?;
/// let archive_id = 10; // Archive containing item definitions.
/// let item_defs: HashMap<u16, ItemDefinition> = util::parse_defs(&cache, archive_id)?;
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

#[inline]
pub fn load_map_def<S: Store>(cache: &Cache<S>, region_id: u16) -> crate::Result<Option<def::osrs::MapDefinition>> {
    let region_id = region_id as u32;
    let x = region_id >> 8;
    let y = region_id & 0xFF;

    if let Ok(map_archive) = cache.archive_by_name(5, &format!("m{}_{}", x, y)) {
        let buffer = cache.read_archive(&map_archive)?;
        let buffer = crate::codec::decode(&buffer)?;
        
        return Ok(Some(def::osrs::MapDefinition::new(x, y, &buffer)?))
    }

    Ok(None)
}

/// Useful for decoding parameters when reading from definition buffers.
/// 
/// # Errors
/// 
/// Can return `std::io::Error` if reading from the `BufReader<&[u8]>` fails.
#[inline]
pub fn read_parameters(reader: &mut BufReader<&[u8]>) -> io::Result<HashMap<u32, String>> {
    let len = reader.read_u8()?;
    let mut map = HashMap::new();

    for _ in 0..len {
        let is_string = reader.read_u8()? == 1;
        let key = reader.read_u24()?;
        
        let value = if is_string {
            reader.read_string()?
        } else {
            reader.read_i32()?.to_string()
        };

        map.insert(key, value);
    }

    Ok(map)
}