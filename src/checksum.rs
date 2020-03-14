use crate::{ CacheError, Container, CompressionType };

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
    #[inline]
    pub const fn new() -> Self {
        Self { entries: Vec::new() }
    }

    #[inline]
    pub fn push(&mut self, entry: Entry) {
        self.entries.push(entry);
    }

    #[inline]
    pub fn validate_crcs(&self, crcs: &[u32]) -> bool {
        let owned_crcs: Vec<u32> = self.entries.iter()
            .map(|entry| entry.crc)
            .collect();
        
        owned_crcs == crcs
    }

    #[inline]
    pub fn encode(self) -> Result<Vec<u8>, CacheError> {
        let mut buffer = Vec::with_capacity(self.entries.len() * 2 * 4);

		for entry in self.entries {
            buffer.extend_from_slice(&u32::to_be_bytes(entry.crc));
            buffer.extend_from_slice(&u32::to_be_bytes(entry.revision));
        }

		Ok(Container::new(CompressionType::None, buffer, -1).compress()?)
    }
}