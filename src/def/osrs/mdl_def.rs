use std::{
	io,
	io::BufReader,
	collections::HashMap,
};

use crate::{ Definition, ext::ReadExt, util };

#[derive(Clone, Eq, PartialEq, Debug, Default)]
pub struct ModelDefinition {
	pub id: u32,
	pub texture_render_types: Vec<u8>,
	pub vertex_count: usize,
	pub triangle_count: usize,
	pub texture_triangle_count: usize,
	pub vertex_position_x: Vec<u16>,
	pub vertex_position_y: Vec<u16>,
	pub vertex_position_z: Vec<u16>,
	pub face_vertex_indices1: Vec<u8>,
	pub face_vertex_indices2: Vec<u8>,
	pub face_vertex_indices3: Vec<u8>,
	pub vertex_skins: Option<Vec<u8>>,
	pub face_render_types: Option<Vec<i8>>,
	pub face_render_priorities: Option<Vec<i8>>,
	pub priority: u8,
	pub face_alphas: Option<Vec<i8>>,
	pub face_skins: Option<Vec<u8>>,
	pub face_textures: Option<Vec<i16>>,
	pub face_coordinates: Option<Vec<u8>>,
	pub face_colors: Vec<u16>,
	pub texture_coordinates: Option<Vec<i8>>,
	pub texture_triangle_vertex_indices1: Option<Vec<u8>>,
	pub texture_triangle_vertex_indices2: Option<Vec<u8>>,
	pub texture_triangle_vertex_indices3: Option<Vec<u8>>,
	pub texture_primary_colors: Option<Vec<u8>>,
}

impl Definition for ModelDefinition {
	#[inline]
    fn new(id: u32, buffer: &[u8]) -> io::Result<Self> {
		let mdl_def = decode_buffer(id, buffer)?;
		
		Ok(mdl_def)
    }
}

