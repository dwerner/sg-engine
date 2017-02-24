extern crate winit;
extern crate vulkano;
extern crate time;
extern crate cgmath;

extern crate engine;
use engine::renderer::{Vertex, Renderer};
use engine::libloader::LibLoader;

use cgmath::Vector3;

extern crate subproject;
use std::time::Duration;
use subproject::{state};

use std::thread;

#[cfg(target_os = "windows")]
const LIBPATH: &'static str = "../../subproject/target/debug/deps/subproject.dll";

#[cfg(target_os = "linux")]
const LIBPATH: &'static str = "subproject/target/debug/deps/libsubproject.so";


fn main() {
	// spin off the dylib loader in the background,
	// pretty useless for now but shows basic functionality
	//thread::spawn(|| {
	play_dylib_load();
	//});

	// draw with Renderer / Vulkano
	play_draw_stuff();
}


fn play_dylib_load() {
	let mut state = state::State { blob: 42, name: "(I'm text from main.rs)".to_string(), data: vec!["arf".to_string()] };
	let mut loader = LibLoader::new(LIBPATH);
	loop {
		thread::sleep(Duration::from_millis(1000));
        loader.check();
        loader.func(&mut state);
	}
}

fn play_draw_stuff() {
	let mut renderer = Renderer::new();

	let red  = [1.0, 0.0, 0.0, 1.0];
	let blue = [0.0, 1.0, 0.0, 1.0];
	let green= [0.0, 0.0, 1.0, 1.0];
	let items = vec![
		Vertex::new([-0.5, -0.25, 0.0], red),
		Vertex::new([0.0, 0.5, 0.0], green),
		Vertex::new([0.25, -0.1, 0.0], blue),

		Vertex::new([0.5, 0.25, 0.0], red),
		Vertex::new([0.0, -0.5, 0.0], blue),
		Vertex::new([-0.25, 0.1, 0.0], green),
	];

	let vertex_buffer = renderer.create_buffer(items);
	
	let mut frame = 0;
	'running: loop {
		frame += 1;
		if frame % 100 == 0 {
			println!("FPS: {}", renderer.fps());
		}

		thread::sleep(Duration::from_millis(16));
		renderer.render(&vertex_buffer);

		// Make use of winit
		for ev in renderer.native_window().poll_events() {
			match ev {
				winit::Event::Closed => {
					println!("Window closed.");
					break 'running;
				},
				_ => ()
			}
		}
	}
}

