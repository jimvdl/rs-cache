use std::io;
use std::io::BufReader;
use crate::Definition;
use crate::ext::ReadExt;
use serde::{ Serialize, Deserialize };


#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Debug, Default)]
pub struct InventoryDefinition {
    pub id: u32,
    pub capacity: Option<u16>,
}

impl Definition for InventoryDefinition {
    #[inline]
    fn new(id: u32, buffer: &[u8]) -> io::Result<Self> {
        let mut reader = BufReader::new(buffer);
        let item_def = decode_buffer(id, &mut reader)?;

        Ok(item_def)
    }
}

fn decode_buffer(id: u32, reader: &mut BufReader<&[u8]>) -> io::Result<InventoryDefinition> {
    let mut inv_def = InventoryDefinition {
        id,
        capacity: None,
        .. InventoryDefinition::default()
    };

    loop {
        let opcode = reader.read_u8()?;
        match opcode {
            0 => break,
            2 => inv_def.capacity = reader.read_u16().ok(),
            _ => {}
        }
    }

    Ok(inv_def)
}