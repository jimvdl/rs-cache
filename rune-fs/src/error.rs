//! Error management.

use std::io;
use thiserror::Error;

/// A specialized result type for cache operations.
///
/// This type is broadly used across rscache for any operation which may produce a
/// [RuneFsError](enum.RuneFsError.html).
///
/// # Examples
///
/// A convenience function that bubbles an `rscache::Result` to its caller:
///
/// ```
/// use rscache::Cache;
/// use rscache::codec;
///
/// // Same result as Result<Vec<u8>, RuneFsError>
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
pub(crate) type Result<T> = std::result::Result<T, Error>;

/// Super error type for all sub cache errors
#[derive(Error, Debug)]
pub enum Error {
    /// Wrapper for the std::io::Error type.
    #[error(transparent)]
    Io(#[from] io::Error),
    #[error(transparent)]
    Read(#[from] ReadError),
    #[error(transparent)]
    Compression(#[from] CompressionUnsupported),
    /// Clarification error for failed parsers.
    #[error(transparent)]
    Parse(#[from] ParseError),
}

impl From<nom::Err<()>> for Error {
    #[inline]
    fn from(_: nom::Err<()>) -> Self {
        Self::Parse(ParseError::Unknown)
    }
}

#[derive(Error, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum ReadError {
    #[error("index {0} not found")]
    IndexNotFound(u8),
    #[error("index {idx} does not contain archive group {arc}")]
    ArchiveNotFound { idx: u8, arc: u32 },
    #[error("reference table (index 255) not found")]
    ReferenceTableNotFound,
    #[error("sector archive id was {0} but expected {1}")]
    SectorArchiveMismatch(u32, u32),
    #[error("sector chunk was {0} but expected {1}")]
    SectorChunkMismatch(usize, usize),
    #[error("sector next was {0} but expected {1}")]
    SectorNextMismatch(u32, u32),
    #[error("sector parent index id was {0} but expected {1}")]
    SectorIndexMismatch(u8, u8),
}

#[derive(Error, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[error("unsupported compression type {0}")]
pub struct CompressionUnsupported(pub(crate) u8);

#[derive(Error, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum ParseError {
    #[error("unknown parser error")]
    Unknown,
    #[error("unable to parse archive {0}, unexpected eof")]
    Archive(u32),
    #[error("unable to parse child sector of parent {0}, unexpected eof")]
    Sector(usize),
}
