use std;

use std::ops::{
	Add, AddAssign,
	Sub, SubAssign,
	Mul, MulAssign,
	Div, DivAssign
};

pub struct Vector3 {
	x: f32,
	y: f32,
	z: f32
}

impl Vector3 {
	fn new(x: f32, y: f32, z: f32) -> Self {
		Vector3{ x:x, y:y, z:z }
	}
}

impl Add for Vector3 {
	type Output = Vector3;

	fn add(self, other: Vector3) -> Vector3 {
		Vector3{ x: self.x + other.x, y: self.y + other.y, z: self.z + other.z }
	}
}

pub struct Transform {
	position: Vector3,
	rotation: Vector3,
	scale: Vector3
}

impl Transform {
	fn new(p: Vector3, r: Vector3, s: Vector3) -> Self {
		Transform{ position: p, rotation: r, scale: s }
	}
}

pub struct Matrix([f32;16]);

pub struct GameObject {
	transform: Transform
}

impl GameObject {
	fn new(t: Transform) -> Self {
		GameObject{ transform: t }
	}
}

#[test]
fn tests_should_work() {
	let p = Vector3::new(0f32, 0f32, 0f32);
	let r = Vector3::new(0f32, 0f32, 0f32);
	let s = Vector3::new(0f32, 0f32, 0f32);
	let t = Transform::new(p, r, s);
	let _ = GameObject::new(t);
	assert!(true);
}

#[test]
fn test_vector3_add() {
	let a = Vector3::new(1.0, 1.0, 1.0);
	let b = Vector3::new(2.0, 3.0, 4.0);
	let c = a + b;
	assert!( c.x == 3.0 );
	assert!( c.y == 4.0 );
	assert!( c.z == 5.0 );
}
