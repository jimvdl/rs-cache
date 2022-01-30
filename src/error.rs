//! Error management.

use runefs::Error as RuneFsError;
use std::io;
use thiserror::Error;

pub(crate) type Result<T> = std::result::Result<T, Error>;

/// Super error type for all cache errors.
#[derive(Error, Debug)]
pub enum Error {
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
    #[error("expected crc length of {expected} but was {actual}")]
    InvalidLength {
        expected: usize, 
        actual: usize,
    },
    #[error("mismatch crc at index {idx}, expected {internal} but was {external}")]
    InvalidCrc {
        idx: usize,
        internal: u32,
        external: u32,
    },
}
