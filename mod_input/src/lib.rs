extern crate game_state;

use game_state::state::State;
use game_state::Renderer;

use std::sync::Arc;

#[no_mangle]
pub extern "C" fn mod_input_load( state: &mut State ) {
    state.input_state.clear();
}

#[no_mangle]
pub extern "C" fn mod_input_tick( state: &mut State ) {
    // Renderers own the input event loop associated with their
    // internals: i.e. the window manager window
    // - get input events and convert them to our internal format
    // and push them into the input events queue

    // we want to clear that queue each tick, regardless of if we dealt with the events
    state.input_state.pending_input_events.clear();

    // Now we want to
    for i in 0 .. state.renderers.len() {
        let mut events = state.renderers[i].get_input_events();
        state.input_state.pending_input_events.append(&mut events);
    }
}

#[no_mangle]
pub extern "C" fn mod_input_unload( state: &mut State ) {
    state.input_state.clear();
}
