pub mod world;

#[test]
fn a_whole_new_world() { 
	let world = world::World::new();
	assert!(world.bodies.len() == 0);
}
