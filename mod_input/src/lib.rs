extern crate game_state;
extern crate cgmath;

use game_state::state::{
    State,
    InputAccess,
    WorldAccess,
};
use game_state::input::events::{
    InputEvent,
};
use game_state::input::screen::{
    ScreenPoint,
};
use game_state::time::Duration;


// this module's purpose is to turn input events into meaningful application input
// this might include closing windows, keyboard presses, mouse drags
// mapping user settings to keyboard and mouse bindings, joystick event source

#[no_mangle]
pub extern "C" fn mod_input_load( state: &mut State ) {
    state.on_input_load();
}

#[no_mangle]
pub extern "C" fn mod_input_update( state: &mut State, dt: &Duration ) {
    state.clear_input_events();

    state.gather_input_events();

    if state.has_pending_input_events() {
        let events = state.get_input_events().clone();
        for e in events {
            match e {
                InputEvent::MouseMove(_id, sp, _) => {
                    
                    let (x,y) = (sp.x as f32, sp.y as f32);
                    let mut camera = &mut state.get_world().get_facets().cameras[0];
                    camera.view = cgmath::Matrix4::from_angle_y(
                        cgmath::Rad((x/100.0) as f32)
                    ) + cgmath::Matrix4::from_angle_x(
                        cgmath::Rad((y/100.0) as f32)
                    );
                },
                _ => {}
            }
        }

    }
}

#[no_mangle]
pub extern "C" fn mod_input_unload( state: &mut State ) {
    state.on_input_unload();
}
