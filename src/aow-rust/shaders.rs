
/*******************************************************************************
 * Copyright 2012 Jannis Tsiroyannis
 *
 * This file is part of Assembly of Worlds.
 *
 * Assembly of Worlds is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * Assembly of Worlds is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with Assembly of Worlds.  If not, see <http://www.gnu.org/licenses/>.
 *
 *****************************************************************************/
use libc::c_void; // TODO: we shouldn't need this

use super::textures::Texture;

#[allow(non_camel_case_types)]
const NUMBER_OF_SHADERS:u8 = 5;

/*******************************************************************************
 * Types
 *
 ******************************************************************************/
#[allow(non_camel_case_types)]
pub enum Shader {
    NO_SHADER = -1,
    TEXTURED_SHADER = 0,
    TEXTURED_LIT_SHADER,
    TEXTURED_LIT_SHADOWED_SHADER,
    SHADOW_SHADER,
    COLORED_SHADER,
}

/*******************************************************************************
 * Functions
 *
 ******************************************************************************/
pub fn initialize_shaders(){}
pub fn uninitialize_shaders(){}
pub fn set_shader(
    shader: Shader,
    /*GLint*/ vbo: i32,
    data: *const c_void, //TODO type this instead of using c_void
    r: f32,
    g: f32,
    b: f32){

}
pub fn set_texture(texture: Texture){}
pub fn set_texture_raw(/*GLint*/textureName: i32){}
pub fn set_shadow_map(/*GLint*/ textureName: i32){}
pub fn set_shader_mvp(){}
pub fn unset_shader(){}



// for now these are just the shaders I added for the glium example. will revisit
pub const VERTEX_SHADER_SRC: &'static str = r#"
	#version 140

	in vec2 position;
	in vec2 uv;
	out vec2 v_uv;

	uniform mat4 matrix;

	void main() {
		v_uv = uv;
		gl_Position = matrix * vec4(position, 0.0, 1.0);
	}
"#;

pub const FRAGMENT_SHADER_SRC: &'static str = r#"
	#version 140

	in vec2 v_uv;
	out vec4 color;

	uniform sampler2D tex;

	void main() {
		color = vec4(1,0,0,0);//texture(tex, v_uv);
	}
"#;