fn decode_buffer(id: u32, buffer: &[u8]) -> io::Result<ModelDefinition> {
	let mut mdl_def = ModelDefinition {
		id,
		.. ModelDefinition::default()
	};

	let mut var2 = BufReader::new(&buffer[buffer.len() - 23..buffer.len()]);

	let vertices_count = var2.read_u16()? as usize;
	let triangle_count = var2.read_u16()? as usize;
	let texture_triangle_count = var2.read_u8()? as usize;
	let var13 = var2.read_u8()?;
	let model_priority = var2.read_u8()?;
	let var50 = var2.read_u8()?;
	let var17 = var2.read_u8()?;
	let model_texture = var2.read_u8()?;
	let model_vertex_skins = var2.read_u8()?;
	let var20 = var2.read_u16()? as usize;
	let var21 = var2.read_u16()? as usize;
	let var42 = var2.read_u16()? as usize;
	let var22 = var2.read_u16()? as usize;
	let var38 = var2.read_u16()? as usize;
	let mut var7 = 0;
	let mut var29 = 0;
	let mut texture_count = 0;

	var2 = BufReader::new(&buffer[..buffer.len() - 23]);

	if texture_triangle_count > 0 {
		for _ in 0..texture_triangle_count {
			let render_type = var2.read_u8()?;
			mdl_def.texture_render_types.push(render_type);

			if render_type == 0 {
				texture_count += 1;
			}

			if (1..=3).contains(&render_type) {
				var7 += 1;
			}

			if render_type == 2 {
				var29 += 1;
			}
		}
	}

	let mut position = texture_triangle_count + vertices_count;
	let render_type_pos = position;
	if var13 == 1 {
		position += triangle_count;
	}
	let var49 = position;
	position += triangle_count;
	let priority_pos = position;
	if model_priority == 255 {
		position += triangle_count;
	}

	let triangle_skin_pos = position;
	if var17 == 1 {
		position += triangle_count;
	}

	let var35 = position;
	if model_vertex_skins == 1 {
		position += vertices_count;
	}

	let alpha_pos = position;
	if var50 == 1 {
		position += triangle_count;
	}

	let var11 = position;
	position += var22;
	let texture_pos = position;
	if model_texture == 1 {
		position += triangle_count * 2;
	}

	let texture_coord_pos = position;
	position += var38;
	let color_pos = position;
	position += triangle_count * 2;
	let var40 = position;
	position += var20;
	let var41 = position;
	position += var21;
	let var8 = position;
	position += var42;
	let var43 = position;
	position += texture_count * 6;
	let var37 = position;
	position += var7 * 6;
	let var48 = position;
	position += var7 * 6;
	let var56 = position;
	position += var7 * 2;
	let var45 = position;
	position += var7;
	let var46 = position;
	position += var7 * 2 + var29 * 2;
	mdl_def.vertex_count = vertices_count;
	mdl_def.triangle_count = triangle_count;
	mdl_def.texture_triangle_count = texture_triangle_count;
	mdl_def.vertex_position_x = Vec::with_capacity(vertices_count);
	mdl_def.vertex_position_y = Vec::with_capacity(vertices_count);
	mdl_def.vertex_position_z = Vec::with_capacity(vertices_count);
	mdl_def.face_vertex_indices1 = Vec::with_capacity(triangle_count);
	mdl_def.face_vertex_indices2 = Vec::with_capacity(triangle_count);
	mdl_def.face_vertex_indices3 = Vec::with_capacity(triangle_count);
	if model_vertex_skins == 1 {
		mdl_def.vertex_skins = Some(Vec::with_capacity(vertices_count));
	}

	if var13 == 1 {
		mdl_def.face_render_types = Some(Vec::with_capacity(triangle_count));
	} else {
		mdl_def.priority = model_priority;
	}

	if var50 == 1 {
		mdl_def.face_alphas = Some(Vec::with_capacity(triangle_count));
	}

	if var17 == 1 {
		mdl_def.face_skins = Some(Vec::with_capacity(triangle_count));
	}

	if model_texture == 1 {
		mdl_def.face_textures = Some(Vec::with_capacity(triangle_count));
	}

	if model_texture == 1 && texture_triangle_count > 0 {
		mdl_def.texture_coordinates = Some(Vec::with_capacity(triangle_count));
	}

	mdl_def.face_colors = Vec::with_capacity(triangle_count);
	if texture_triangle_count > 0 {
		mdl_def.texture_triangle_vertex_indices1 = Some(Vec::with_capacity(texture_triangle_count));
		mdl_def.texture_triangle_vertex_indices2 = Some(Vec::with_capacity(texture_triangle_count));
		mdl_def.texture_triangle_vertex_indices3 = Some(Vec::with_capacity(texture_triangle_count));
		if var7 > 0 {
			// skip
		}

		if var29 > 0 {
			mdl_def.texture_primary_colors = Some(Vec::with_capacity(var29));
		}
	}

	var2 = BufReader::new(&buffer[texture_triangle_count..]);
	let mut var24 = BufReader::new(&buffer[var40..]);
	let mut var3 = BufReader::new(&buffer[var41..]);
	let mut var28 = BufReader::new(&buffer[var8..]);
	let mut var6 = BufReader::new(&buffer[var35..]);
	let mut vX = 0;
	let mut vY = 0;
	let mut vZ = 0;

	let mut vertex_z_offset = 0;
	let mut var10 = 0;
	let mut vertex_y_offset = 0;
	let mut var15 = 0;
	for point in 0..vertices_count {
		let vertex_flags = var2.read_u8()?;
		let mut vertex_x_offset = 0;
		let vertex_x_offset = if (vertex_flags & 1) != 0 {
			var24.read_smart_u16()?
		} else { 0 };

		vertex_y_offset = 0;
		if (vertex_flags & 2) != 0 {
			vertex_y_offset = var3.read_smart_u16()?;
		}

		vertex_z_offset = 0;
		if (vertex_flags & 4) != 0 {
			vertex_z_offset = var28.read_smart_u16()?;
		}

		mdl_def.vertex_position_x[point] = vX + vertex_x_offset;
		mdl_def.vertex_position_y[point] = vY + vertex_y_offset;
		mdl_def.vertex_position_z[point] = vZ + vertex_z_offset;
		vX = mdl_def.vertex_position_x[point];
		vY = mdl_def.vertex_position_y[point];
		vZ = mdl_def.vertex_position_z[point];
		if model_vertex_skins == 1 {
			if let Some(vertex_skins) = mdl_def.vertex_skins.as_mut() {
				vertex_skins[point] = var6.read_u8()?;
			}
		}
	}

	var2 = BufReader::new(&buffer[color_pos..]);
	var24 = BufReader::new(&buffer[render_type_pos..]);
	var3 = BufReader::new(&buffer[priority_pos..]);
	var28 = BufReader::new(&buffer[alpha_pos..]);
	var6 = BufReader::new(&buffer[triangle_skin_pos..]);
	let mut var55 = BufReader::new(&buffer[texture_pos..]);
	let mut var51 = BufReader::new(&buffer[texture_coord_pos..]);

	for point in 0..triangle_count {
		mdl_def.face_colors[point] = var2.read_u16()?;
		if var13 == 1 {
			if let Some(face_render_types) = mdl_def.face_render_types.as_mut() {
				face_render_types[point] = var24.read_i8()?;
			}
		}

		if model_priority == 255 {
			if let Some(face_render_priorities) = mdl_def.face_render_priorities.as_mut() {
				face_render_priorities[point] = var3.read_i8()?;
			}
		}

		if var50 == 1 {
			if let Some(face_alphas) = mdl_def.face_alphas.as_mut() {
				face_alphas[point] = var28.read_i8()?;
			}
		}

		if var17 == 1 {
			if let Some(face_skins) = mdl_def.face_skins.as_mut() {
				face_skins[point] = var6.read_u8()?;
			}
		}

		if model_texture == 1 {
			if let Some(face_textures) = mdl_def.face_textures.as_mut() {
				face_textures[point] = (var55.read_u16()?).wrapping_sub(1) as i16;
			}
		}

		if mdl_def.texture_coordinates.is_none() {
			if let Some(face_textures) = mdl_def.face_textures.as_ref() {
				if face_textures[point] != -1 {
					if let Some(texture_coordinates) = mdl_def.texture_coordinates.as_mut() {
						texture_coordinates[point] = (var51.read_u8()?).wrapping_sub(1) as i8;
					}
				}
			}
		}
	}

	

	unimplemented!()
}