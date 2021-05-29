//! Compression and decompression of cache buffers.

use std::io::{ self, Read };
use std::convert::TryFrom;

use nom::{
    combinator::cond,
	number::complete::{
        be_u8,
		be_i16,
		be_u32,
    },
};
use bzip2::{
	read::BzDecoder,
	write::BzEncoder,
};
use libflate::gzip::{
	Decoder,
	Encoder,
};

use crate::error::CompressionError;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum Compression {
	None,
	Bzip2,
	Gzip,
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
/// use rscache::codec::Compression;
/// 
/// # fn main() -> rscache::Result<()> {
/// # let buffer = vec![0; 20];
/// let encoded_buffer = rscache::codec::encode(Compression::Bzip2, &buffer, None)?;
/// 
/// assert_eq!(Compression::Bzip2 as u8, encoded_buffer[0]);
/// # Ok(())
/// # }
/// ```
#[inline]
pub fn encode(compression: Compression, data: &[u8], revision: Option<i16>) -> crate::Result<Vec<u8>> {
	let compressed_data = match compression {
		Compression::None => data.to_owned(),
		Compression::Bzip2 => compress_bzip2(data)?,
		Compression::Gzip => compress_gzip(data)?,
	};

	let mut buffer = Vec::with_capacity(compressed_data.len() + 11);
	buffer.push(compression as u8);
	buffer.extend(&u32::to_be_bytes(compressed_data.len() as u32));
	
	if compression != Compression::None {
		buffer.extend(&u32::to_be_bytes(data.len() as u32));
	}

	buffer.extend(compressed_data);

	if let Some(revision) = revision {
		buffer.extend(&i16::to_be_bytes(revision));
	}

	Ok(buffer)
}

/// Decodes the buffer.
/// 
/// The following process takes place when decoding:
/// 1. Read the first byte to determine which compression type should be used to decompress.
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
/// # use rscache::{ Cache, store::MemoryStore };
/// use rscache::codec::Compression;
/// 
/// # fn main() -> rscache::Result<()> {
/// # let path = "./data/osrs_cache";
/// # let cache: Cache<MemoryStore> = Cache::new(path)?;
/// let buffer = cache.read(2, 10)?;
/// let decoded_buffer = rscache::codec::decode(&buffer)?;
/// # Ok(())
/// # }
/// ```
#[inline]
pub fn decode(buffer: &[u8]) -> crate::Result<Vec<u8>> {
	let (buffer, compression) = be_u8(buffer)?;
	let compression = Compression::try_from(compression)?;

	let (buffer, len) = be_u32(buffer)?;
	let (_revision, buffer) = match compression {
		Compression::None => decompress_none(buffer, len as usize)?,
		Compression::Bzip2 => decompress_bzip2(buffer, len as usize)?,
		Compression::Gzip => decompress_gzip(buffer, len as usize)?,
	};

	Ok(buffer)
}

fn compress_bzip2(data: &[u8]) -> io::Result<Vec<u8>> {
	let compressor = Encoder::new(data.to_owned())?;
	compressor.finish().into_result()
}

fn compress_gzip(data: &[u8]) -> io::Result<Vec<u8>> {
	let compressor = BzEncoder::new(data.to_owned(), bzip2::Compression::default());
	let mut compressed_data = compressor.finish()?;
	compressed_data.drain(..4);

	Ok(compressed_data)
}

fn decompress_none(buffer: &[u8], len: usize) -> crate::Result<(Option<i16>, Vec<u8>)> {
	let mut compressed_data = vec![0; len];
	compressed_data.copy_from_slice(buffer);

	let (_, revision) = cond(buffer.len() - len >= 2, be_i16)(buffer)?;

	Ok((revision, compressed_data))
}

fn decompress_bzip2(buffer: &[u8], len: usize) -> crate::Result<(Option<i16>, Vec<u8>)> {
	let (buffer, decompressed_len) = be_u32(buffer)?;
	let mut compressed_data = vec![0; len];
	compressed_data[4..len].copy_from_slice(&buffer[..len - 4]);
	compressed_data[..4].copy_from_slice(b"BZh1");

	let (_, revision) = cond(buffer.len() - len >= 2, be_i16)(buffer)?;

	let mut decompressor = BzDecoder::new(compressed_data.as_slice());
	let mut decompressed_data = vec![0; decompressed_len as usize];
	decompressor.read_exact(&mut decompressed_data)?;

	Ok((revision, decompressed_data))
}

fn decompress_gzip(buffer: &[u8], len: usize) -> crate::Result<(Option<i16>, Vec<u8>)> {
	let (buffer, decompressed_len) = be_u32(buffer)?;
	let mut compressed_data = vec![0; len - 4];
	compressed_data.copy_from_slice(&buffer[..len - 4]);

	let (_, revision) = cond(buffer.len() - len >= 2, be_i16)(buffer)?;

	let mut decoder = Decoder::new(&compressed_data[..])?;
	let mut decompressed_data = vec![0; decompressed_len as usize];
	decoder.read_exact(&mut decompressed_data)?;

	Ok((revision, decompressed_data))
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