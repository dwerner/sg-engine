extern crate game_state;
extern crate cgmath;

use game_state::{
    Identity,
    Identifyable,
};

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

use std::collections::VecDeque;

use game_state::input::InputSource;
use game_state::input::events::JoyButton;

use game_state::time::Duration;

use game_state::thing::Direction;

// this module's purpose is to turn input events into meaningful application input
// this might include closing windows, keyboard presses, mouse drags
// mapping user settings to keyboard and mouse bindings

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
            let mut camera = &mut state.get_world().get_facets().cameras[0];
            match e {
                InputEvent::KeyUp(_id, keycode) => {
                    match keycode {
                        // TODO: support multiple keypresses
                        17 | 18 | 30 | 31 | 32 | 46 => {
                            camera.movement_dir = None;
                        }
                        _ => {}
                    }
                },
                InputEvent::JoyButtonUp(_src, _id, btn) => {
                    camera.movement_dir = None;
                },
                InputEvent::JoyButtonDown(_src, _id, btn) => {
                    match btn {
                        JoyButton::DPadUp => camera.movement_dir = Some(Direction::Forward),
                        JoyButton::DPadLeft => camera.movement_dir = Some(Direction::Left),
                        JoyButton::DPadDown => camera.movement_dir = Some(Direction::Backward),
                        JoyButton::DPadRight => camera.movement_dir = Some(Direction::Right),
                        _ => {}
                    }
                },
                InputEvent::KeyDown(_id, keycode) => {
                    //
                    // user released 17
                    // user released 30
                    // user released 31
                    // user released 32
                    match keycode {
                        16 => {
                            println!("user pressed 'q' : hard exit.");
                            std::process::exit(0);
                        }
                        18 =>  // e
                            camera.movement_dir = Some(Direction::Up),
                        46 =>  // e
                            camera.movement_dir = Some(Direction::Down),
                        17 =>  // w
                            camera.movement_dir = Some(Direction::Forward),
                        30 => // a
                            camera.movement_dir = Some(Direction::Left),
                        31 =>// s
                            camera.movement_dir = Some(Direction::Backward),
                        32 =>// d
                            camera.movement_dir = Some(Direction::Right),
                        _ => {}
                    }
                },
                InputEvent::MouseMove(_id, _sp, delta) => {
                    let sensitivity = 100.0;
                    let (dx, dy) = (delta.delta_x as f32, delta.delta_y as f32);
                    let xa = dx / sensitivity;
                    let ya = dy / sensitivity;
                    camera.rotate(
                        cgmath::Vector3::new(-ya, -xa, 0.0)
                    );
                },
                evt => {
                    println!("event {:?}", evt);
                }
            }
        }

    }

    {
        let mut camera = &mut state.get_world().get_facets().cameras[0];
        camera.update(dt);
    }
}

#[no_mangle]
pub extern "C" fn mod_input_unload( state: &mut State ) {
    state.on_input_unload();
}
