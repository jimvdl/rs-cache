use std::{
    io,
    io::BufReader,
};

use serde::{ Serialize, Deserialize };

use super::Definition;
use crate::extension::ReadExt;

const X: usize = 64;
const Y: usize = 64;
const Z: usize = 4;

#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct MapDefinition {
    pub region_x: u16,
    pub region_y: u16,
    pub data: Vec<Vec<Vec<MapData>>>,
}

#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct MapData {
    pub height: u8,
    pub attr_opcode: u8,
    pub settings: u8,
    pub overlay_id: i8,
    pub overlay_path: u8,
    pub overlay_rotation: u8,
    pub underlay_id: u8,
}

impl Definition for MapDefinition {
    #[inline]
    fn new(id: u16, buffer: &[u8]) -> crate::Result<Self> {
        let x = id >> 8;
        let y = id & 0xFF;

        let mut reader = BufReader::new(buffer);
        let map_def = decode_buffer(x, y, &mut reader)?;

        Ok(map_def)
    }
}

impl MapDefinition {
    #[inline]
    pub fn map_data(&self, x: usize, y: usize, z: usize) -> &MapData {
        &self.data[z][x][y]
    }

    #[inline]
    pub const fn region_base_coords(&self) -> (u16, u16) {
        (self.region_x << 6, self.region_y << 6)
    }

    #[inline]
    pub fn blocked_tiles(&self) -> Vec<(u16, u16, u16)> {
        let region_base_x = self.region_x << 6;
        let region_base_y = self.region_y << 6;
        let mut blocked_tiles = Vec::new();

        for z in 0..Z {
            for x in 0..X {
                for y in 0..Y {
                    let map_data = &self.data[z][x][y];

                    if map_data.settings & 1 == 1 {
                        blocked_tiles.push((
                            region_base_x + x as u16, 
                            region_base_y + y as u16, 
                            z as u16)
                        );
                    }
                }
            }
        }

        blocked_tiles
    }
}

fn decode_buffer(x: u16, y: u16, reader: &mut BufReader<&[u8]>) -> io::Result<MapDefinition> {
    let mut map_def = MapDefinition {
        region_x: x,
        region_y: y,
        data: vec![vec![vec![MapData::default(); X]; Y]; Z],
    };

    for z in 0..Z {
        for x in 0..X {
            for y in 0..Y {
                let map_data = &mut map_def.data[z][x][y];

                loop {
                    let attribute = reader.read_u8()?;

                    match attribute {
                        0 => break,
                        1 => {
                            map_data.height = reader.read_u8()?; break
                        },
                        2..=49 => {
                            map_data.attr_opcode = attribute;
                            map_data.overlay_id = reader.read_i8()?;
                            map_data.overlay_path = (attribute - 2) / 4;
                            map_data.overlay_rotation = (attribute - 2) & 3;
                        },
                        50..=81 => {
                            map_data.settings = attribute - 49;
                        },
                        _ => map_data.underlay_id = attribute - 81,
                    }
                }
            }
        }
    }

    Ok(map_def)
}