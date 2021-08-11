//! Helpful utility functions, macros and structs.

/// All OSRS specific utilities.
#[macro_use]
pub mod osrs;
/// All RS3 specific utilities.
#[macro_use]
pub mod rs3;

use std::{ 
    path::Path, 
    collections::HashMap,
    fs::File,
    io::{ BufReader, self, Read },
};

use crate::{
    store::Store,
    cache::{ MAIN_DATA, IDX_PREFIX, REFERENCE_TABLE},
    idx::Index,
    ext::ReadExt,
    error::ReadError,
};

macro_rules! impl_iter_for_loader {
    ($ldr:ident, $def:ty, $defs_field:ident) => {
        impl $ldr {
            #[inline]
            pub fn iter(&self) -> hash_map::Iter<'_, u32, $def> {
                self.$defs_field.iter()
            }

            #[inline]
            pub fn iter_mut(&mut self) -> hash_map::IterMut<'_, u32, $def> {
                self.$defs_field.iter_mut()
            }
        }

        impl<'a> Iterator for &'a $ldr {
            type Item = (&'a u32, &'a $def);
        
            #[inline]
            fn next(&mut self) -> Option<Self::Item> {
                self.$defs_field.iter().next()
            }
        }

        impl IntoIterator for $ldr {
            type Item = (u32, $def);
            type IntoIter = hash_map::IntoIter<u32, $def>;

            #[inline]
            fn into_iter(self) -> Self::IntoIter {
                self.$defs_field.into_iter()
            }
        }

        impl<'a> IntoIterator for &'a mut $ldr {
            type Item = (&'a u32, &'a mut $def);
            type IntoIter = hash_map::IterMut<'a, u32, $def>;

            #[inline]
            fn into_iter(self) -> Self::IntoIter {
                self.$defs_field.iter_mut()
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
    pub fn hash<T: Into<String>>(string: T) -> i32 {
        let string = string.into();
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
/// let store: CustomStore = util::load_store("./data/osrs_cache")?;
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

    let ref_tbl_path = path.join(format!("{}{}", IDX_PREFIX, REFERENCE_TABLE));
    if !ref_tbl_path.exists() {
        return Err(ReadError::ReferenceTableNotFound.into());
    }

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