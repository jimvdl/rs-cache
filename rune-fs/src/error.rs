//! Error management.

use std::io;
use thiserror::Error;

pub(crate) type Result<T> = std::result::Result<T, Error>;

/// Super error type for all runefs errors.
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
