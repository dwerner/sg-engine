extern crate game_state;

use game_state::state;

#[no_mangle]
pub extern "C" fn use_state2( s: &mut state::State ) {
	s.blob += 3;
	println!("simulation2 says: {} {}", s.name, s.blob);
}
