//! Provides two functions to encode/decode buffers.
//! 
//! 
//! Includes compression and decompression.
//! 
//! NOTE: decoding only works on buffers that are fetched 
//! from the cache. It cannot decode buffers that were encoded
//! using the `encode()` function.

use std::io::{ self, Read };
use std::convert::TryFrom;

use bzip2::{
	read::BzDecoder,
	write::BzEncoder,
};
use libflate::gzip::{
	Decoder,
	Encoder,
};

use crate::{ CacheError, CompressionError };

/// Compression types that are supported by the encode and decode functions.
/// 
/// If you select `Compression::None` when encoding the buffer is simply formatted.
/// The formatting is explained [here](fn.encode.html).
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum Compression {
	None,
	Bzip2,
	Gzip
}

impl From<Compression> for u8 {
	#[inline]
	fn from(compression: Compression) -> Self {
		match compression {
			Compression::None => 0,
			Compression::Bzip2 => 1,
			Compression::Gzip => 2,
		}
	}
}

impl TryFrom<u8> for Compression {
	type Error = CompressionError;

	#[inline]
	fn try_from(compression: u8) -> Result<Self, Self::Error> {
		match compression {
			0 => Ok(Self::None),
			1 => Ok(Self::Bzip2),
			2 => Ok(Self::Gzip),
			_ => Err(CompressionError::Unsupported(compression))
		}
	}
}

/// Encodes a buffer, with the selected `Compression` format. Revision is an optional argument
/// that encodes the version of this buffer into it, if no revision should be encoded
/// pass None.
/// 
/// The following process takes place when encoding:
/// 1. Compress the buffer with the selected compression format.
/// 2. Allocate a new buffer.
/// 3. Push the compression type as a byte into the new buffer.
/// 4. Push the length (u32) into the buffer of the compressed data from step 1.
/// 5. If a compression type was selected (and not `Compression::None`) insert the uncompressed length as u32.
/// 6. Extend the buffer with the compressed data.
/// 7. Add the `revision` as i16 if present.
/// 8. Encode complete.
/// 
/// Supported compression types:
/// - Gzip
/// - Bzip2
/// 
/// **NOTE: When compressing with gzip the header is removed 
/// before the compressed data is returned.
/// The encoded buffer will not contain the gzip header.**
/// 
/// # Errors
/// 
/// Returns an error if the data couldn't be compressed or is invalid.
/// 
/// # Examples
/// 
/// ```
/// # use rscache::Cache;
/// # use rscache::CacheError;
/// # use rscache::LinkedListExt;
/// use rscache::codec::Compression;
/// 
/// # fn main() -> Result<(), CacheError> {
/// # let buffer = vec![0; 20];
/// let encoded_buffer = rscache::codec::encode(Compression::Bzip2, &buffer, None)?;
/// 
/// assert_eq!(Compression::Bzip2 as u8, encoded_buffer[0]);
/// # Ok(())
/// # }
/// ```
#[inline]
pub fn encode(compression: Compression, data: &[u8], revision: Option<i16>) -> Result<Vec<u8>, CacheError> {
	let compressed_data = match compression {
		Compression::None => data.to_owned(),
		Compression::Bzip2 => compress_bzip2(data)?,
		Compression::Gzip => compress_gzip(data)?,
	};

	let mut buffer = Vec::new();
	buffer.push(compression as u8);
	buffer.extend_from_slice(&u32::to_be_bytes(compressed_data.len() as u32));
	
	if compression != Compression::None {
		buffer.extend_from_slice(&u32::to_be_bytes(data.len() as u32));
	}

	buffer.extend(compressed_data);

	let revision = revision.unwrap_or(-1);
	if revision != -1 {
		buffer.extend_from_slice(&i16::to_be_bytes(revision));
	}

	Ok(buffer)
}

