use crate::{ codec::Compression, codec };

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
    pub(crate) const fn new() -> Self {
        Self { entries: Vec::new() }
    }

    pub(crate) fn push(&mut self, entry: Entry) {
        self.entries.push(entry);
    }

    #[inline]
    pub fn validate(&self, crcs: &[u32]) -> bool {
        let internal: Vec<u32> = self.entries.iter()
            .map(|entry| entry.crc)
            .collect();
        
            internal == crcs
    }

    #[inline]
    pub fn encode(self) -> crate::Result<Vec<u8>> {
        let mut buffer = Vec::with_capacity(self.entries.len() * 2 * 4);

		for entry in self.entries {
            buffer.extend_from_slice(&u32::to_be_bytes(entry.crc));
            buffer.extend_from_slice(&u32::to_be_bytes(entry.revision));
        }

        Ok(codec::encode(Compression::None, &buffer, None)?)
    }
}