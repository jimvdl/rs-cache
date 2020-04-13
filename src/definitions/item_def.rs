use std::{
	io::{
		Read,
		BufReader,
	},
	collections::HashMap,
};

use utils::ReadExt;

pub struct ItemDefinition {
	pub id: u16,
	pub inventory_model: u16,
	pub name: String,
	pub zoom2d: u16,
	pub x_an2d: u16,
	pub y_an2d: u16,
	pub x_offset2d: u16,
	pub y_offset2d: u16,
	pub stackable: bool,
	pub cost: i32,
	pub members_only: bool,
	pub male_model10: u16,
	pub male_model_offset: u8,
	pub male_model1: u16,
	pub female_model10: u16,
	pub female_model_offset: u8,
	pub female_model1: u16,
	pub options: [String; 5],
	pub interface_options: [String; 5],
	pub color_find: Vec<u16>,
	pub color_replace: Vec<u16>,
	pub texture_find: Vec<u16>,
	pub texture_replace: Vec<u16>,
	pub stockmarket: bool,
	pub male_model12: u16,
	pub female_model12: u16,
	pub male_head_model1: u16,
	pub female_head_model1: u16,
	pub male_head_model2: u16,
	pub female_head_model2: u16,
	pub zan2d: u16,
	pub noted_id: u16,
	pub noted_template: u16,
	pub count_obj: Option<[i32; 10]>,
	pub count_co: [u16; 10],
	pub resize_x: u16,
	pub resize_y: u16,
	pub resize_z: u16,
	pub ambient: i8,
	pub contrast: i8,
	pub team: u8,
	pub bought_link: u16,
	pub bought_tempalte: u16,
	pub params: HashMap<u32, String>,
}

impl ItemDefinition {
	#[inline]
	pub fn new(id: u16, buffer: &[u8]) -> Self {
		let mut inventory_model = 0;
		let mut name = String::new();
		let mut zoom2d = 0;
		let mut x_an2d = 0;
		let mut y_an2d = 0;
		let mut x_offset2d = 0;
		let mut y_offset2d = 0;
		let mut stackable = false;
		let mut cost = 0;
		let mut members_only = false;
		let mut male_model10 = 0;
		let mut male_model_offset = 0;
		let mut male_model1 = 0;
		let mut female_model10 = 0;
		let mut female_model_offset = 0;
		let mut female_model1 = 0;
		let mut options: [String; 5] = Default::default();
		let mut interface_options: [String; 5] = Default::default();
		let mut color_find = Vec::new();
		let mut color_replace = Vec::new();
		let mut texture_find = Vec::new();
		let mut texture_replace = Vec::new();
		let mut stockmarket = false;
		let mut male_model12 = 0;
		let mut female_model12 = 0;
		let mut male_head_model1 = 0;
		let mut female_head_model1 = 0;
		let mut male_head_model2 = 0;
		let mut female_head_model2 = 0;
		let mut zan2d = 0;
		let mut noted_id = 0;
		let mut noted_template = 0;
		let mut count_obj = None;
		let mut count_co = [0; 10];
		let mut resize_x = 0;
		let mut resize_y = 0;
		let mut resize_z = 0;
		let mut ambient = 0;
		let mut contrast = 0;
		let mut team = 0;
		let mut bought_link = 0;
		let mut bought_tempalte = 0;
		let mut params = HashMap::new();

		let mut reader = BufReader::new(&buffer[..]);

		loop {
			let opcode = reader.read_u8();

			match opcode {
				0 => break,
				1 => { inventory_model = reader.read_u16(); },
				2 => { name = reader.read_string(); },
				4 => { zoom2d = reader.read_u16(); },
				5 => { x_an2d = reader.read_u16(); },
				6 => { y_an2d = reader.read_u16(); },
				7 => { x_offset2d = reader.read_u16(); },
				8 => { y_offset2d = reader.read_u16(); },
				11 => stackable = true,
				12 => { cost = reader.read_i32(); },
				16 => members_only = true,
				23 => {
					male_model10 = reader.read_u16();
					male_model_offset = reader.read_u8();
				},
				24 => { male_model1 = reader.read_u16(); },
				25 => {
					female_model10 = reader.read_u16();
					female_model_offset = reader.read_u8();
				},
				26 => { female_model1 = reader.read_u16(); },
				30..=34 => { options[opcode as usize - 30] = reader.read_string(); },
				35..=39 => { interface_options[opcode as usize - 35] = reader.read_string(); },
				40 => {
					let len = reader.read_u8();
					for _ in 0..len {
						color_find.push(reader.read_u16());
						color_replace.push(reader.read_u16());
					}
				},
				41 => {
					let len = reader.read_u8();
					for _ in 0..len {
						texture_find.push(reader.read_u16());
						texture_replace.push(reader.read_u16());
					}
				},
				42 => { reader.read_u8(); },
				65 => stockmarket = true,
				78 => { male_model12 = reader.read_u16(); },
				79 => { female_model12 = reader.read_u16(); },
				90 => { male_head_model1 = reader.read_u16(); },
				91 => { female_head_model1 = reader.read_u16(); },
				92 => { male_head_model2 = reader.read_u16(); },
				93 => { female_head_model2 = reader.read_u16(); },
				95 => { zan2d = reader.read_u16(); },
				97 => { noted_id = reader.read_u16(); },
				98 => { noted_template = reader.read_u16(); stackable = true; },
				100..=109 => {
					if count_obj.is_none() {
						count_obj = Some([0; 10]);
						count_co = [0; 10];
					}
					reader.read_u16();
					//count_obj[opcode as usize - 100] = c_obj;
					count_co[opcode as usize - 100] = reader.read_u16();
				},
				110 => { resize_x = reader.read_u16(); },
				111 => { resize_y = reader.read_u16(); },
				112 => { resize_z = reader.read_u16(); },
				113 => { ambient = reader.read_i8(); },
				114 => { contrast = reader.read_i8(); },
				115 => { team = reader.read_u8(); },
				139 => { bought_link = reader.read_u16(); },
				140 => { bought_tempalte = reader.read_u16(); },
				148 | 149 => { reader.read_u16(); },
				249 => {
					let len = reader.read_u8();

					for _ in 0..len {
						let is_string = reader.read_u8() == 1;
						let key = reader.read_u24();
						
						let value = if is_string {
							reader.read_string()
						} else {
							reader.read_i32().to_string()
						};

						params.insert(key, value);
					}
				}
				_ => unreachable!()
			}
		}

		Self {
			id,
			inventory_model,
			name,
			zoom2d,
			x_an2d,
			y_an2d,
			x_offset2d,
			y_offset2d,
			stackable,
			cost,
			members_only,
			male_model10,
			male_model_offset,
			male_model1,
			female_model10,
			female_model_offset,
			female_model1,
			options,
			interface_options,
			color_find,
			color_replace,
			texture_find,
			texture_replace,
			stockmarket,
			male_model12,
			female_model12,
			male_head_model1,
			female_head_model1,
			male_head_model2,
			female_head_model2,
			zan2d,
			noted_id,
			noted_template,
			count_obj,
			count_co,
			resize_x,
			resize_y,
			resize_z,
			ambient,
			contrast,
			team,
			bought_link,
			bought_tempalte,
			params,
		}
	}

}