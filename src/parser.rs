use nom::{
    multi::many0,
    bytes::complete::take,
	number::complete::{
		be_u16,
		be_u32,
    },
};

use crate::CacheError;

#[inline]
pub fn parse_n_u16(buffer: &[u8], count: usize) -> Result<(&[u8], Vec<u16>), CacheError> {
    let (buffer, taken) = take(count * 2)(buffer)?;
    let (_, values) = many0(be_u16)(taken)?;

    Ok((buffer, values))
}

#[inline]
pub fn parse_n_u32(buffer: &[u8], count: usize) -> Result<(&[u8], Vec<u32>), CacheError> {
    let (buffer, taken) = take(count * 4)(buffer)?;
    let (_, values) = many0(be_u32)(taken)?;

    Ok((buffer, values))
}

#[inline]
pub fn parse_usize_from_u16(buffer: &[u8]) -> Result<(&[u8], usize), CacheError> {
    let (buffer, value) = be_u16(buffer)?;

    Ok((buffer, value as usize))
}