/// Decodes a buffer. The buffer needs to have the `Read` trait implemented.
/// 
/// The following process takes place when decoding:
/// 1. Read the first byte to determine which compression should be used to decompress.
/// 2. Read the length of the rest of the buffer.
/// 3. Decompress the remaining bytes.
/// 
/// # Errors
/// 
/// Returns an error if the remaining bytes couldn't be decompressed.
/// 
/// # Examples
/// 
/// ```
/// # use rscache::Cache;
/// # use rscache::CacheError;
/// # use rscache::LinkedListExt;
/// use rscache::codec::Compression;
/// 
/// # fn main() -> Result<(), CacheError> {
/// # let path = "./data/cache";
/// # let cache = Cache::new(path)?;
/// let buffer = cache.read(2, 10)?.to_vec();
/// let decoded_buffer = rscache::codec::decode(&mut buffer.as_slice())?;
/// # Ok(())
/// # }
/// ```
#[inline]
pub fn decode<R: Read>(reader: &mut R) -> Result<Vec<u8>, CacheError> {
	let mut buf = [0; 1];
	reader.read_exact(&mut buf)?;
	let compression = Compression::try_from(buf[0])?;

	let mut buf = [0; 4];
	reader.read_exact(&mut buf)?;
	let len = u32::from_be_bytes(buf) as usize;

	let (_revision, buffer) = match compression {
		Compression::None => decompress_none(reader, len)?,
		Compression::Bzip2 => decompress_bzip2(reader, len)?,
		Compression::Gzip => decompress_gzip(reader, len)?,
	};

	Ok(buffer)
}

fn compress_bzip2(data: &[u8]) -> Result<Vec<u8>, io::Error> {
	let compressor = Encoder::new(data.to_owned())?;
	compressor.finish().into_result()
}

fn compress_gzip(data: &[u8]) -> Result<Vec<u8>, io::Error> {
	let compressor = BzEncoder::new(data.to_owned(), bzip2::Compression::Default);
	let mut compressed_data = compressor.finish()?;
	compressed_data.drain(0..4);

	Ok(compressed_data)
}

fn decompress_none<R: Read>(reader: &mut R, len: usize) -> Result<(i16, Vec<u8>), CacheError> {
	let mut compressed_data = vec![0; len];
	reader.read_exact(&mut compressed_data)?;

	Ok((read_revision(reader)?, compressed_data))
}

fn decompress_bzip2<R: Read>(reader: &mut R, len: usize) -> Result<(i16, Vec<u8>), CacheError> {
	let mut buf = [0; 4];
	reader.read_exact(&mut buf)?;
	let decompressed_len = u32::from_be_bytes(buf) as usize;

	let mut compressed_data = vec![0; len - 4];
	reader.read_exact(&mut compressed_data)?;

	let revision = read_revision(reader)?;

	compressed_data.insert(0, b'1');
	compressed_data.insert(0, b'h');
	compressed_data.insert(0, b'Z');
	compressed_data.insert(0, b'B');
	let mut decompressor = BzDecoder::new(&compressed_data[..]);
	let mut decompressed_data = vec![0; decompressed_len];
	decompressor.read_exact(&mut decompressed_data)?;

	Ok((revision, decompressed_data))
}

fn decompress_gzip<R: Read>(reader: &mut R, len: usize) -> Result<(i16, Vec<u8>), CacheError> {
	let mut buf = [0; 4];
	reader.read_exact(&mut buf)?;
	let decompressed_len = u32::from_be_bytes(buf) as usize;

	let mut compressed_data = vec![0; len - 4];
	reader.read_exact(&mut compressed_data)?;

	let revision = read_revision(reader)?;

	let mut decoder = Decoder::new(&compressed_data[..])?;
	let mut decompressed_data = vec![0; decompressed_len];
	decoder.read_exact(&mut decompressed_data)?;

	Ok((revision, decompressed_data))
}

fn read_revision<R: Read>(reader: &mut R) -> Result<i16, CacheError> {
	if let Some(remaining) = reader.bytes().size_hint().1 {
		if remaining >= 2 {
			let mut rev_buffer = [0; 2];
			reader.read_exact(&mut rev_buffer)?;
			return Ok(i16::from_be_bytes(rev_buffer))
		}
	}

	Ok(-1)
}