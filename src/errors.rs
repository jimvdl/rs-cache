use std::{ error::Error, fmt, io };

#[derive(Debug)]
pub enum CacheError {
	Io(io::Error),
	Read(ReadError),
	Compression(CompressionError),
	Parse(nom::Err<()>),
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
impl_from!(nom::Err<()>, Parse);

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
			Self::Parse(_) => write!(f, "Parsing failed."),
		}
	}
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum ReadError {
	IndexNotFound(u8),
	ArchiveNotFound(u8, u16),
	NameNotInArchive(i32, String, u8),
}

impl Error for ReadError {}

impl fmt::Display for ReadError {
	#[inline]
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Self::IndexNotFound(id) => write!(f, "Index {} not found.", id),
			Self::ArchiveNotFound(index_id, archive_id) => write!(f, "Index {} does not contain archive {}.", index_id, archive_id),
			Self::NameNotInArchive(hash, name, index_id) => write!(f, "Identifier hash {} for name {} not found in index {}.", hash, name, index_id),
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