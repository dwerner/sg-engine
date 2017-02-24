extern crate game_state;

use game_state::state;

#[no_mangle]
pub extern "C" fn use_state( s: &mut state::State ) {
	s.blob -= 1;
	println!("simulation says:  {} {} {:?}", s.name, s.blob, s.data);
}
