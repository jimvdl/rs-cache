use std::io::{ Read, BufReader };

pub trait ReadExt: Read {
    fn read_u8(&mut self) -> u8;
    fn read_i8(&mut self) -> i8;
    fn read_u16(&mut self) -> u16;
    fn read_u24(&mut self) -> u32;
    fn read_i32(&mut self) -> i32;
    fn read_string(&mut self) -> String;
}

impl ReadExt for BufReader<&[u8]> {
    fn read_u8(&mut self) -> u8 {
        let mut buffer = [0; 1];
        self.read_exact(&mut buffer).unwrap();
        u8::from_be_bytes(buffer)
    }
    
    fn read_i8(&mut self) -> i8 {
        let mut buffer = [0; 1];
        self.read_exact(&mut buffer).unwrap();
        i8::from_be_bytes(buffer)
    }
    
    fn read_u16(&mut self) -> u16 {
        let mut buffer = [0; 2];
        self.read_exact(&mut buffer).unwrap();
        u16::from_be_bytes(buffer)
    }
    
    fn read_u24(&mut self) -> u32 {
        let mut buffer = [0; 3];
        self.read_exact(&mut buffer).unwrap();
        ((buffer[0] as u32) << 16) | ((buffer[1] as u32) << 8) | (buffer[2] as u32)
    }
    
    fn read_i32(&mut self) -> i32 {
        let mut buffer = [0; 4];
        self.read_exact(&mut buffer).unwrap();
        i32::from_be_bytes(buffer)
    }
    
    fn read_string(&mut self) -> String {
        let mut bytes = Vec::new();
    
        loop {
            let mut buffer = [0; 1];
            self.read_exact(&mut buffer).unwrap();
            let byte = u8::from_be_bytes(buffer);
            if byte != 0 {
                bytes.push(byte);
            } else {
                break;
            }
        }
    
        String::from_utf8_lossy(&bytes[..]).to_string()
    }
}