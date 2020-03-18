use std::collections::LinkedList;

/// Adds extensions onto the std collection: [`LinkedList<T>`].
/// 
/// [`LinkedList<T>`]: https://doc.rust-lang.org/std/collections/struct.LinkedList.html
pub trait LinkedListExt {
    fn to_vec(&self) -> Vec<u8>;
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