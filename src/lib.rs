pub mod net;
pub mod protocol;
pub mod physics;
pub mod renderer;

#[macro_use]
extern crate vulkano;

extern crate winit;
extern crate vulkano_win;
extern crate glsl_to_spirv;
extern crate vulkano_shaders;

pub struct State {
	blah: i32
}

#[no_mangle]
pub extern "C" fn update(state: &mut State) {
	//do stuff
}

