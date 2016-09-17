extern crate winit;
extern crate vulkano;
extern crate rustc_serialize;
extern crate time;

extern crate engine;
use engine::renderer::{Vertex, Renderer};

fn main() {

	play_wire_proto();

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


// Just playing around with some wireprotocol bits here.
extern crate bincode;
extern crate capnp;
fn play_wire_proto() {

	use bincode::SizeLimit;
	use bincode::rustc_serialize::{encode/*, decode*/};
	use rustc_serialize::{json /*, Encodable, Decodable*/};
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
