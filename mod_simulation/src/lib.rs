use std::time::Duration;

use game_state::state;

#[no_mangle]
pub extern "C" fn mod_simulation_load(_s: &mut state::State) {}

#[no_mangle]
pub extern "C" fn mod_simulation_update(_s: &mut state::State, _dt: &Duration) {
    //println!("sim tick, probably need deltatime (since this mod was last ticked)");
}

#[no_mangle]
pub extern "C" fn mod_simulation_unload(_s: &mut state::State) {}
