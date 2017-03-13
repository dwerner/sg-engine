pub mod net;
pub mod physics;
pub mod renderer;
pub mod libloader;
pub mod input;
pub mod ui;

#[macro_use]
extern crate vulkano;

extern crate winit;
extern crate vulkano_win;
extern crate glsl_to_spirv;
extern crate vulkano_shaders;
extern crate cgmath;
extern crate bincode;
extern crate time;

extern crate libloading;
extern crate ansi_term;

// our subproject
extern crate game_state;

