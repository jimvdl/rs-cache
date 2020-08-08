use crate::{ codec::Compression, codec };

#[derive(Debug, Clone)]
pub struct Entry {
    pub crc: u32,
    pub revision: u32,
}

/// Validator for the `Cache`.
/// 
/// The `Checksum` is used to validate if every file used by the cache
/// is still valid. It contains a list of entries, one entry for each index file.
/// Every entry contains a crc and a revision.
/// 
/// In order to create the `Checksum` the 
/// [create_checksum()](struct.Cache.html#method.create_checksum) function has to be 
/// called on `Cache`. 
#[derive(Clone, Debug, Default)]
pub struct Checksum {
    entries: Vec<Entry>
}

impl Checksum {
    pub(crate) const fn new() -> Self {
        Self { entries: Vec::new() }
    }

    pub(crate) fn push(&mut self, entry: Entry) {
        self.entries.push(entry);
    }

    /// Validates the given crcs with internal crcs of the `Checksum`.
    /// 
    /// # Examples
    /// 
    /// ```
    /// # use rscache::OsrsCache;
    /// # fn main() -> rscache::Result<()> {
    /// # let path = "./data/cache";
    /// # let cache = OsrsCache::new(path)?;
    /// # let checksum = cache.create_checksum()?;
    /// // client crcs:
    /// let crcs = vec![1593884597, 1029608590, 16840364, 4209099954, 3716821437, 165713182, 
    ///                 686540367, 4262755489, 2208636505, 3047082366, 586413816, 2890424900, 
    ///                 3411535427, 3178880569, 153718440, 3849392898, 0, 2813112885, 1461700456, 
    ///                 2751169400, 2927815226];
    /// 
    /// let valid = checksum.validate(&crcs);
    /// 
    /// assert!(valid);
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn validate(&self, crcs: &[u32]) -> bool {
        let internal: Vec<u32> = self.entries.iter()
            .map(|entry| entry.crc)
            .collect();
        
            internal == crcs
    }

    /// Consumes the `Checksum` and encodes it into a byte buffer.
    /// 
    /// After encoding the checksum it can be sent to the client.
    /// 
    /// # Errors
    /// 
    /// Returns a `CacheError` if the encoding fails.
    /// 
    /// # Examples
    /// 
    /// ```
    /// # use rscache::Checksum;
    /// # use std::net::TcpStream;
    /// # use std::io::Write;
    /// fn encode_checksum(checksum: Checksum, stream: &mut TcpStream) -> rscache::Result<()> {
    ///     let buffer = checksum.encode()?;
    /// 
    ///     stream.write_all(&buffer)?;
    ///     Ok(())
    /// }
    /// ```
    #[inline]
    pub fn encode(self) -> crate::Result<Vec<u8>> {
        let mut buffer = Vec::with_capacity(self.entries.len() * 8);

		for entry in self.entries {
            buffer.extend(&u32::to_be_bytes(entry.crc));
            buffer.extend(&u32::to_be_bytes(entry.revision));
        }

        Ok(codec::encode(Compression::None, &buffer, None)?)
    }
}