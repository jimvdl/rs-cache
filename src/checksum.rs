use crate::{ CacheError, compression::Compression, compression };

#[derive(Debug, Clone)]
pub struct Entry {
    pub crc: u32,
    pub revision: u32,
}

#[derive(Clone, Debug, Default)]
pub struct Checksum {
    entries: Vec<Entry>
}

impl Checksum {
    /// Constructs a new, empty Checksum.
    /// 
    /// If you want to validate crc values from the cache use the `create_checksum()` 
    /// function on the `Cache` struct instead.
    /// 
    /// # Examples
    /// 
    /// ```
    /// # use rscache::Checksum;
    /// let checksum = Checksum::new();
    /// ```
    #[inline]
    pub const fn new() -> Self {
        Self { entries: Vec::new() }
    }

    #[inline]
    pub fn push(&mut self, entry: Entry) {
        self.entries.push(entry);
    }

    /// Validates the given crcs with internal crcs of the `Checksum`.
    /// 
    /// # Examples
    /// 
    /// ```
    /// # use rscache::{ Cache, CacheError };
    /// # fn main() -> Result<(), CacheError> {
    /// # let path = "./data/cache";
    /// # let cache = Cache::new(path)?;
    /// # let checksum = cache.create_checksum()?;
    /// // client crcs:
    /// let crcs = vec![1593884597, 1029608590, 16840364, 4209099954, 3716821437, 165713182, 
    ///                 686540367, 4262755489, 2208636505, 3047082366, 586413816, 2890424900, 
    ///                 3411535427, 3178880569, 153718440, 3849392898, 0, 2813112885, 1461700456, 
    ///                 2751169400, 2927815226];
    /// 
    /// let valid = checksum.validate_crcs(&crcs);
    /// 
    /// assert_eq!(valid, true);
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn validate_crcs(&self, crcs: &[u32]) -> bool {
        let owned_crcs: Vec<u32> = self.entries.iter()
            .map(|entry| entry.crc)
            .collect();
        
        owned_crcs == crcs
    }

    /// Consumes the `Checksum` and encodes it into a byte buffer.
    /// 
    /// After encoding the checksum it can be send to the client.
    /// 
    /// # Errors
    /// 
    /// Returns a `CacheError` if the encoding fails.
    /// 
    /// # Examples
    /// 
    /// ```
    /// # use rscache::{ Cache, CacheError, Checksum };
    /// # use std::net::TcpStream;
    /// # use std::io::Write;
    /// fn encode_checksum(checksum: Checksum, stream: &mut TcpStream) -> Result<(), CacheError> {
    ///     let buffer = checksum.encode()?;
    /// 
    ///     stream.write_all(&buffer)?;
    ///     Ok(())
    /// }
    /// ```
    #[inline]
    pub fn encode(self) -> Result<Vec<u8>, CacheError> {
        let mut buffer = Vec::with_capacity(self.entries.len() * 2 * 4);

		for entry in self.entries {
            buffer.extend_from_slice(&u32::to_be_bytes(entry.crc));
            buffer.extend_from_slice(&u32::to_be_bytes(entry.revision));
        }

        Ok(compression::compress(Compression::None, &buffer, None)?)
    }
}