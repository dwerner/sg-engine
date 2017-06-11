extern crate game_state;

use game_state::state;
use game_state::state::SimulationAccess;
use game_state::time::Duration;


#[no_mangle]
pub extern "C" fn mod_simulation_load( s: &mut state::State ) {

}

#[no_mangle]
pub extern "C" fn mod_simulation_update( s: &mut state::State, dt: &Duration) {
    //println!("sim tick, probably need deltatime (since this mod was last ticked)");
}

#[no_mangle]
pub extern "C" fn mod_simulation_unload( s: &mut state::State ) {
}
