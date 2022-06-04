//! Validator for the cache.
//!
//! # Example
//!
//! ```
//! # use rscache::{Cache, error::Error};
//! use rscache::checksum::Checksum;
//!
//! # fn main() -> Result<(), Error> {
//! # let cache = Cache::new("./data/osrs_cache")?;
//! # let client_crcs = vec![1593884597, 1029608590, 16840364, 4209099954, 3716821437, 165713182, 686540367,
//! #                     4262755489, 2208636505, 3047082366, 586413816, 2890424900, 3411535427, 3178880569,
//! #                     153718440, 3849392898, 3628627685, 2813112885, 1461700456, 2751169400, 2927815226];
//! // Either one works
//! let checksum = cache.checksum()?;
//! let checksum = Checksum::new(&cache)?;
//!
//! checksum.validate(&client_crcs)?;
//!
//! // Encode the checksum with the OSRS protocol.
//! let buffer = checksum.encode()?;
//! # Ok(())
//! # }
//! ```

use std::iter::IntoIterator;
use std::slice::Iter;

use crate::{error::ValidateError, Cache};
use nom::{combinator::cond, number::complete::be_u32};
use runefs::{
    codec::{Buffer, Encoded},
    REFERENCE_TABLE_ID,
};

#[cfg(feature = "rs3")]
use num_bigint::{BigInt, Sign};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "rs3")]
use whirlpool::{Digest, Whirlpool};

/// Each entry in the checksum is mapped to an [`Index`](runefs::Index).
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(not(feature = "rs3"), derive(Default))]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Entry {
    pub(crate) crc: u32,
    pub(crate) version: u32,
    #[cfg(feature = "rs3")]
    pub(crate) hash: Vec<u8>,
}

/// Validator for the `Cache`.
///
/// Used to validate cache index files. It contains a list of entries, one entry for each index file.
///
/// In order to create a `Checksum` you can either use the [`checksum`](crate::Cache::checksum) function on `Cache` or
/// use [`new`](Checksum::new) and pass in a reference to an exisiting cache. They both achieve the same result.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Checksum {
    index_count: usize,
    entries: Vec<Entry>,
}

impl Checksum {
    /// Generate a checksum based on the given cache.
    /// 
    /// # Errors
    /// 
    /// Decoding of a index buffer fails, this is considered a bug.
    pub fn new(cache: &Cache) -> crate::Result<Self> {
        Ok(Self {
            index_count: cache.indices.count(),
            entries: Self::entries(cache)?,
        })
    }

    fn entries(cache: &Cache) -> crate::Result<Vec<Entry>> {
        let entries: Vec<Entry> = (0..cache.indices.count())
            .into_iter()
            .filter_map(|idx_id| cache.read(REFERENCE_TABLE_ID, idx_id as u32).ok())
            .enumerate()
            .map(|(idx_id, buffer)| -> crate::Result<Entry> {
                if buffer.is_empty() || idx_id == 47 {
                    Ok(Entry::default())
                } else {
                    // let (buffer, size) = if with_rsa {
                    //     be_u8(buffer.as_slice())?
                    // } else {
                    //     (buffer.as_slice(), (buffer.len() / 8) as u8)
                    // };

                    #[cfg(feature = "rs3")]
                    let hash = {
                        let mut hasher = Whirlpool::new();
                        hasher.update(&buffer);
                        hasher.finalize().as_slice().to_vec()
                    };

                    let checksum = crc32fast::hash(&buffer);

                    let data = buffer.decode()?;
                    let (_, version) = cond(data[0] >= 6, be_u32)(&data[1..5])?;
                    let version = version.unwrap_or(0);

                    Ok(Entry {
                        crc: checksum,
                        version,
                        #[cfg(feature = "rs3")]
                        hash,
                    })
                }
            })
            .filter_map(crate::Result::ok)
            .collect();

        Ok(entries)
    }

