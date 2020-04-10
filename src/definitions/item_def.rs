use std::{
	io::{
		Read,
		BufReader,
	},
	collections::HashMap,
};

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
			let opcode = read_u8(&mut reader);

			match opcode {
				0 => break,
				1 => { inventory_model = read_u16(&mut reader); },
				2 => { name = read_string(&mut reader); },
				4 => { zoom2d = read_u16(&mut reader); },
				5 => { x_an2d = read_u16(&mut reader); },
				6 => { y_an2d = read_u16(&mut reader); },
				7 => { x_offset2d = read_u16(&mut reader); },
				8 => { y_offset2d = read_u16(&mut reader); },
				11 => stackable = true,
				12 => { cost = read_i32(&mut reader); },
				16 => members_only = true,
				23 => {
					male_model10 = read_u16(&mut reader);
					male_model_offset = read_u8(&mut reader);
				},
				24 => { male_model1 = read_u16(&mut reader); },
				25 => {
					female_model10 = read_u16(&mut reader);
					female_model_offset = read_u8(&mut reader);
				},
				26 => { female_model1 = read_u16(&mut reader); },
				30..=34 => { options[opcode as usize - 30] = read_string(&mut reader); },
				35..=39 => { interface_options[opcode as usize - 35] = read_string(&mut reader); },
				40 => {
					let len = read_u8(&mut reader);
					for _ in 0..len {
						color_find.push(read_u16(&mut reader));
						color_replace.push(read_u16(&mut reader));
					}
				},
				41 => {
					let len = read_u8(&mut reader);
					for _ in 0..len {
						texture_find.push(read_u16(&mut reader));
						texture_replace.push(read_u16(&mut reader));
					}
				},
				42 => { read_u8(&mut reader); },
				65 => stockmarket = true,
				78 => { male_model12 = read_u16(&mut reader); },
				79 => { female_model12 = read_u16(&mut reader); },
				90 => { male_head_model1 = read_u16(&mut reader); },
				91 => { female_head_model1 = read_u16(&mut reader); },
				92 => { male_head_model2 = read_u16(&mut reader); },
				93 => { female_head_model2 = read_u16(&mut reader); },
				95 => { zan2d = read_u16(&mut reader); },
				97 => { noted_id = read_u16(&mut reader); },
				98 => { noted_template = read_u16(&mut reader); stackable = true; },
				100..=109 => {
					if count_obj.is_none() {
						count_obj = Some([0; 10]);
						count_co = [0; 10];
					}
					read_u16(&mut reader);
					//count_obj[opcode as usize - 100] = c_obj;
					count_co[opcode as usize - 100] = read_u16(&mut reader);
				},
				110 => { resize_x = read_u16(&mut reader); },
				111 => { resize_y = read_u16(&mut reader); },
				112 => { resize_z = read_u16(&mut reader); },
				113 => { ambient = read_i8(&mut reader); },
				114 => { contrast = read_i8(&mut reader); },
				115 => { team = read_u8(&mut reader); },
				139 => { bought_link = read_u16(&mut reader); },
				140 => { bought_tempalte = read_u16(&mut reader); },
				148 | 149 => { read_u16(&mut reader); },
				249 => {
					let len = read_u8(&mut reader);

					for _ in 0..len {
						let is_string = read_u8(&mut reader) == 1;
						let key = read_u24(&mut reader);
						
						let value = if is_string {
							read_string(&mut reader)
						} else {
							read_i32(&mut reader).to_string()
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

fn read_u8(reader: &mut BufReader<&[u8]>) -> u8 {
	let mut buffer = [0; 1];
	reader.read_exact(&mut buffer).unwrap();
	u8::from_be_bytes(buffer)
}

fn read_i8(reader: &mut BufReader<&[u8]>) -> i8 {
	let mut buffer = [0; 1];
	reader.read_exact(&mut buffer).unwrap();
	i8::from_be_bytes(buffer)
}

fn read_u16(reader: &mut BufReader<&[u8]>) -> u16 {
	let mut buffer = [0; 2];
	reader.read_exact(&mut buffer).unwrap();
	u16::from_be_bytes(buffer)
}

fn read_u24(reader: &mut BufReader<&[u8]>) -> u32 {
	let mut buffer = [0; 3];
	reader.read_exact(&mut buffer).unwrap();
	((buffer[0] as u32) << 16) | ((buffer[1] as u32) << 8) | (buffer[2] as u32)
}

fn read_i32(reader: &mut BufReader<&[u8]>) -> i32 {
	let mut buffer = [0; 4];
	reader.read_exact(&mut buffer).unwrap();
	i32::from_be_bytes(buffer)
}

fn read_string(reader: &mut BufReader<&[u8]>) -> String {
	let mut bytes = Vec::new();

	loop {
		let mut buffer = [0; 1];
		reader.read_exact(&mut buffer).unwrap();
		let byte = u8::from_be_bytes(buffer);
		if byte != 0 {
			bytes.push(byte);
		} else {
			break;
		}
	}

	String::from_utf8_lossy(&bytes[..]).to_string()
}