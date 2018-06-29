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
                InputEvent::KeyUp(_id, keycode) => {
                    let q = 'q' as u32;
                    let kc = keycode as u8;
                    println!("user released {}", kc);
                    match kc as char {
                        _ => {}
                    }
                },
                InputEvent::KeyDown(_id, keycode) => {

                    let q = 'q' as u32;
                    let kc = keycode as u8;
                    println!("user pressed {} {}", kc, q);
                    match keycode {
                        16 => {
                            println!("user pressed 'q' : hard exit");
                            std::process::exit(0);
                        }
                        _ => {}
                    }
                },
                InputEvent::MouseMove(_id, _sp, delta) => {
                    let sensitivity = 100.0;
                    let (dx,dy) = (delta.delta_x as f32, delta.delta_y as f32);
                    let mut camera = &mut state.get_world().get_facets().cameras[0];
                    let xa = cgmath::Rad(-dx/sensitivity);
                    let ya = cgmath::Rad(-dy/sensitivity);
                    let from = camera.view;
                    camera.view = from * cgmath::Matrix4::from_angle_x(ya) * cgmath::Matrix4::from_angle_y(xa);

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
