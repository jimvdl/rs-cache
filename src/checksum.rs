//! Validator for the cache.
//!
//! # Example
//!
//! ```
//! # use rscache::Cache;
//! use rscache::checksum::{Checksum};
//!
//! # fn main() -> rscache::Result<()> {
//! # let cache = Cache::new("./data/osrs_cache")?;
//! let checksum = Checksum::new(&cache)?;
//!
//! // Encode the checksum with the OSRS protocol.
//! let buffer = checksum.encode()?;
//! # Ok(())
//! # }
//! ```

use std::iter::IntoIterator;
use std::slice::Iter;

use crate::{codec, codec::Compression, error::ValidateError, Cache, REFERENCE_TABLE};
use crc::{Crc, CRC_32_ISO_HDLC};
use nom::{combinator::cond, number::complete::be_u32};

#[cfg(feature = "rs3")]
use num_bigint::{BigInt, Sign};
#[cfg(feature = "serde-derive")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "rs3")]
use whirlpool::{Digest, Whirlpool};

const CRC: Crc<u32> = Crc::<u32>::new(&CRC_32_ISO_HDLC);

/// Contains index validation data.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[cfg_attr(feature = "serde-derive", derive(Serialize, Deserialize))]
pub struct Entry {
    pub crc: u32,
    pub version: u32,
    #[cfg(feature = "rs3")]
    pub hash: Vec<u8>,
}

// TODO: fix documentation
/// Validator for the `Cache`.
///
/// Used to validate cache index files. It contains a list of entries, one entry for each index file.
///
/// In order to create the `Checksum` the
/// [create_checksum()](../struct.Cache.html#method.create_checksum) function has to be
/// called on `Cache`.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
#[cfg_attr(feature = "serde-derive", derive(Serialize, Deserialize))]
pub struct Checksum<'a> {
    index_count: usize,
    entries: Vec<Entry>,
    rsa_keys: Option<RsaKeys<'a>>,
}

impl<'a> Checksum<'a> {
    #[inline]
    pub fn new(cache: &Cache) -> crate::Result<Self> {
        Self::new_internal(cache, None)
    }

    #[cfg(any(feature = "rs3", doc))]
    #[inline]
    pub fn with_rsa(cache: &Cache, rsa_keys: RsaKeys<'a>) -> crate::Result<Self> {
        Self::new_internal(cache, Some(rsa_keys))
    }

    fn new_internal(cache: &Cache, rsa_keys: Option<RsaKeys<'a>>) -> crate::Result<Self> {
        let entries: Vec<Entry> = (0..cache.indices.len())
            .into_iter()
            .filter_map(|idx_id| cache.read(REFERENCE_TABLE, idx_id as u32).ok())
            .enumerate()
            .map(|(idx_id, buffer)| -> crate::Result<Entry> {
                if buffer.is_empty() || idx_id == 47 {
                    Ok(Entry::default())
                } else {
                    let data = codec::decode(&buffer)?;
                    let (_, version) = cond(data[0] >= 6, be_u32)(&data[1..5])?;
                    let version = version.unwrap_or(0);

                    #[cfg(feature = "rs3")]
                    let hash = {
                        let mut hasher = Whirlpool::new();
                        hasher.update(&buffer);
                        hasher.finalize().as_slice().to_vec()
                    };

                    let mut digest = CRC.digest();
                    digest.update(&buffer);

                    Ok(Entry {
                        crc: digest.finalize(),
                        version,
                        #[cfg(feature = "rs3")]
                        hash,
                    })
                }
            })
            .filter_map(crate::Result::ok)
            .collect();

        Ok(Self {
            index_count: cache.indices.len(),
            entries,
            rsa_keys,
        })
    }

    /// Consumes the `Checksum` and encodes it into a byte buffer.
    ///
    ///
    /// Note: It defaults to OSRS. RS3 clients use RSA to encrypt
    /// network traffic, which includes the checksum. When encoding for RS3 clients
    /// first call [`with_rsa_keys`](struct.Checksum.html#method.with_rsa_keys) to make
    /// the checksum aware of the clients keys.
    ///
    /// After encoding the checksum it can be sent to the client.
    ///
    /// # Errors
    ///
    /// Returns a `CacheError` if the encoding fails.
    #[inline]
    pub fn encode(self) -> crate::Result<Vec<u8>> {
        match self.rsa_keys {
            Some(_) => {
                #[cfg(feature = "rs3")]
                return self.encode_rs3();
                #[cfg(not(feature = "rs3"))]
                unreachable!()
            }
            None => self.encode_osrs(),
        }
    }

