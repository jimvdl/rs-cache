use std::io::{ self, Read };

use bzip2::{
	read::BzDecoder,
	write::BzEncoder,
	Compression,
};
use libflate::gzip::{
	Decoder,
	Encoder,
};

use crate::CacheError;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CompressionType {
	None,
	Bzip2,
	Gzip
}

impl From<u8> for CompressionType {
	fn from(compression: u8) -> Self {
		match compression {
			0 => Self::None,
			1 => Self::Bzip2,
			2 => Self::Gzip,
			_ => unreachable!(),
		}
	}
}

#[derive(Debug)]
pub struct Container {
	pub compression: CompressionType,
	pub data: Vec<u8>,
	pub revision: i16,
}

impl Container {
	#[inline]
	pub fn new(compression: CompressionType, data: Vec<u8>, revision: i16) -> Self {
		Self { compression, data, revision }
	}

	#[inline]
	pub fn decode<R: Read>(reader: &mut R) -> Result<Self, CacheError> {
		let mut buf = [0; 1];
		reader.read_exact(&mut buf)?;
		let compression: CompressionType = buf[0].into();

		let mut buf = [0; 4];
		reader.read_exact(&mut buf)?;
		let len = u32::from_be_bytes(buf) as usize;

		let (revision, buffer) = match compression {
			CompressionType::None => decompress_none(reader, len)?,
			CompressionType::Bzip2 => decompress_bzip2(reader, len)?,
			CompressionType::Gzip => decompress_gzip(reader, len)?,
		};

		Ok(Self::new(compression, buffer, revision))
	}

	#[inline]
	pub fn encode(&self) -> Result<Vec<u8>, CacheError> {
		let compressed_data = match self.compression {
			CompressionType::None => self.data.clone(),
			CompressionType::Bzip2 => compress_bzip2(self.data.clone())?,
			CompressionType::Gzip => compress_gzip(self.data.clone())?,
		};

		let mut buffer = Vec::new();
		buffer.push(self.compression as u8);
		buffer.extend_from_slice(&u32::to_be_bytes(compressed_data.len() as u32));
		
		if self.compression != CompressionType::None {
			buffer.extend_from_slice(&u32::to_be_bytes(self.data.len() as u32));
		}

		buffer.extend(compressed_data);

		if self.revision != -1 {
			buffer.extend_from_slice(&i16::to_be_bytes(self.revision));
		}

		Ok(buffer)
	}

	pub fn data(&self) -> &[u8] {
		&self.data[..]
	}
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

fn compress_bzip2(data: Vec<u8>) -> Result<Vec<u8>, io::Error> {
	let compressor = Encoder::new(data)?;
	compressor.finish().into_result()
}

fn compress_gzip(data: Vec<u8>) -> Result<Vec<u8>, io::Error> {
	let compressor = BzEncoder::new(data, Compression::Default);
	let mut compressed_data = compressor.finish()?;
	compressed_data.drain(0..4);

	Ok(compressed_data)
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