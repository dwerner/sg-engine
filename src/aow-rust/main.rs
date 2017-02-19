extern crate aow_rust;

#[macro_use]
extern crate glium;

extern crate image;

use std::io::Cursor;
use std::time::Duration;

// our crate is auto-renamed from aow-rust (as specified in our Cargo.toml)
// to aow_rust because the dash is illegal, just as in C
use aow_rust::model::Model;
use aow_rust::shaders::{ VERTEX_SHADER_SRC, FRAGMENT_SHADER_SRC };
use aow_rust::options::Options;


struct Position {
	bound:f32,
	x: f32,
	y: f32,
	hvel: f32,
	vvel: f32,
	r:f32,
}

impl Position {
	fn new() -> Self {
		let speed = 0.01;
		Position {
			bound: 0.5,
			x: -0.5,
			y: -0.5,
			hvel: speed,
			vvel: speed+0.002,
			r: 1.0,
		}
	}
	fn update(&mut self) {
		if self.x > self.bound { self.hvel = -self.hvel; }
		if self.x < -self.bound { self.hvel = -self.hvel; }
		if self.y > self.bound { self.vvel = -self.vvel; }
		if self.y < -self.bound { self.vvel = -self.vvel; }
		self.x += self.hvel;
		self.y += self.vvel;
		self.r += 0.01;
	}
	fn x(&self) -> f32 { self.x }
	fn y(&self) -> f32 { self.y }
	fn r(&self) -> f32 { self.r }
}

fn main() {
	use glium::{DisplayBuild, Surface};

	let _ = Options::load();
	
    let mut model = Model::new();
    model.load("assets/container.3ds");
	let shape = model.vertices;

	let image = image::load(
		Cursor::new(&include_bytes!(
				"../assets/opengl.jpg"
		)[..]),
	image::JPEG).unwrap().to_rgba();

	let image_dimensions = image.dimensions();

	let raw_image = glium::texture::RawImage2d::from_raw_rgba_reversed(
		image.into_raw(),
		image_dimensions
	);

	let window = glium::glutin::WindowBuilder::new().
        with_vsync().
        with_title("Assembly of Worlds").
		build_glium().
		unwrap();

	let vertex_buffer = glium::VertexBuffer::new(&window, &shape).unwrap();
	let texture = glium::texture::Texture2d::new(&window, raw_image).unwrap();

	let indices = glium::index::NoIndices(
		glium::index::PrimitiveType::TriangleStrip
	);


	let program = glium::Program::from_source(
		&window,
		VERTEX_SHADER_SRC,
		FRAGMENT_SHADER_SRC,
		None
	).unwrap();


	let mut pos = Position::new();

	loop {

        std::thread::sleep(Duration::from_millis(16)); // yes it's still a broken frame loop. :P

		pos.update();

		let mut target = window.draw();
		target.clear_color(0.0, 0.0, 1.0, 1.0);

		let uniforms = uniform! {
			matrix: [
				[ pos.r().cos(), pos.r().sin(), 0.0, 0.0],
				[ -pos.r().sin(), pos.r().cos(), 0.0, 0.0],
				[0.0, 0.0, 1.0, 0.0],
				[ pos.x() , pos.y(), 0.0, 1.0f32],
			],
			tex: &texture,
		};

		target.draw(
			&vertex_buffer,
			&indices,
			&program,
			&uniforms,
			&Default::default()
		).unwrap(); 

		target.finish().unwrap();
		for event in window.poll_events() {
			match event {
				glium::glutin::Event::Closed => return,
				_ => ()
			}
		}
	}

}
