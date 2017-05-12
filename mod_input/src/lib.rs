extern crate game_state;

use game_state::state::State;

//use game_state::input::events::InputEvent;
use game_state::state::InputAccess;


// this module's purpose is to turn input events into meaningful application input
// this might include closing windows, keyboard presses, mouse drags
// mapping user settings to keyboard and mouse bindings, joystick event source

#[no_mangle]
pub extern "C" fn mod_input_load( state: &mut State ) {
    state.on_input_load();
}

#[no_mangle]
pub extern "C" fn mod_input_tick( state: &mut State ) {
    state.clear_input_events();

    state.gather_input_events();
    if state.has_pending_input_events() {
        //println!("mod_input pending events -> {:?}", state.get_input_events());
    }
}

#[no_mangle]
pub extern "C" fn mod_input_unload( state: &mut State ) {
    state.on_input_unload();
}
