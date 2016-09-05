pub mod net;
pub mod protocol;
pub mod physics;
pub mod renderer;

pub struct State {
	blah: i32
}

#[no_mangle]
pub extern "C" fn update(state: &mut State) {
	//do stuff
}

