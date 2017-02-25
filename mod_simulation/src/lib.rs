extern crate game_state;

use game_state::state;

#[no_mangle]
pub extern "C" fn mod_simulation_load( s: &mut state::State ) {
	println!("mod_simulation_load");
}

#[no_mangle]
pub extern "C" fn mod_simulation_tick( s: &mut state::State ) {
	s.blob += 1;
	println!("mod_simulation_tick {} {} {:?} ...", s.name, s.blob, s.data);
}

#[no_mangle]
pub extern "C" fn mod_simulation_unload( s: &mut state::State ) {
    s.blob = 0;
	println!("mod_simulation_unload");
}
