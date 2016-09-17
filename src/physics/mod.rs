pub mod world;

#[test]
fn a_whole_new_world() { 
	let world = world::World::new();
	assert!(world.bodies.len() == 0);
	for body in world.bodies {
		assert!(body.body().x == 0f32);
	}
}
