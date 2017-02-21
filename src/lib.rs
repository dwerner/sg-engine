pub mod net;
pub mod physics;
pub mod renderer;
pub mod game;

#[macro_use]
extern crate vulkano;

extern crate winit;
extern crate vulkano_win;
extern crate glsl_to_spirv;
extern crate vulkano_shaders;
extern crate cgmath;
extern crate bincode;
extern crate rustc_serialize;

extern crate libloading;
extern crate subproject;

use libloading::{Symbol, Library};
use subproject::state;

#[cfg(target_os = "windows")]
const LIBPATH: &'static str = "../../subproject/target/debug/deps/subproject.dll";

#[cfg(target_os = "linux")]
const LIBPATH: &'static str = "subproject/target/debug/deps/libsubproject.so";

pub fn load_state(state: &mut state::State) {

	// TODO: add last-modified-date check to this
	let lib = Library::new(LIBPATH);
	match lib {
		Ok(lib) => {
			unsafe {
				let f: Result<Symbol<unsafe extern fn(&mut state::State)>, std::io::Error> = lib.get(b"use_state\0");
				match f {
					Ok(use_state) => {
						use_state(state);
					},
					Err(err) => println!("{}", err)
				}
			}
		},
		Err(err) => {
			println!("{}", err);
		}
	}
}