    /// Consumes the `Checksum` and encodes it into a byte buffer.
    ///
    /// 
    /// Note: It defaults to OSRS. RS3 clients use RSA to encrypt
    /// network traffic, which includes the checksum. When encoding for RS3 clients
    /// use [`RsaChecksum`](RsaChecksum) instead.
    ///
    /// After encoding the checksum it can be sent to the client.
    ///
    /// # Errors
    ///
    /// Encoding of the formatted buffer fails, this is considered a bug.
    pub fn encode(self) -> crate::Result<Buffer<Encoded>> {
        let mut buffer = Vec::with_capacity(self.entries.len() * 8);

        for entry in self.entries {
            buffer.extend(&u32::to_be_bytes(entry.crc));
            buffer.extend(&u32::to_be_bytes(entry.version));
        }

        // let mut buffer = codec::encode(Compression::None, &buffer, None)?;

        // #[cfg(feature = "whirlpool")]
        // {
        //     let mut hasher = Whirlpool::new();
        //     hasher.update(&buffer);
        //     let mut hash = hasher.finalize().as_slice().to_vec();
        //     hash.insert(0, 0);

        //     let rsa_keys = self.rsa_keys.as_ref().unwrap();
        //     let exp = BigInt::parse_bytes(rsa_keys.exponent, 10).unwrap_or_default();
        //     let mud = BigInt::parse_bytes(rsa_keys.modulus, 10).unwrap_or_default();
        //     let rsa = BigInt::from_bytes_be(Sign::Plus, &hash)
        //         .modpow(&exp, &mud)
        //         .to_bytes_be()
        //         .1;

        //     buffer.extend(rsa);
        // }

        // Ok(buffer)
        // Ok(codec::encode(Compression::None, &buffer, None)?)
        Ok(Buffer::from(buffer).encode()?)
    }

    /// Validates the given crcs from the client with the internal crcs of this cache.
    /// 
    /// # Errors
    /// 
    /// When the lengths of the crc iterators don't match up because too many or too few indices 
    /// were shared between the client and the server, or if a crc value mismatches.
    pub fn validate<'b, I>(&self, crcs: I) -> Result<(), ValidateError>
    where
        I: IntoIterator<Item = &'b u32>,
        <I as IntoIterator>::IntoIter: ExactSizeIterator,
    {
        let crcs = crcs.into_iter();

        if self.entries.len() != crcs.len() {
            return Err(ValidateError::InvalidLength {
                expected: self.entries.len(),
                actual: crcs.len(),
            });
        }
        for (index, (internal, external)) in self
            .entries
            .iter()
            .map(|entry| &entry.crc)
            .zip(crcs)
            .enumerate()
        {
            if internal != external {
                return Err(ValidateError::InvalidCrc {
                    idx: index,
                    internal: *internal,
                    external: *external,
                });
            }
        }

        Ok(())
    }

    #[allow(missing_docs)]
    #[inline]
    pub const fn index_count(&self) -> usize {
        self.index_count
    }

    #[allow(missing_docs)]
    #[inline]
    pub fn iter(&self) -> Iter<'_, Entry> {
        self.entries.iter()
    }
}

/// A struct that holds both keys for RSA encryption.
#[cfg(feature = "rs3")]
#[cfg_attr(docsrs, doc(cfg(feature = "rs3")))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct RsaKeys<'a> {
    pub(crate) exponent: &'a [u8],
    pub(crate) modulus: &'a [u8],
}

#[cfg(feature = "rs3")]
#[cfg_attr(docsrs, doc(cfg(feature = "rs3")))]
impl<'a> RsaKeys<'a> {
    /// Generate a RSA key set with the given keys.
    pub const fn new(exponent: &'a [u8], modulus: &'a [u8]) -> Self {
        Self { exponent, modulus }
    }

