use std::{
	io,
	io::BufReader,
	collections::HashMap,
};

use crate::{ Definition, ext::ReadExt, util };

/// Contains all the information about a certain object fetched from the cache through
/// the [ObjectLoader](struct.ObjectLoader.html).
/// 
/// The `ObjectModelData` is hidden in the documents
/// because it is rarely accessed, it contains useless information in most use-cases. 
#[derive(Clone, Eq, PartialEq, Debug, Default)]
pub struct ObjectDefinition {
    pub id: u32,
    pub model_data: ObjectModelData,
    pub name: String,
    pub config_id: Option<u16>,
    pub map_area_id: Option<u16>,
    pub map_scene_id: u16,
    pub animation_id: u16,
    pub solid: bool,
    pub shadow: bool,
    pub obstruct_ground: bool,
    pub supports_items: Option<u8>,
    pub actions: [String; 5],
    pub interact_type: u8,
    pub rotated: bool,
    pub ambient_sound_id: u16,
    pub blocks_projectile: bool,
    pub wall_or_door: Option<u8>,
    pub contoured_ground: Option<u8>,
    pub config_change_dest: Vec<u16>,
    pub params: HashMap<u32, String>,
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct ObjectModelData {
    pub models: Vec<u16>,
    pub types: Vec<u8>,
    pub recolor_find: Vec<u16>,
    pub recolor_replace: Vec<u16>,
    pub retexture_find: Vec<u16>,
    pub retexture_replace: Vec<u16>,
    pub size_x: u8,
    pub size_y: u8,
    pub offset_x: u16,
    pub offset_y: u16,
    pub offset_z: u16,
    pub model_size_x: u16,
    pub model_size_y: u16,
    pub model_size_z: u16,
    pub varp_id: Option<u16>,
    pub ambient: u8,
    pub contrast: u8,
    pub decord_displacement: u8,
    pub merge_normals: bool,
    pub blocking_mask: u8,
}

impl Definition for ObjectDefinition {
    #[inline]
    fn new(id: u32, buffer: &[u8]) -> io::Result<Self> {
        let mut reader = BufReader::new(buffer);
        let mut obj_def = decode_buffer(id, &mut reader)?;
        post(&mut obj_def);

		Ok(obj_def)
    }
}

fn decode_buffer(id: u32, reader: &mut BufReader<&[u8]>) -> io::Result<ObjectDefinition> {
    let mut obj_def = ObjectDefinition {
        id,
        interact_type: 2,
        blocks_projectile: true,
        solid: true,
        model_data: ObjectModelData {
            decord_displacement: 16,
            size_x: 1,
            size_y: 1,
            model_size_x: 128,
            model_size_y: 128,
            model_size_z: 128,
            .. ObjectModelData::default()
        },
        .. ObjectDefinition::default()
    };

	loop {
		let opcode = reader.read_u8()?;

		match opcode {
			0 => break,
			1 => { 
                let len = reader.read_u8()?;
				for _ in 0..len {
					obj_def.model_data.models.push(reader.read_u16()?);
					obj_def.model_data.types.push(reader.read_u8()?);
				}
            },
			2 => { obj_def.name = reader.read_string()?; },
			5 => {
                let len = reader.read_u8()?;
                obj_def.model_data.types.clear();
				for _ in 0..len {
					obj_def.model_data.models.push(reader.read_u16()?);
				}
            },
			14 => { obj_def.model_data.size_x = reader.read_u8()?; },
            15 => { obj_def.model_data.size_y = reader.read_u8()?; },
            17 => { obj_def.interact_type = 0; obj_def.blocks_projectile = false; },
            18 => { obj_def.blocks_projectile = false; },
            19 => { obj_def.wall_or_door = Some(reader.read_u8()?); },
            21 => { obj_def.contoured_ground = Some(0); },
			22 => { obj_def.model_data.merge_normals = true; },
            24 => { obj_def.animation_id = reader.read_u16()?; },
            27 => { obj_def.interact_type = 1; },
            28 => { obj_def.model_data.decord_displacement = reader.read_u8()?; }
            29 => { obj_def.model_data.ambient = reader.read_u8()?; },
            30..=34 => { obj_def.actions[opcode as usize - 30] = reader.read_string()?; },
            39 => { obj_def.model_data.contrast = reader.read_u8()?; },
            40 => {
                let len = reader.read_u8()?;
				for _ in 0..len {
                    obj_def.model_data.recolor_find.push(reader.read_u16()?);
					obj_def.model_data.recolor_replace.push(reader.read_u16()?);
				}
			},
			41 => {
                let len = reader.read_u8()?;
				for _ in 0..len {
                    obj_def.model_data.retexture_find.push(reader.read_u16()?);
					obj_def.model_data.retexture_replace.push(reader.read_u16()?);
				}
            },
            62 => { obj_def.rotated = true; },
            64 => { obj_def.shadow = true; },
            65 => { obj_def.model_data.model_size_x = reader.read_u16()?; },
            66 => { obj_def.model_data.model_size_z = reader.read_u16()?; },
            67 => { obj_def.model_data.model_size_y = reader.read_u16()?; },
            68 => { obj_def.map_scene_id = reader.read_u16()?; },
            69 => { obj_def.model_data.blocking_mask = reader.read_u8()?; }
            70 => { obj_def.model_data.offset_x = reader.read_u16()?; },
            71 => { obj_def.model_data.offset_z = reader.read_u16()?; },
            72 => { obj_def.model_data.offset_y = reader.read_u16()?; },
            73 => { obj_def.obstruct_ground = true; },
            74 => { obj_def.solid = false; },
            75 => { obj_def.supports_items = Some(reader.read_u8()?); },
            77 => {
                let varp_id = reader.read_u16()?;
                obj_def.model_data.varp_id = if varp_id == std::u16::MAX { None } else { Some(varp_id) };
                
                let config_id = reader.read_u16()?;
                obj_def.config_id = if config_id == std::u16::MAX { None } else { Some(config_id) };
                
                let len = reader.read_u8()?;
                obj_def.config_change_dest = Vec::new();
				for _ in 0..=len {
                    obj_def.config_change_dest.push(reader.read_u16()?);
				}
            },
            78 => { 
                obj_def.ambient_sound_id = reader.read_u16()?; 
                reader.read_u8()?; 
            },
            79 => {
                reader.read_u16()?;
                reader.read_u16()?;
                reader.read_u8()?;
                
                let len = reader.read_u8()?;
				for _ in 0..len {
                    reader.read_u16()?;
				}
            },
            81 => { obj_def.contoured_ground = Some(reader.read_u8()?); },
            82 => { obj_def.map_area_id = Some(reader.read_u16()?); },
            92 => {
                let varp_id = reader.read_u16()?;
                obj_def.model_data.varp_id = if varp_id == std::u16::MAX { None } else { Some(varp_id) };

                let config_id = reader.read_u16()?;
                obj_def.config_id = if config_id == std::u16::MAX { None } else { Some(config_id) };
                
                // should append var at end
                let _var = reader.read_u16()?;

                let len = reader.read_u8()?;
                obj_def.config_change_dest = Vec::new();
				for _ in 0..=len {
                    obj_def.config_change_dest.push(reader.read_u16()?);
				}
            },
            249 => { obj_def.params = util::read_parameters(reader)?; },
            23 => { /* skip */ },
			_ => unreachable!()
		}
	}

	Ok(obj_def)
}

fn post(obj_def: &mut ObjectDefinition) {
    if obj_def.wall_or_door.is_none() {
        obj_def.wall_or_door = Some(0);
        if !obj_def.model_data.models.is_empty() && (obj_def.model_data.types.is_empty() || obj_def.model_data.types[0] == 10) {
            obj_def.wall_or_door = Some(1);
        }

        for var in 0..5 {
            if !obj_def.actions[var].is_empty() {
                obj_def.wall_or_door = Some(1);
            }
        }
    }

    if obj_def.supports_items.is_none() {
        obj_def.supports_items = Some(if obj_def.interact_type != 0 { 1 } else { 0 });
    }
}