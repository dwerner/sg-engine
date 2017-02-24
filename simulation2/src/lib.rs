extern crate game_state;

use game_state::state;

#[no_mangle]
pub extern "C" fn use_state2( s: &mut state::State ) {
	s.blob += 3;
	let sound = if s.blob % 3 == 0 {
		"||>".to_string()
	} else {
		"<".to_string()
	};
    s.data.push(sound);
	println!("simulation2 says: {} {}, but I pushed a sound", s.name, s.blob);
}
