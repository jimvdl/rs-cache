use std::{
    io,
    io::BufReader,
    collections::HashMap,
};

use serde::{ Serialize, Deserialize };
use nom::number::complete::be_u8;

use super::Definition;
use crate::parse::{
    be_u16_smart,
    be_u32_smart_compat,
};

#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct LocationDefinition {
    pub id: u16,
    pub region_x: u16,
    pub region_y: u16,
    pub data: Vec<Location>,
}

#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Location {
    pub id: u32,
    pub loc_type: u8,
    pub orientation: u8,
    pub pos: (u32, u32, u32),
}

impl Definition for LocationDefinition {
    #[inline]
    fn new(id: u16, buffer: &[u8]) -> crate::Result<Self> {
        let loc_def = decode_buffer(id, buffer)?;

        Ok(loc_def)
    }
}

fn decode_buffer(id: u16, mut buffer: &[u8]) -> crate::Result<LocationDefinition> {
    let mut loc_def = LocationDefinition {
        id,
        .. LocationDefinition::default()
    };

    let mut id = -1;

    loop {
        let (buf, id_offset) = be_u32_smart_compat(buffer)?;
        buffer = buf;

        if id_offset == 0 || buffer.is_empty() {
            break;
        }

        id += id_offset as i32;

        let mut pos = 0;

        loop {
            let (buf, pos_offset) = be_u16_smart(buffer)?;
            let pos_offset = pos_offset as u32;
            buffer = buf;

            if pos_offset == 0 || buffer.is_empty() {
                break;
            }

            pos += pos_offset - 1;

            let local_x = pos & 0x3F;
            let local_y = pos >> 6 & 0x3F;
            let local_z = pos >> 12 & 0x3;

            let (buf, attr) = be_u8(buffer)?;
            buffer = buf;

            loc_def.data.push(Location { 
                id: id as u32, 
                loc_type: attr >> 2,
                orientation: attr & 0x3,
                pos: (local_x, local_y, local_z)
            });

            if buffer.is_empty() {
                return Ok(loc_def);
            }
        }
    }

    Ok(loc_def)
}