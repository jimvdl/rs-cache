use std::{
    io,
    io::BufReader,
};

use serde::{ Serialize, Deserialize };

use crate::{ Definition, ext::ReadExt, util };

/// Contains all the information about a certain item fetched from the cache through
/// the [ItemLoader](../../ldr/rs3/struct.ItemLoader.html).
#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct ItemDefinition {
    pub id: u32,
    pub model_data: ModelData,
    pub name: String,
    pub stackable: bool,
    pub cost: i32,
    pub members_only: bool,
    pub options: [String; 5],
    pub interface_options: [String; 5],
    pub unnoted: bool,
    // might give this an enum
    pub equip_slot: u8,
    pub equip_hide_slot1: u8,
    pub equip_hide_slot2: u8,
    pub noted_id: Option<u16>,
    pub noted_template: Option<u16>,
    pub stack_ids: Option<[u16; 10]>,
    pub stack_count: Option<[u16; 10]>,
    pub team: u8,
    pub lend_id: Option<u16>,
    pub lend_tempalte: Option<u16>,
    pub lent: bool,
    pub bind_link: Option<u16>,
    pub bind_tempalte: Option<u16>,
}

#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct ModelData {
    pub id: u32,
    pub zoom: u16,
    pub rotation1: u16,
    pub rotation2: u16,
    pub offset1: u16,
    pub offset2: u16,
    pub male_equip_id: u32,
    pub female_equip_id: u32,
    pub male_equip1: u32,
    pub male_equip2: u32,
    pub female_equip1: u32,
    pub female_equip2: u32,
    pub original_colors: Vec<u16>,
    pub modified_colors: Vec<u16>,
    pub original_texture_colors: Vec<u16>,
    pub modified_texture_colors: Vec<u16>,
}

impl Definition for ItemDefinition {
    #[inline]
    fn new(id: u32, buffer: &[u8]) -> io::Result<Self> {
        let mut reader = BufReader::new(buffer);
        let item_def = decode_buffer(id, &mut reader)?;

        Ok(item_def)
    }
}

fn decode_buffer(id: u32, reader: &mut BufReader<&[u8]>) -> io::Result<ItemDefinition> {
    let mut item_def = ItemDefinition {
        id,
        options: [
            "".to_string(), 
            "".to_string(), 
            "Take".to_string(), 
            "".to_string(), 
            "".to_string()
        ],
        interface_options: [
            "".to_string(), 
            "".to_string(), 
            "".to_string(), 
            "".to_string(), 
            "Drop".to_string()
        ],
        .. ItemDefinition::default()
    };

    loop {
        let opcode = reader.read_u8()?;

        match opcode {
            0 => break,
            1 => { item_def.model_data.id = reader.read_smart()?; },
            2 => { item_def.name = reader.read_string()?; },
            4 => { item_def.model_data.zoom = reader.read_u16()?; },
            5 => { item_def.model_data.rotation1 = reader.read_u16()?; },
            6 => { item_def.model_data.rotation2 = reader.read_u16()?; },
            7 => { item_def.model_data.offset1 = reader.read_u16()?; },
            8 => { item_def.model_data.offset2 = reader.read_u16()?; },
            11 => item_def.stackable = true,
            12 => { item_def.cost = reader.read_i32()?; },
            13 => { item_def.equip_slot = reader.read_u8()?; },
            14 => { item_def.equip_hide_slot1 = reader.read_u8()?; },
            16 => item_def.members_only = true,
            23 => { item_def.model_data.male_equip1 = reader.read_smart()?; },
            24 => { item_def.model_data.male_equip2 = reader.read_smart()?; },
            25 => { item_def.model_data.female_equip1 = reader.read_smart()?; },
            26 => { item_def.model_data.female_equip2 = reader.read_smart()?; },
            27 => { item_def.equip_hide_slot2 = reader.read_u8()?; },
            30..=34 => { item_def.options[opcode as usize - 30] = reader.read_string()?; },
            35..=39 => { item_def.interface_options[opcode as usize - 35] = reader.read_string()?; },
            40 => {
                let len = reader.read_u8()? as usize;
                item_def.model_data.original_colors = Vec::with_capacity(len);
                item_def.model_data.modified_colors = Vec::with_capacity(len);
                for _ in 0..len {
                    item_def.model_data.original_colors.push(reader.read_u16()?);
                    item_def.model_data.modified_colors.push(reader.read_u16()?);
                }
            },
            41 => {
                let len = reader.read_u8()? as usize;
                item_def.model_data.original_texture_colors = Vec::with_capacity(len);
                item_def.model_data.original_texture_colors = Vec::with_capacity(len);
                for _ in 0..len {
                    item_def.model_data.original_texture_colors.push(reader.read_u16()?);
                    item_def.model_data.original_texture_colors.push(reader.read_u16()?);
                }
            },
            42 => { 
                let len = reader.read_u8()?;
                for _ in 0..len {
                    reader.read_u8()?;
                }
            },
            65 => { item_def.unnoted = true; },
            78 => { item_def.model_data.male_equip_id = reader.read_smart()?; },
            79 => { item_def.model_data.female_equip_id = reader.read_smart()?; },
            97 => { item_def.noted_id = Some(reader.read_u16()?); },
            98 => { item_def.noted_template = Some(reader.read_u16()?); item_def.stackable = true; },
            100..=109 => {
                item_def.stack_ids = Some([0; 10]);
                item_def.stack_count = Some([0; 10]);
                
                match item_def.stack_ids {
                    Some(mut stack_ids) => {
                        stack_ids[opcode as usize - 100] = reader.read_u16()?;
                    },
                    _ => unreachable!()
                }
                match item_def.stack_count {
                    Some(mut stack_count) => {
                        stack_count[opcode as usize - 100] = reader.read_u16()?;
                    },
                    _ => unreachable!()
                }
            },
            115 => { item_def.team = reader.read_u8()?; },
            121 => { 
                item_def.lend_id = Some(reader.read_u16()?); 
                item_def.interface_options[4] = "Discard".to_owned();
                item_def.lent = true;
            },
            122 => { item_def.lend_tempalte = Some(reader.read_u16()?); },
            125 | 126 => { 
                reader.read_u8()?;
                reader.read_u8()?;
                reader.read_u8()?;
            },
            127..=130 => {
                reader.read_u8()?;
                reader.read_u16()?;
            },
            132 => {
                let len = reader.read_u8()?;
                for _ in 0..len {
                    reader.read_u16()?;
                }
            },
            139 => { 
                item_def.bind_link = Some(reader.read_u16()?); 
                item_def.interface_options[4] = "Destroy".to_owned();
            },
            140 => { item_def.bind_tempalte = Some(reader.read_u16()?); },
            164 => { reader.read_string()?; },
            251 | 252 => {
                let len = reader.read_u8()?;
                for _ in 0..len {
                    reader.read_u16()?;
                    reader.read_u16()?;
                }
            },
            249 => { util::read_parameters(reader)?; },
            15 | 156 | 157 | 165 | 167 => {},
            96 | 113 | 114 | 134 => { reader.read_u8()?; },
            18 | 44 | 45 | 94 | 95 | 110..=112 | 142..=146 | 150..=154 | 161..=163 => { reader.read_u16()?; },
            90..=93 | 242..=248 => { reader.read_smart()?; },
            _ => { println!("{} {}", id, opcode); unreachable!() }
        }
    }

    Ok(item_def)
}