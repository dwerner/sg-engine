pub mod net;
pub mod physics;
pub mod renderer;
pub mod libloader;

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

// our subproject
extern crate game_state;

