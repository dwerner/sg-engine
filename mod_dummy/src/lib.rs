use std::time::Duration;

use game_state::state::State;

#[no_mangle]
pub extern "C" fn mod_dummy_load(state: &mut State) {}

#[no_mangle]
pub extern "C" fn mod_dummy_update(state: &mut State, dt: &Duration) {}

#[no_mangle]
pub extern "C" fn mod_dummy_unload(state: &mut State) {}
