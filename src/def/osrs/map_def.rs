use std::{
    io,
    io::BufReader,
};

use serde::{ Serialize, Deserialize };

use super::Definition;
use crate::{ 
    ext::ReadExt,
    // Cache,
    codec,
};

const X: usize = 64;
const Y: usize = 64;
const Z: usize = 4;

#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct MapDefinition {
    pub region_x: u32,
    pub region_y: u32,
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
    fn new(id: u16, buffer: &[u8]) -> io::Result<Self> {
        let x = id as u32 >> 8;
        let y = id as u32 & 0xFF;

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
    
    // #[inline]
    // pub(crate) fn load_internal(cache: &Cache, id: u32) -> crate::Result<Self> {
    //     let x = id >> 8;
    //     let y = id & 0xFF;

    //     let map_archive = cache.archive_by_name(5, format!("m{}_{}", x, y))?;
    //     println!("reading archive: {}", map_archive.id);
    //     let buffer = cache.read_archive(map_archive)?;
    //     let buffer = codec::decode(&buffer)?;
        
    //     Ok(Self::new(id, &buffer)?)
    // }

    #[inline]
    pub fn blocked_tiles(&self) -> Vec<(u32, u32, u32)> {
        let region_base_x = self.region_x << 6;
        let region_base_y = self.region_y << 6;
        let mut blocked_tiles = Vec::new();

        for z in 0..Z {
            for x in 0..X {
                for y in 0..Y {
                    let map_data = &self.data[z][x][y];

                    if map_data.settings & 1 == 1 {
                        blocked_tiles.push((
                            region_base_x + x as u32, 
                            region_base_y + y as u32, 
                            z as u32)
                        );
                    }
                }
            }
        }

        blocked_tiles
    }
}

fn decode_buffer(x: u32, y: u32, reader: &mut BufReader<&[u8]>) -> io::Result<MapDefinition> {
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
                        0 => break ,
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