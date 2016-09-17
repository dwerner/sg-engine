use cgmath::{Vector3, Transform};

#[test]
fn test_vector3_add() {
	let a = Vector3::new(1.0, 1.0, 1.0);
	let b = Vector3::new(2.0, 3.0, 4.0);
	let c = a + b;
	assert!( c.x == 3.0 );
	assert!( c.y == 4.0 );
	assert!( c.z == 5.0 );
}

pub struct GameObject { }

