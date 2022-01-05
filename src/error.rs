//! Error management.

use runefs::error::RuneFsError;
use std::io;
use thiserror::Error;

/// A specialized result type for cache operations.
///
/// This type is broadly used across rscache for any operation which may produce a
/// [CacheError](enum.CacheError.html).
///
/// # Examples
///
/// A convenience function that bubbles an `rscache::Result` to its caller:
///
/// ```
/// use rscache::Cache;
/// use rscache::codec;
///
/// // Same result as Result<Vec<u8>, CacheError>
/// fn item_def_data(cache: &Cache) -> rscache::Result<Vec<u8>> {
///     let index_id = 2;
///     let archive_id = 10;
///
///     let buffer = cache.read(index_id, archive_id)?;
///     let buffer = codec::decode(&buffer)?;
///
///     Ok(buffer)
/// }
/// ```
pub type Result<T> = std::result::Result<T, CacheError>;

/// Super error type for all sub cache errors
#[derive(Error, Debug)]
pub enum CacheError {
    /// Wrapper for the std::io::Error type.
    #[error(transparent)]
    Io(#[from] io::Error),
    #[error(transparent)]
    NameHash(#[from] NameHashMismatch),
    #[error("unknown parser error")]
    Parse(#[from] nom::Err<()>),
    #[error(transparent)]
    Validate(#[from] ValidateError),
    #[error(transparent)]
    RuneFs(#[from] RuneFsError),
}

#[derive(Error, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[error("identifier hash {hash} for name \"{name}\" not found in index {idx}")]
pub struct NameHashMismatch {
    pub(crate) hash: i32,
    pub(crate) name: String,
    pub(crate) idx: u8,
}

#[derive(Error, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum ValidateError {
    #[error("expected crc length of {0} but was {1}")]
    InvalidLength(usize, usize),
    #[error("mismatch crc at index {idx}, expected {internal} but was {external}")]
    InvalidCrc {
        idx: usize,
        internal: u32,
        external: u32,
    },
}
