//! Helpful utility functions, macros and structs.

#[allow(unused_assignments)]
mod huffman;
#[allow(clippy::many_single_char_names, clippy::too_many_lines)]
mod isaac_rand;

pub use huffman::Huffman;
pub use isaac_rand::IsaacRand;

use std::{
    collections::HashMap,
    io::{self, BufReader},
};

use crate::extension::ReadExt;

macro_rules! impl_osrs_loader {
    ($ldr:ident, $def:ty, index_id: $idx_id:expr $(, archive_id: $arc_id:expr)?) => {
        impl $ldr {
            #[allow(unreachable_code)]
            pub fn new(cache: &Cache) -> crate::Result<Self> {
                $(
                    let map = <$def>::fetch_from_archive(cache, $idx_id, $arc_id)?;

                    return Ok(Self(map));
                )?

                let map = <$def>::fetch_from_index(cache, $idx_id)?;

                Ok(Self(map))
            }

            pub fn load(&self, id: u16) -> Option<&$def> {
                self.0.get(&id)
            }
        }

        impl_iter_for_loader!($ldr, u16, $def);
    };
}

#[cfg(feature = "rs3")]
macro_rules! impl_rs3_loader {
    ($ldr:ident, $def:ty, index_id: $idx_id:expr) => {
        impl $ldr {
            pub fn new(cache: &Cache) -> crate::Result<Self> {
                let map = <$def>::fetch_from_index(cache, $idx_id)?;

                Ok(Self(map))
            }

            pub fn load(&self, id: u32) -> Option<&$def> {
                self.0.get(&id)
            }
        }

        impl_iter_for_loader!($ldr, u32, $def);
    };
}

macro_rules! impl_iter_for_loader {
    ($ldr:ident, $id:ty, $def:ty) => {
        impl $ldr {
            #[inline]
            pub fn iter(&self) -> hash_map::Iter<'_, $id, $def> {
                self.0.iter()
            }

            #[inline]
            pub fn iter_mut(&mut self) -> hash_map::IterMut<'_, $id, $def> {
                self.0.iter_mut()
            }
        }

        impl IntoIterator for $ldr {
            type Item = ($id, $def);
            type IntoIter = hash_map::IntoIter<$id, $def>;

            #[inline]
            fn into_iter(self) -> Self::IntoIter {
                self.0.into_iter()
            }
        }

        impl<'a> IntoIterator for &'a $ldr {
            type Item = (&'a $id, &'a $def);
            type IntoIter = hash_map::Iter<'a, $id, $def>;
            #[inline]
            fn into_iter(self) -> Self::IntoIter {
                self.0.iter()
            }
        }

        impl<'a> IntoIterator for &'a mut $ldr {
            type Item = (&'a $id, &'a mut $def);
            type IntoIter = hash_map::IterMut<'a, $id, $def>;

            #[inline]
            fn into_iter(self) -> Self::IntoIter {
                self.0.iter_mut()
            }
        }
    };
}

/// djd2 module for string hashing
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
    pub fn hash<T: AsRef<str>>(string: T) -> i32 {
        let string = string.as_ref();
        let mut hash = 0;

        for index in 0..string.len() {
            hash =
                string.chars().nth(index).unwrap_or_else(|| {
                    panic!("index {} not valid in str len {}", index, string.len())
                }) as i32
                    + ((hash << 5) - hash);
        }
        hash
    }
}

/// Useful for decoding parameters when reading from definition buffers.
///
/// # Errors
///
/// Can return `std::io::Error` if reading from the `BufReader<&[u8]>` fails.
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
