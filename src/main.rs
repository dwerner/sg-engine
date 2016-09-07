/*
 *	 Idea: arbitrary model support through observables (a framework for simulation state sync?)
 *
 *   - Synchronization model
 *
 *   http://gafferongames.com/networked-physics/snapshot-compression/
 *
 *   https://github.com/rygorous/gaffer_net/blob/master/main.cpp
 *
 *   Target bandwidth range vs Actual bandwidth use
 *
 * 	 ????
 *   - separate sync mechanisms for separate properties
 *   - prioritization of sync for different properties
 *   - adaptive sync methodology
 *   - express changing values as rate of change for interp
 *   - trans Simuation migration of objects
 *   - support simulation-level persistence (binary file, and maybe redis?)
 *   - property bounding (limit range, define quantization)
 *   - custom property serialization traits (e.g. quaternion's 'smallest three')
 *   - delta compression - send only what has changed
 *   - arbitrary precision types (like varint)
 *   - desync handling
 *   
 *   Knobs:
 *	 - snapshot send rate (per second)
 *	 - packet size
 *	 - interpolation between snapshots in buffer
 *	 - size of snapshot buffer
 *	 - extrapolation of velocities, linear and angular
 *	 - protocol (tcp/udp) - udp send/ack_send
 *	 - data compression (none, zlib, compress)
 *
 *	 Detections:
 *	 - snapshot length in bytes
 *	 - bandwidth
 *	 - latency
 *	 - packet loss
 *
 *	 Deterministic Lock-step
 *	 Snapshots and Interpolation (send all state)
 *	 State synchronization
 *
 *	 p2p vs client/server
 *
 *	 Gameworld = [ x x x x x x ] ==> [ x x x x x y ] === [ 6x, x->y ]
 *
 *	 more scrap:
 *	 		- sync priority (level of detail for syncs)
 *	 			- near points of interest (high-low) etc
 *
 *
 *	 'Object model'
 *
 *	 SimSync
 *	  \-> Schedule
 *	 	  \-> Object
 *	 		  \-> Object
 *	 			  \-> ...
 */

/*
 * SyncUpdate - the main 
 */

extern crate winit;
extern crate vulkano;
extern crate rustc_serialize;
extern crate bincode;
extern crate capnp;
extern crate time;

use bincode::SizeLimit;
use bincode::rustc_serialize::{encode/*, decode*/};

use rustc_serialize::{json /*, Encodable, Decodable*/};

extern crate engine;
use engine::renderer;
use engine::renderer::renderer::{Vertex};

//use std::fmt::Debug;

#[derive(Debug, RustcEncodable, RustcDecodable, PartialEq)]
struct SyncUpdate<M> {
	sequence: u32, 
	data: M
}

#[derive(Debug, RustcEncodable, RustcDecodable, PartialEq)]
enum Anything {
	Nothing,
	Everything(u8),
}

#[derive(Debug, RustcEncodable, RustcDecodable, PartialEq)]
struct Datum {
	v: u8,
}

#[derive(Debug, RustcEncodable, RustcDecodable, PartialEq)]
struct Datum2(u8);

#[derive(Debug, RustcEncodable, RustcDecodable, PartialEq)]
struct BoolDat(bool);

fn main() {

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

	let mut renderer = engine::renderer::renderer::Renderer::new();

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

	let vertex_buffer = renderer.createBuffer(items);
	
	let mut frame = 0;
	'running: loop {
		frame += 1;
		if frame % 100 == 0 {
			println!("FPS: {}", renderer.fps());
		}

		std::thread::sleep(std::time::Duration::from_millis(16));
		renderer.render(&vertex_buffer);

		// Make use of winit
		for ev in renderer.window().window().poll_events() {
			match ev {
				winit::Event::Closed => {
					println!("Window closed.");
					return;
				},
				_ => ()
			}
		}
	}
}
