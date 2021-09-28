//! Faster parsers using nom.

use nom::{ 
    IResult, 
    error::ParseError,
    bytes::complete::{
        tag,
		take_while,
	},
	sequence::terminated,
};

/// Reads a 0-terminated string from the given buffer. Uses `String::from_utf8_lossy()` for the conversion.
/// 
/// # Errors
/// 
/// Parser can reach EOF early if not enough bytes are supplied or no 0-termination character is present.
/// 
/// # Example
/// 
/// ```
/// use rscache::parse::rs_string;
/// 
/// # fn main() -> rscache::Result<()> {
/// let buffer = &[82, 117, 110, 105, 116, 101, 32, 98, 97, 114, 0, 52, 14, 85, 65, 4, 56];
/// 
/// let (buffer, string) = rs_string(buffer)?;
/// 
/// assert_eq!(&string, "Runite bar");
/// assert_eq!(buffer, &[52, 14, 85, 65, 4, 56]);
/// # Ok(())
/// # }
/// ```
#[inline]
pub fn rs_string<'a, E: ParseError<&'a[u8]>>(buffer: &'a [u8]) -> IResult<&'a[u8], String, E> {
    let (buffer, string) = terminated(take_while(|byte| byte != 0), tag([0]))(buffer)?;

    Ok((buffer, String::from_utf8_lossy(string).to_string()))
}

/// Contains parsers specific to the RS3 protocol.
pub mod rs3 {
    use nom::{ 
        IResult, 
        error::ParseError,
        number::complete::{ be_u8, be_u16, be_u32 },
    };

    /// Reads 1 byte if the first byte < 128, reads 2 bytes otherwise.
    ///
    /// # Errors
    /// 
    /// Parser can reach EOF early if not enough bytes are supplied.
    /// 
    /// # Example
    /// 
    /// ```
    /// use rscache::parse::rs3::be_u16_smart;
    /// 
    /// # fn main() -> rscache::Result<()> {
    /// let buffer = &[17, 142, 64, 4, 24, 254];
    /// 
    /// let (buffer, value1) = be_u16_smart(buffer)?;
    /// let (buffer, value2) = be_u16_smart(buffer)?;
    /// 
    /// assert_eq!(value1, 209);
    /// assert_eq!(value2, 52800);
    /// assert_eq!(buffer, &[4, 24, 254]);
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn be_u16_smart<'a, E: ParseError<&'a[u8]>>(buffer: &'a [u8]) -> IResult<&'a[u8], u16, E> {
        if buffer[0] < 128 {
            let (buffer, value) = be_u8(buffer)?;
            Ok((buffer, value.wrapping_sub(64) as u16))
        } else {
            let (buffer, value) = be_u16(buffer)?;
            Ok((buffer, value.wrapping_sub(0xC000)))
        }
    }

    /// Reads 2 bytes if the first byte <= -1 after calculations, reads 4 bytes otherwise.
    /// 
    /// # Errors
    /// 
    /// Parser can reach EOF early if not enough bytes are supplied.
    /// 
    /// # Example
    /// 
    /// ```
    /// use rscache::parse::rs3::be_u32_smart;
    /// 
    /// # fn main() -> rscache::Result<()> {
    /// let buffer = &[255, 54, 2, 0, 62, 1, 42, 233];
    /// 
    /// let (buffer, value1) = be_u32_smart(buffer)?;
    /// let (buffer, value2) = be_u32_smart(buffer)?;
    /// 
    /// assert_eq!(value1, 2134245888);
    /// assert_eq!(value2, 15873);
    /// assert_eq!(buffer, &[42, 233]);
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn be_u32_smart<'a, E: ParseError<&'a[u8]>>(buffer: &'a [u8]) -> IResult<&'a[u8], u32, E> {
        if (buffer[0] as i64 ^ 0xffffffff) as i8 <= -1 {
            let (buffer, value) = be_u16(buffer)?;
            Ok((buffer, value as u32))
        } else {
            let (buffer, value) = be_u32(buffer)?;
            Ok((buffer, value & 0x7fffffff))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::rs3::*;

    #[test]
    fn rs_string_parser() -> crate::Result<()> {
        let buffer = vec![82, 117, 110, 105, 116, 101, 32, 98, 97, 114, 0, 52, 14, 85, 65, 4, 56];

        let (buffer, string) = rs_string(&buffer)?;

        assert_eq!(&string, "Runite bar");
        assert_eq!(&buffer, &[52, 14, 85, 65, 4, 56]);

        Ok(())
    }

    #[test]
    fn be_u16_smart_parser() -> crate::Result<()> {
        let buffer = &[17, 142, 64, 4, 24, 254];
     
        let (buffer, value1) = be_u16_smart(buffer)?;
        let (buffer, value2) = be_u16_smart(buffer)?;
        
        assert_eq!(value1, 209);
        assert_eq!(value2, 52800);
        assert_eq!(buffer, &[4, 24, 254]);

        Ok(())
    }

    #[test]
    fn be_u32_smart_parser() -> crate::Result<()> {
        let buffer = &[255, 54, 2, 0, 62, 1, 42, 233];
    
        let (buffer, value1) = be_u32_smart(buffer)?;
        let (buffer, value2) = be_u32_smart(buffer)?;
    
        assert_eq!(value1, 2134245888);
        assert_eq!(value2, 15873);
        assert_eq!(buffer, &[42, 233]);

        Ok(())
    }
}