    /// Encrypts the given hash.
    // TODO: maybe make this panic if the exponent or modulus not line up
    pub fn encrypt(&self, hash: &[u8]) -> Vec<u8> {
        let exp = BigInt::parse_bytes(self.exponent, 10).unwrap_or_default();
        let mud = BigInt::parse_bytes(self.modulus, 10).unwrap_or_default();
        BigInt::from_bytes_be(Sign::Plus, hash)
            .modpow(&exp, &mud)
            .to_bytes_be()
            .1
    }
}

/// Wraps a general `Checksum` with the added benefit of encrypting
/// the whirlpool hash into the checksum buffer.
/// 
/// # Example
/// 
/// ```
/// # use rscache::{Cache, error::Error};
/// use rscache::checksum::{RsaChecksum, RsaKeys};
///
/// # fn main() -> Result<(), Error> {
/// # let cache = Cache::new("./data/osrs_cache")?;
/// # const EXPONENT: &'static [u8] = b"5206580307236375668350588432916871591810765290737810323990754121164270399789630501436083337726278206128394461017374810549461689174118305784406140446740993";
/// # const MODULUS: &'static [u8] = b"6950273013450460376345707589939362735767433035117300645755821424559380572176824658371246045200577956729474374073582306250298535718024104420271215590565201";
/// let keys = RsaKeys::new(EXPONENT, MODULUS);
/// 
/// // Either one works
/// let checksum = cache.checksum_with(keys)?;
/// // let checksum = RsaChecksum::with_keys(&cache, keys)?;
/// # Ok(())
/// # }
/// ```
#[cfg(feature = "rs3")]
#[cfg_attr(docsrs, doc(cfg(feature = "rs3")))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct RsaChecksum<'a> {
    checksum: Checksum,
    #[cfg_attr(feature = "serde", serde(borrow))]
    rsa_keys: RsaKeys<'a>,
}

#[cfg(feature = "rs3")]
impl<'a> RsaChecksum<'a> {
    /// Generate a checksum with RSA encryption support.
    pub fn with_keys(cache: &Cache, rsa_keys: RsaKeys<'a>) -> crate::Result<Self> {
        Ok(Self {
            checksum: Checksum::new(cache)?,
            rsa_keys,
        })
    }

    /// Same as [`Checksum::encode`](Checksum::encode) but for RS3.
    pub fn encode(self) -> crate::Result<Buffer<Encoded>> {
        let index_count = self.checksum.index_count - 1;
        let mut buffer = vec![0; 81 * index_count];

        buffer[0] = index_count as u8;
        for (index, entry) in self.checksum.entries.iter().enumerate() {
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

        buffer.extend(self.rsa_keys.encrypt(&hash));

        Ok(Buffer::from(buffer))
    }
}

#[cfg(feature = "rs3")]
impl<'a> From<(&'a [u8], &'a [u8])> for RsaKeys<'a> {
    fn from(keys: (&'a [u8], &'a [u8])) -> Self {
        RsaKeys::new(keys.0, keys.1)
    }
}

impl IntoIterator for Checksum {
    type Item = Entry;
    type IntoIter = std::vec::IntoIter<Entry>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.entries.into_iter()
    }
}

impl<'a> IntoIterator for &'a Checksum {
    type Item = &'a Entry;
    type IntoIter = Iter<'a, Entry>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.entries.iter()
    }
}

#[cfg(feature = "rs3")]
impl<'a> IntoIterator for RsaChecksum<'a> {
    type Item = Entry;
    type IntoIter = std::vec::IntoIter<Entry>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.checksum.entries.into_iter()
    }
}

#[cfg(feature = "rs3")]
impl<'a> IntoIterator for &'a RsaChecksum<'a> {
    type Item = &'a Entry;
    type IntoIter = Iter<'a, Entry>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.checksum.entries.iter()
    }
}

#[cfg(feature = "rs3")]
impl Default for Entry {
    #[inline]
    fn default() -> Self {
        Self {
            crc: 0,
            version: 0,
            hash: vec![0; 64],
        }
    }
}
