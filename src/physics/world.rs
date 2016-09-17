use std::vec::Vec;

use cgmath::{Vector3};

pub struct World {
	pub bodies: Vec<Body>
}

impl World {
	pub fn new() -> Self {
		World { bodies: Vec::new() }
	}
}

pub struct Body {
	body: Vector3<f32>
}

impl Body {
	pub fn body(&self) -> Vector3<f32> {
		self.body
	}
}
