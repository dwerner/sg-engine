use std::vec::Vec;

pub struct World {
	pub bodies: Vec<Body>
}

impl World {
	pub fn new() -> Self {
		World { bodies: Vec::new() }
	}
}

pub struct Body {
	x: f32,
	y: f32,
	z: f32
}
