extern crate game_state;
use game_state::state::State;

#[no_mangle]
pub extern "C" fn mod_dummy_load( state: &mut State ) {

}

#[no_mangle]
pub extern "C" fn mod_dummy_tick( state: &mut State ) {

}

#[no_mangle]
pub extern "C" fn mod_dummy_unload( state: &mut State ) {

}
