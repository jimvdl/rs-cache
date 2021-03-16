use std::{
	io,
	io::BufReader,
	collections::HashMap,
};

use crate::{ Definition, ext::ReadExt, util };

/// Contains all the information about a certain item fetched from the cache through
/// the [ItemLoader](struct.ItemLoader.html).
/// 
/// The `InventoryModelData` and the `CharacterModelData` were hidden in the documents
/// because these are rarely accessed, they contain useless information in most use-cases. 
#[derive(Clone, Eq, PartialEq, Debug, Default)]
pub struct ItemDefinition {
	pub id: u16,
	pub inventory_model_data: InventoryModelData,
	pub character_model_data: CharacterModelData,
	pub name: String,
	pub stackable: bool,
	pub cost: i32,
	pub members_only: bool,
	pub options: [String; 5],
	pub interface_options: [String; 5],
	pub tradable: bool,
	pub noted_id: Option<u16>,
	pub noted_template: Option<u16>,
	pub count_obj: Option<[i32; 10]>,
	pub count_co: [u16; 10],
	pub team: u8,
	pub bought_link: Option<u16>,
	pub bought_tempalte: Option<u16>,
	pub shift_click_drop_index: Option<u8>,
	pub params: HashMap<u32, String>,
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct InventoryModelData {
	pub inventory_model: u16,
	pub zoom2d: u16,
	pub x_an2d: u16,
	pub y_an2d: u16,
	pub z_an2d: u16,
	pub x_offset2d: u16,
	pub y_offset2d: u16,
	pub resize_x: u16,
	pub resize_y: u16,
	pub resize_z: u16,
	pub color_find: Vec<u16>,
	pub color_replace: Vec<u16>,
	pub texture_find: Vec<u16>,
	pub texture_replace: Vec<u16>,
	pub ambient: i8,
	pub contrast: i8,
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct CharacterModelData {
	pub male_model10: Option<u16>,
	pub male_model_offset: u8,
	pub male_model1: Option<u16>,
	pub female_model10: Option<u16>,
	pub female_model_offset: u8,
	pub female_model1: Option<u16>,
	pub male_model12: Option<u16>,
	pub female_model12: Option<u16>,
	pub male_head_model1: Option<u16>,
	pub female_head_model1: Option<u16>,
	pub male_head_model2: Option<u16>,
	pub female_head_model2: Option<u16>,
}

impl Definition for ItemDefinition {
	#[inline]
    fn new(id: u16, buffer: &[u8]) -> io::Result<Self> {
        let mut reader = BufReader::new(&buffer[..]);
		let item_def = decode_buffer(id, &mut reader)?;

		Ok(item_def)
    }
}

fn decode_buffer(id: u16, reader: &mut BufReader<&[u8]>) -> io::Result<ItemDefinition> {
	let mut item_def = ItemDefinition {
		id,
		inventory_model_data: InventoryModelData {
			resize_x: 128,
			resize_y: 128,
			resize_z: 128,
			zoom2d: 2000,
			.. InventoryModelData::default()
		},
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
			1 => { item_def.inventory_model_data.inventory_model = reader.read_u16()?; },
			2 => { item_def.name = reader.read_string()?; },
			4 => { item_def.inventory_model_data.zoom2d = reader.read_u16()?; },
			5 => { item_def.inventory_model_data.x_an2d = reader.read_u16()?; },
			6 => { item_def.inventory_model_data.y_an2d = reader.read_u16()?; },
			7 => { item_def.inventory_model_data.x_offset2d = reader.read_u16()?; },
			8 => { item_def.inventory_model_data.y_offset2d = reader.read_u16()?; },
			11 => { item_def.stackable = true; },
			12 => { item_def.cost = reader.read_i32()?; },
			16 => item_def.members_only = true,
			23 => {
				item_def.character_model_data.male_model10 = Some(reader.read_u16()?);
				item_def.character_model_data.male_model_offset = reader.read_u8()?;
			},
			24 => { item_def.character_model_data.male_model1 = Some(reader.read_u16()?); },
			25 => {
				item_def.character_model_data.female_model10 = Some(reader.read_u16()?);
				item_def.character_model_data.female_model_offset = reader.read_u8()?;
			},
			26 => { item_def.character_model_data.female_model1 = Some(reader.read_u16()?); },
			30..=34 => { item_def.options[opcode as usize - 30] = reader.read_string()?; },
			35..=39 => { item_def.interface_options[opcode as usize - 35] = reader.read_string()?; },
			40 => {
				let len = reader.read_u8()?;
				for _ in 0..len {
					item_def.inventory_model_data.color_find.push(reader.read_u16()?);
					item_def.inventory_model_data.color_replace.push(reader.read_u16()?);
				}
			},
			41 => {
				let len = reader.read_u8()?;
				for _ in 0..len {
					item_def.inventory_model_data.texture_find.push(reader.read_u16()?);
					item_def.inventory_model_data.texture_replace.push(reader.read_u16()?);
				}
			},
			42 => { item_def.shift_click_drop_index = Some(reader.read_u8()?); },
			65 => { item_def.tradable = true; },
			78 => { item_def.character_model_data.male_model12 = Some(reader.read_u16()?); },
			79 => { item_def.character_model_data.female_model12 = Some(reader.read_u16()?); },
			90 => { item_def.character_model_data.male_head_model1 = Some(reader.read_u16()?); },
			91 => { item_def.character_model_data.female_head_model1 = Some(reader.read_u16()?); },
			92 => { item_def.character_model_data.male_head_model2 = Some(reader.read_u16()?); },
			93 => { item_def.character_model_data.female_head_model2 = Some(reader.read_u16()?); },
			95 => { item_def.inventory_model_data.z_an2d = reader.read_u16()?; },
			97 => { item_def.noted_id = Some(reader.read_u16()?); },
			98 => { item_def.noted_template = Some(reader.read_u16()?); item_def.stackable = true; },
			100..=109 => {
				if item_def.count_obj.is_none() {
					item_def.count_obj = Some([0; 10]);
					item_def.count_co = [0; 10];
				}
				reader.read_u16()?;
				//count_obj[opcode as usize - 100] = c_obj;
				item_def.count_co[opcode as usize - 100] = reader.read_u16()?;
			},
			110 => { item_def.inventory_model_data.resize_x = reader.read_u16()?; },
			111 => { item_def.inventory_model_data.resize_y = reader.read_u16()?; },
			112 => { item_def.inventory_model_data.resize_z = reader.read_u16()?; },
			113 => { item_def.inventory_model_data.ambient = reader.read_i8()?; },
			114 => { item_def.inventory_model_data.contrast = reader.read_i8()?; },
			115 => { item_def.team = reader.read_u8()?; },
			139 => { item_def.bought_link = Some(reader.read_u16()?); },
			140 => { item_def.bought_tempalte = Some(reader.read_u16()?); },
			148 | 149 => { reader.read_u16()?; },
			249 => { item_def.params = util::read_parameters(reader)?; },
			_ => unreachable!()
		}
	}

	Ok(item_def)
}