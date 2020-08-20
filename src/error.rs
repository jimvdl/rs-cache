//! Error management.

use std::{ error::Error, fmt, io };

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
/// use rscache::OsrsCache;
/// use rscache::codec;
/// 
/// // Same result as Result<Vec<u8>, CacheError>
/// fn item_def_data(cache: &OsrsCache) -> rscache::Result<Vec<u8>> {
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
#[derive(Debug)]
pub enum CacheError {
	/// Wrapper for the std::io::Error type.
	Io(io::Error),
	Read(ReadError),
	Compression(CompressionError),
	/// Clarification error for failed parsers.
	Parse(ParseError)
}

macro_rules! impl_from {
	($ty:path, $var:ident) => {
		impl From<$ty> for CacheError {
			#[inline]
			fn from(err: $ty) -> Self {
				Self::$var(err)
			}
		}
	};
}

impl_from!(io::Error, Io);
impl_from!(ReadError, Read);
impl_from!(CompressionError, Compression);
impl_from!(ParseError, Parse);

impl From<nom::Err<()>> for CacheError {
	#[inline]
	fn from(err: nom::Err<()>) -> Self {
		dbg!(err);

		Self::Parse(ParseError::Unknown)
	}
}

impl Error for CacheError {
	#[inline]
	fn source(&self) -> Option<&(dyn Error + 'static)> {
		match self {
			Self::Io(err) => Some(err),
			Self::Read(err) => Some(err),
			Self::Compression(err) => Some(err),
			Self::Parse(err) => Some(err),
		}
	}
}

impl fmt::Display for CacheError {
	#[inline]
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Self::Io(err) => err.fmt(f),
			Self::Read(err) => err.fmt(f),
			Self::Compression(err) => err.fmt(f),
			Self::Parse(err) => err.fmt(f),
		}
	}
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum ReadError {
	IndexNotFound(u8),
	ArchiveNotFound(u8, u32),
	NameNotInArchive(i32, String, u8),
	SectorArchiveMismatch(u32, u32),
	SectorChunkMismatch(u16, u16),
	SectorNextMismatch(u32, u32),
	SectorIndexMismatch(u8, u8),
}

impl Error for ReadError {}

impl fmt::Display for ReadError {
	#[inline]
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Self::IndexNotFound(id) => write!(f, "Index {} not found.", id),
			Self::ArchiveNotFound(index_id, archive_id) => write!(f, "Index {} does not contain archive group {}.", index_id, archive_id),
			Self::NameNotInArchive(hash, name, index_id) => write!(f, "Identifier hash {} for name {} not found in index {}.", hash, name, index_id),
			Self::SectorArchiveMismatch(received, expected) => write!(f, "Sector archive id was {} but expected {}.", received, expected),
			Self::SectorChunkMismatch(received, expected) => write!(f, "Sector chunk was {} but expected {}.", received, expected),
			Self::SectorNextMismatch(received, expected) => write!(f, "Sector next was {} but expected {}.", received, expected),
			Self::SectorIndexMismatch(received, expected) => write!(f, "Sector parent index id was {} but expected {}.", received, expected),
		}
	}
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum CompressionError {
	Unsupported(u8),
}

impl Error for CompressionError {}

impl fmt::Display for CompressionError {
	#[inline]
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Self::Unsupported(compression) => write!(f, "Invalid compression: {} is unsupported.", compression),
		}
	}
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum ParseError {
	Unknown,
	Archive(u32),
	Sector(u32),
}

impl Error for ParseError {}

impl fmt::Display for ParseError {
	#[inline]
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Self::Unknown => write!(f, "Unknown parser error."),
			Self::Archive(id) => write!(f, "Unable to parse archive {}, unexpected eof.", id),
			Self::Sector(id) => write!(f, "Unable to parse child sector of parent {}, unexpected eof.", id),
		}
	}
}