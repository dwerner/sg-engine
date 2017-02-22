extern crate winit;
extern crate vulkano;
extern crate time;
extern crate cgmath;

extern crate engine;
use engine::renderer::{Vertex, Renderer};
use engine::game::{Region, RegionId, Game, GameObject, ObjectId};
use engine::LibLoader;

// Just playing around with some wireprotocol bits here.
extern crate bincode;
extern crate capnp;
extern crate rustc_serialize;
use bincode::SizeLimit;
use bincode::rustc_serialize::{encode/*, decode*/};
use rustc_serialize::{json /*, Encodable, Decodable*/};

use cgmath::Vector3;

extern crate subproject;
use std::time::Duration;
use subproject::{state};

use std::thread;


fn main() {
	let vec3 = Vector3::new(0.0, 0.0, 0.0);
	let bytes = encode(&vec3, bincode::SizeLimit::Infinite).unwrap();
	println!("Vector3 bin length: {}", bytes.len());

	play_gameobject_bin();
	play_region_bin();
	play_game_bin();
	play_game_loaded_bin();

	// spin off the dylib loader in the background, 
	// pretty useless for now but shows basic functionality
	//thread::spawn(|| {
		play_dylib_load();
	//});

	// draw with Renderer / Vulkano
	// play_draw_stuff();
}

fn play_dylib_load() {
	let mut state = state::State { blob: 42, name: "(I'm text from main.rs)".to_string(), data: vec!["arf".to_string()] };
	let mut loader = LibLoader::new();
	loop {
		std::thread::sleep(Duration::from_millis(1000));
        loader.check();
        loader.func(&mut state);
	}
}

fn play_gameobject_bin() {
	let obj = GameObject::new(
		ObjectId(0),
		Vector3::new(0.0, 0.0, 0.0),
		Vector3::new(0.0, 0.0, 0.0),
		Vector3::new(0.0, 0.0, 0.0),
	);
	let bytes = encode(&obj, bincode::SizeLimit::Infinite).unwrap();
	println!("empty GameObject bin length: {}", bytes.len());
}


fn play_region_bin() {
	let region = Region::new(RegionId(0));
	let bytes = encode(&region, bincode::SizeLimit::Infinite).unwrap();
	println!("empty Region bin length: {}", bytes.len());
}

fn play_game_bin() {
	let game = Game::new(Vec::new());
	let bytes = encode(&game, bincode::SizeLimit::Infinite).unwrap();
	println!("empty Game bin length: {}", bytes.len());
}

fn play_game_loaded_bin() {
	let mut r = Region::new(RegionId(0));
	for i in 0..99 {
		let o = GameObject::new(
			ObjectId(i),
			Vector3::new(0.0, 0.0, 0.0),
			Vector3::new(0.0, 0.0, 0.0),
			Vector3::new(0.0, 0.0, 0.0),
		);
		r.add_game_object(o);
	}

	let mut game = Game::new(vec![r]);

	let bytes = encode(&game, bincode::SizeLimit::Infinite).unwrap();
	println!("full game state bin length: {}", bytes.len());
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

		std::thread::sleep(std::time::Duration::from_millis(16));
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



fn play_wire_proto() {

	#[derive(Debug, RustcEncodable, RustcDecodable, PartialEq)] struct SyncUpdate<M> { sequence: u32, data: M }
	#[derive(Debug, RustcEncodable, RustcDecodable, PartialEq)] enum Anything { Nothing, Everything(u8), }
	#[derive(Debug, RustcEncodable, RustcDecodable, PartialEq)] struct Datum { v: u8, }
	#[derive(Debug, RustcEncodable, RustcDecodable, PartialEq)] struct Datum2(u8);
	#[derive(Debug, RustcEncodable, RustcDecodable, PartialEq)] struct BoolDat(bool);

	let pwd = std::env::current_dir();
	println!("{:?}", pwd);

	let mut vec = Vec::with_capacity(256);
	for x in 0u8..255 {
		vec.push(BoolDat( x > 124));
	}

	let update = SyncUpdate { sequence: 42, data: vec };
	let update_str = json::encode(&update).unwrap().to_string();
	let update_bin = encode(&update, SizeLimit::Infinite).unwrap();

	//println!("update_str {}", update_str);
	println!("update_str len: {}", update_str.len());
	println!("update_bin len: {}", update_bin.len());
}
