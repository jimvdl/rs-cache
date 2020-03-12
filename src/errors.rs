use std::{ error::Error, fmt, io };

#[derive(Debug)]
pub enum CacheError {
	Io(io::Error),
	Read(ReadError),
	Compression(CompressionError),
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

impl Error for CacheError {
	#[inline]
	fn source(&self) -> Option<&(dyn Error + 'static)> {
		match self {
			Self::Io(err) => Some(err),
			Self::Read(err) => Some(err),
			Self::Compression(err) => Some(err),
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
		}
	}
}

#[derive(Debug)]
pub enum ReadError {
	IndexNotFound(u8),
	ArchiveNotFound(u8, u8),
	WhirlpoolUnsupported(),
	RefTblEntryNotFound(u8),
}

impl Error for ReadError {}

impl fmt::Display for ReadError {
	#[inline]
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Self::IndexNotFound(id) => write!(f, "Index {} was not found.", id),
			Self::ArchiveNotFound(archive_id, index_id) => write!(f, "Archive {} not found in index {}.", archive_id, index_id),
			Self::WhirlpoolUnsupported() => write!(f, "Whirlpool is currently unsupported."),
			Self::RefTblEntryNotFound(id) => write!(f, "Reference Table Entry {} not found.", id),
		}
	}
}

#[derive(Debug)]
pub enum CompressionError {
	UnsupportedType(u8),
	LengthMismatch(usize, usize),
}

impl Error for CompressionError {}

impl fmt::Display for CompressionError {
	#[inline]
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Self::UnsupportedType(tp) => write!(f, "Unsupported compression type {}.", tp),
			Self::LengthMismatch(expected, actual) => write!(f, "Uncompressed length mismatch: expected length {} but length was {}.", expected, actual),
		}
	}
}
