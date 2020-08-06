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
	Gzip
}

#[inline]
pub fn encode(compression: Compression, data: &[u8], revision: Option<i16>) -> crate::Result<Vec<u8>> {
	let compressed_data = match compression {
		Compression::None => data.to_owned(),
		Compression::Bzip2 => compress_bzip2(data)?,
		Compression::Gzip => compress_gzip(data)?,
	};

	let mut buffer = Vec::with_capacity(compressed_data.len() + 11);
	buffer.push(compression as u8);
	buffer.extend_from_slice(&u32::to_be_bytes(compressed_data.len() as u32));
	
	if compression != Compression::None {
		buffer.extend_from_slice(&u32::to_be_bytes(data.len() as u32));
	}

	buffer.extend(compressed_data);

	if let Some(revision) = revision {
		buffer.extend_from_slice(&i16::to_be_bytes(revision));
	}

	Ok(buffer)
}

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
	let compressor = BzEncoder::new(data.to_owned(), bzip2::Compression::Default);
	let mut compressed_data = compressor.finish()?;
	compressed_data.drain(0..4);

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
	let mut compressed_data = vec![0; len - 4];
	compressed_data.copy_from_slice(&buffer[..len - 4]);

	let (_, revision) = cond(buffer.len() - len >= 2, be_i16)(buffer)?;

	compressed_data.insert(0, b'1');
	compressed_data.insert(0, b'h');
	compressed_data.insert(0, b'Z');
	compressed_data.insert(0, b'B');
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