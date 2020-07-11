use std::{
    collections::LinkedList,
    io,
    io::{ Read, BufReader },
    marker::Sized,
};

use crate::{ Cache, CacheError };

/// Adds definition loading.
pub trait Loader<T> {
    fn new(cache: &Cache) -> Result<Self, CacheError> where Self: Sized;
    fn load(&self, id: u16) -> Option<&T>;
}

/// Adds extensions onto the std collection: [`LinkedList<T>`].
/// 
/// [`LinkedList<T>`]: https://doc.rust-lang.org/std/collections/struct.LinkedList.html
pub trait LinkedListExt {
    fn to_vec(&self) -> Vec<u8>;
}

/// Adds easy byte reading onto a [`Read`] instance.
/// 
/// [`Read`]: https://doc.rust-lang.org/std/io/trait.Read.html
pub trait ReadExt: Read {
    fn read_u8(&mut self) -> io::Result<u8>;
    fn read_i8(&mut self) -> io::Result<i8>;
    fn read_u16(&mut self) -> io::Result<u16>;
    fn read_u24(&mut self) -> io::Result<u32>;
    fn read_i32(&mut self) -> io::Result<i32>;
    fn read_string(&mut self) -> io::Result<String>;
}

impl LinkedListExt for LinkedList<&[u8]> {
    /// Iterates through the `LinkedList<&[u8]>` and copies the bytes into one owned buffer.
    /// 
    /// This function allocates a new buffer that is exactly large enough to hold all the slices 
    /// combined that the `LinkedList` references.
    /// [`copy_from_slice()`] is used for high performance and to keep the amount of copies to a 
    /// minimum.
    /// 
    /// # Examples
    /// 
    /// Detailed example:
    /// 
    /// ```
    /// # use rscache::{ Cache, CacheError };
    /// use std::collections::LinkedList;
    /// use rscache::LinkedListExt;
    /// 
    /// # fn main() -> Result<(), CacheError> {
    /// # let path = "./data/cache";
    /// # let cache = Cache::new(path)?;
    /// # let index_id = 2; // Config index
    /// # let archive_id = 10; // Random archive.
    /// let buffer: LinkedList<&[u8]> = cache.read(index_id, archive_id)?;
    /// let buffer: Vec<u8> = buffer.to_vec();
    /// # Ok(())
    /// # }
    /// ```
    /// 
    /// Which can also be written as:
    /// 
    /// ```
    /// # use rscache::{ Cache, CacheError };
    /// # use std::collections::LinkedList;
    /// # use rscache::LinkedListExt;
    /// # fn main() -> Result<(), CacheError> {
    /// # let path = "./data/cache";
    /// # let cache = Cache::new(path)?;
    /// # let index_id = 2; // Config index
    /// # let archive_id = 10; // Random archive.
    /// let buffer = cache.read(index_id, archive_id)?.to_vec();
    /// # Ok(())
    /// # }
    /// ```
    /// 
    /// [`copy_from_slice()`]: https://doc.rust-lang.org/std/primitive.slice.html#method.copy_from_slice
    #[inline]
	fn to_vec(&self) -> Vec<u8> {
		let size = self.iter()
            .map(|data_block| data_block.len())
            .sum();

        let mut buffer = vec![0; size];
        let mut current = 0;

        for data_block in self {
            buffer[current..current + data_block.len()].copy_from_slice(data_block);
            current += data_block.len();
        }

        buffer
	}
}

impl ReadExt for BufReader<&[u8]> {
    #[inline]
    fn read_u8(&mut self) -> io::Result<u8> {
        let mut buffer = [0; 1];
        self.read_exact(&mut buffer)?;

        Ok(u8::from_be_bytes(buffer))
    }
    
    #[inline]
    fn read_i8(&mut self) -> io::Result<i8> {
        let mut buffer = [0; 1];
        self.read_exact(&mut buffer)?;

        Ok(i8::from_be_bytes(buffer))
    }
    
    #[inline]
    fn read_u16(&mut self) -> io::Result<u16> {
        let mut buffer = [0; 2];
        self.read_exact(&mut buffer)?;

        Ok(u16::from_be_bytes(buffer))
    }
    
    #[inline]
    fn read_u24(&mut self) -> io::Result<u32> {
        let mut buffer = [0; 3];
        self.read_exact(&mut buffer)?;

        Ok(((buffer[0] as u32) << 16) | ((buffer[1] as u32) << 8) | (buffer[2] as u32))
    }
    
    #[inline]
    fn read_i32(&mut self) -> io::Result<i32> {
        let mut buffer = [0; 4];
        self.read_exact(&mut buffer)?;

        Ok(i32::from_be_bytes(buffer))
    }
    
    #[inline]
    fn read_string(&mut self) -> io::Result<String> {
        let mut bytes = Vec::new();
    
        loop {
            let mut buffer = [0; 1];
            self.read_exact(&mut buffer)?;
            let byte = u8::from_be_bytes(buffer);
            if byte != 0 {
                bytes.push(byte);
            } else {
                break;
            }
        }
    
        Ok(String::from_utf8_lossy(&bytes[..]).to_string())
    }
}