    // TODO: documentation and write fail tests for this. (also fix the rs3 tests)
    /// Validates crcs with internal crcs.
    ///
    /// Only returns `true` if both the length of the iterators are the same
    /// and all of its elements are `eq`.
    #[inline]
    pub fn validate<'b, I>(&self, crcs: I) -> crate::Result<()>
    where
        I: IntoIterator<Item = &'b u32>,
    {
        let mut crcs = crcs.into_iter();
        let crcs_len = crcs.by_ref().count();
        if self.entries.len() != crcs_len {
            return Err(ValidateError::InvalidLength(self.entries.len(), crcs_len).into());
        }

        for (index, (internal, external)) in self
            .entries
            .iter()
            .map(|entry| &entry.crc)
            .zip(crcs)
            .enumerate()
        {
            if internal != external {
                return Err(ValidateError::InvalidCrc(*internal, *external, index).into());
            }
        }

        Ok(())
    }

    fn encode_osrs(self) -> crate::Result<Vec<u8>> {
        let mut buffer = Vec::with_capacity(self.entries.len() * 8);

        for entry in self.entries {
            buffer.extend(&u32::to_be_bytes(entry.crc));
            buffer.extend(&u32::to_be_bytes(entry.version));
        }

        codec::encode(Compression::None, &buffer, None)
    }

    #[cfg(feature = "rs3")]
    fn encode_rs3(self) -> crate::Result<Vec<u8>> {
        let index_count = self.index_count - 1;
        let mut buffer = vec![0; 81 * index_count];

        buffer[0] = index_count as u8;
        for (index, entry) in self.entries.iter().enumerate() {
            let offset = index * 80;
            buffer[offset + 1..=offset + 4].copy_from_slice(&u32::to_be_bytes(entry.crc));
            buffer[offset + 5..=offset + 8].copy_from_slice(&u32::to_be_bytes(entry.version));
            buffer[offset + 9..=offset + 12].copy_from_slice(&u32::to_be_bytes(0));
            buffer[offset + 13..=offset + 16].copy_from_slice(&u32::to_be_bytes(0));
            buffer[offset + 17..=offset + 80].copy_from_slice(&entry.hash);
        }

        let mut hasher = Whirlpool::new();
        hasher.update(&buffer);
        let mut hash = hasher.finalize().as_slice().to_vec();
        hash.insert(0, 0);

        // unwrap here is safe because this function never executes if self.rsa_keys is not `Some`.
        let rsa_keys = self.rsa_keys.as_ref().unwrap();
        let exp = BigInt::parse_bytes(rsa_keys.exponent, 10).unwrap_or_default();
        let mud = BigInt::parse_bytes(rsa_keys.modulus, 10).unwrap_or_default();
        let rsa = BigInt::from_bytes_be(Sign::Plus, &hash)
            .modpow(&exp, &mud)
            .to_bytes_be()
            .1;

        buffer.extend(rsa);

        Ok(buffer)
    }

    #[inline]
    pub const fn index_count(&self) -> usize {
        self.index_count
    }

    #[inline]
    pub fn iter(&self) -> Iter<'_, Entry> {
        self.entries.iter()
    }
}

// TODO: documentation
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
#[cfg_attr(feature = "serde-derive", derive(Serialize, Deserialize))]
pub struct RsaKeys<'a> {
    pub(crate) exponent: &'a [u8],
    pub(crate) modulus: &'a [u8],
}

impl<'a> RsaKeys<'a> {
    #[inline]
    pub const fn new(exponent: &'a [u8], modulus: &'a [u8]) -> Self {
        Self { exponent, modulus }
    }
}

// impl IntoIterator for Checksum {
//     type Item = Entry;
//     type IntoIter = std::vec::IntoIter<Entry>;

//     #[inline]
//     fn into_iter(self) -> Self::IntoIter {
//         self.entries.into_iter()
//     }
// }

// impl<'a> IntoIterator for &'a Checksum {
//     type Item = &'a Entry;
//     type IntoIter = Iter<'a, Entry>;

//     #[inline]
//     fn into_iter(self) -> Self::IntoIter {
//         self.entries.iter()
//     }
// }

impl Default for Entry {
    #[inline]
    fn default() -> Self {
        Self {
            crc: 0,
            version: 0,
            #[cfg(feature = "rs3")]
            hash: vec![0; 64],
        }
    }
}
