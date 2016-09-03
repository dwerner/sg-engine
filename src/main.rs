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

extern crate rustc_serialize;
extern crate bincode;
extern crate capnp;

use bincode::SizeLimit;
use bincode::rustc_serialize::{encode/*, decode*/};

use rustc_serialize::{json /*, Encodable, Decodable*/};

extern crate sdl2;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

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

	let mut vec = Vec::with_capacity(256);
	for x in 0u8..255 {
		//vec.push(Anything::Everything(x));
		//vec.push(Datum{v:x});
		//vec.push(Datum2(x));
		vec.push(BoolDat( x > 124));
	}

	let update = SyncUpdate { sequence: 42, data: vec };
	let update_str = json::encode(&update).unwrap().to_string();
	let update_bin = encode(&update, SizeLimit::Infinite).unwrap();

	//println!("update_str {}", update_str);
	println!("update_str len: {}", update_str.len());
	println!("update_bin len: {}", update_bin.len());


	let sdl_context = sdl2::init().unwrap();
	let video_subsystem = sdl_context.video().unwrap();

	let window = video_subsystem.window("world state visualization", 800, 600)
		.position_centered()
		.opengl()
		.build()
		.unwrap();

	let mut renderer = window.renderer().build().unwrap();
	let mut texture = renderer.create_texture_streaming(
		PixelFormatEnum::RGB24, 256, 256).unwrap();

	// gradient in memory
	texture.with_lock(None, |buffer: &mut [u8], pitch: usize| {
		for y in 0..256 {
			for x in 0..256 {
				let offset = y*pitch + x*3;
				buffer[offset + 0] = x as u8;
				buffer[offset + 1] = y as u8;
				buffer[offset + 2] = 0;
			}
		}
	}).unwrap();

	renderer.clear();

	// copy prepared texture to Rect
	renderer.copy(&texture, None, Some(Rect::new(100, 100, 256, 256)));

	// copy prepared texture to rotated rect?
	renderer.copy_ex(
		&texture,
		None, 
		Some(Rect::new(450, 100, 256, 256)),
		0.0,
		None,
		false,
		false
		).unwrap();

	renderer.present();

	let mut event_pump = sdl_context.event_pump().unwrap();

	'running: loop {
		for event in event_pump.poll_iter() {
			match event {
				Event::Quit {..} // Window manager tries to quit
				| Event::KeyDown { keycode: Some(Keycode::Q), .. }
				| Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
					break 'running
				},
				_ => {}
			}
			// something here
		}
	}
}
