//! Helpful utility functions, macros and structs.

/// All OSRS specific utilities.
#[macro_use]
pub mod osrs;
/// All RS3 specific utilities.
#[macro_use]
pub mod rs3;
/// Default xtea decipher.
pub mod xtea;

use std::{ 
    collections::HashMap,
    io::{ self, BufReader },
};

use crate::ext::ReadExt;

macro_rules! impl_iter_for_loader {
    ($ldr:ident, $def:ty) => {
        impl $ldr {
            #[inline]
            pub fn iter(&self) -> hash_map::Iter<'_, u32, $def> {
                self.0.iter()
            }

            #[inline]
            pub fn iter_mut(&mut self) -> hash_map::IterMut<'_, u32, $def> {
                self.0.iter_mut()
            }
        }

        impl IntoIterator for $ldr {
            type Item = (u32, $def);
            type IntoIter = hash_map::IntoIter<u32, $def>;

            #[inline]
            fn into_iter(self) -> Self::IntoIter {
                self.0.into_iter()
            }
        }

        impl<'a> IntoIterator for &'a $ldr {
            type Item = (&'a u32, &'a $def);
            type IntoIter = hash_map::Iter<'a, u32, $def>;
        
            #[inline]
            fn into_iter(self) -> Self::IntoIter {
                self.0.iter()
            }
        }

        impl<'a> IntoIterator for &'a mut $ldr {
            type Item = (&'a u32, &'a mut $def);
            type IntoIter = hash_map::IterMut<'a, u32, $def>;

            #[inline]
            fn into_iter(self) -> Self::IntoIter {
                self.0.iter_mut()
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