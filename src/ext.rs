//! Extension traits.

use std::io::{ self, Read };

/// Adds easy byte reading onto a [`Read`] instance.
/// 
/// [`Read`]: https://doc.rust-lang.org/std/io/trait.Read.html
pub trait ReadExt: Read {
    fn read_u8(&mut self) -> io::Result<u8>;
    fn read_i8(&mut self) -> io::Result<i8>;
    fn read_u16(&mut self) -> io::Result<u16>;
    fn read_i16(&mut self) -> io::Result<i16>;
    fn read_smart_u16(&mut self) -> io::Result<u16>;
    fn read_u24(&mut self) -> io::Result<u32>;
    fn read_i24(&mut self) -> io::Result<i32>;
    fn read_u32(&mut self) -> io::Result<u32>;
    fn read_i32(&mut self) -> io::Result<i32>;
    fn read_u64(&mut self) -> io::Result<u64>;
    fn read_i64(&mut self) -> io::Result<i64>;
    fn read_u128(&mut self) -> io::Result<u128>;
    fn read_i128(&mut self) -> io::Result<i128>;
    fn read_smart(&mut self) -> io::Result<u32>;
    fn read_string(&mut self) -> io::Result<String>;
}

impl<T: Read> ReadExt for T {
    #[inline]
    fn read_u8(&mut self) -> io::Result<u8> {
        let mut buffer = [0; 1];
        self.read_exact(&mut buffer)?;

        Ok(u8::from_be_bytes(buffer))
    }
    
    #[inline]
    fn read_i8(&mut self) -> io::Result<i8> {
        Ok(self.read_u8()? as i8)
    }
    
    #[inline]
    fn read_u16(&mut self) -> io::Result<u16> {
        let mut buffer = [0; 2];
        self.read_exact(&mut buffer)?;

        Ok(u16::from_be_bytes(buffer))
    }

    #[inline]
    fn read_i16(&mut self) -> io::Result<i16> {
        Ok(self.read_u16()? as i16)
    }

    #[inline]
    fn read_smart_u16(&mut self) -> io::Result<u16> {
        let byte = self.read_u8()?;

        if byte < 128 {
           Ok(byte.wrapping_sub(64) as u16)
        } else {
            let value = self.read_u8()?;
            let mut arr = [0; 2];
            arr[0] = byte as u8;
            arr[1] = value;

            let value = u16::from_be_bytes(arr);
            Ok(value - 0xC000)
        }
    }
    
    #[inline]
    fn read_u24(&mut self) -> io::Result<u32> {
        let mut buffer = [0; 3];
        self.read_exact(&mut buffer)?;

        Ok(((buffer[0] as u32) << 16) | ((buffer[1] as u32) << 8) | (buffer[2] as u32))
    }

    #[inline]
    fn read_i24(&mut self) -> io::Result<i32> {
        Ok(self.read_u24()? as i32)
    }

    #[inline]
    fn read_u32(&mut self) -> io::Result<u32> {
        let mut buffer = [0; 4];
        self.read_exact(&mut buffer)?;

        Ok(u32::from_be_bytes(buffer))
    }
    
    #[inline]
    fn read_i32(&mut self) -> io::Result<i32> {
        Ok(self.read_u32()? as i32)
    }

    #[inline]
    fn read_u64(&mut self) -> io::Result<u64> {
        let mut buffer = [0; 8];
        self.read_exact(&mut buffer)?;

        Ok(u64::from_be_bytes(buffer))
    }

    #[inline]
    fn read_i64(&mut self) -> io::Result<i64> {
        Ok(self.read_u64()? as i64)
    }

    #[inline]
    fn read_u128(&mut self) -> io::Result<u128> {
        let mut buffer = [0; 16];
        self.read_exact(&mut buffer)?;

        Ok(u128::from_be_bytes(buffer))
    }

    #[inline]
    fn read_i128(&mut self) -> io::Result<i128> {
        Ok(self.read_u128()? as i128)
    }

    // clean this up.
    // can't find a way to peek the first byte, even 
    // an iterator reads the first byte... 
    #[inline]
    fn read_smart(&mut self) -> io::Result<u32> {
        let byte = self.read_u8()?;

        if (byte as i64 ^ 0xffffffff) as i8 <= -1 {
            let value = self.read_u8()?;
            let mut arr = [0; 2];
            arr[0] = byte;
            arr[1] = value;

            return Ok(u16::from_be_bytes(arr) as u32)
        }

        let mut buffer = [0; 3];
        self.read_exact(&mut buffer)?;
        let mut arr = [0; 4];
        arr[0] = byte;
        arr[1] = buffer[0];
        arr[2] = buffer[1];
        arr[3] = buffer[2];

        Ok(u32::from_be_bytes(arr) & 0x7fffffff)
    }

    #[inline]
    fn read_string(&mut self) -> io::Result<String> {
        let mut bytes = Vec::new();
    
        loop {
            let byte = self.read_u8()?;
            if byte != 0 {
                bytes.push(byte);
            } else {
                break;
            }
        }
    
        Ok(String::from_utf8_lossy(&bytes[..]).to_string())
    }
}