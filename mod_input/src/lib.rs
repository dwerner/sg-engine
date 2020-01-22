use std::time::Duration;

use game_state::input::events::InputEvent;
use game_state::input::events::JoyButton;
use game_state::sdl2::video::Window;
use game_state::state::{InputAccess, State, WindowAccess, WorldAccess};
use game_state::thing::Direction;

/*
 * TODO:
use game_state::{Identifyable, Identity};
use game_state::input::screen::ScreenPoint;
use game_state::input::InputSource;
*/

use game_state::nalgebra::Vector3;
use game_state::sdl2::event::Event as SdlEvent;
use game_state::sdl2::keyboard::Keycode;
use game_state::sdl2::video::FullscreenType;

// this module's purpose is to turn input events into meaningful application input
// this might include closing windows, keyboard presses, mouse drags
// mapping user settings to keyboard and mouse bindings
//
// TODO: VecDeque -> (Event Channel)
//

#[no_mangle]
pub extern "C" fn mod_input_load(state: &mut State) {
    state.on_input_load();
}

#[no_mangle]
pub extern "C" fn mod_input_update(state: &mut State, dt: &Duration) {
    let sdlwin = state.get_windows()[0].clone();
    let mut window = unsafe { Window::from_ref(sdlwin) };

    let frame_events = state
        .sdl_subsystems
        .event_pump
        .poll_iter()
        .collect::<Vec<_>>();

    for event in frame_events {
        let mut camera = &mut state.get_world().get_facets().cameras[0];
        match event {
            SdlEvent::Quit { .. } => {
                println!("quitting...");
                std::process::exit(0);
            }
            SdlEvent::KeyDown {
                keycode: Some(code),
                ..
            } => match code {
                Keycode::Q => {
                    println!("user pressed 'q' : hard exit.");
                    std::process::exit(0);
                }
                Keycode::E => camera.movement_dir = Some(Direction::Up),
                Keycode::C => camera.movement_dir = Some(Direction::Down),
                Keycode::W => camera.movement_dir = Some(Direction::Forward),
                Keycode::A => camera.movement_dir = Some(Direction::Left),
                Keycode::S => camera.movement_dir = Some(Direction::Backward),
                Keycode::D => camera.movement_dir = Some(Direction::Right),
                Keycode::F => match window.fullscreen_state() {
                    FullscreenType::Off => window
                        .set_fullscreen(FullscreenType::Desktop)
                        .expect("unable to set fs"),
                    _ => window
                        .set_fullscreen(FullscreenType::Off)
                        .expect("unable to set fs"),
                },
                _ => {}
            },
            SdlEvent::KeyUp {
                keycode: Some(code),
                ..
            } => {
                match code {
                    // TODO: support multiple keypresses
                    Keycode::E | Keycode::C | Keycode::W | Keycode::A | Keycode::S | Keycode::D => {
                        camera.movement_dir = None;
                    }
                    _ => {}
                }
            }
            SdlEvent::MouseMotion { xrel, yrel, .. } => {
                let sensitivity = 100.0;
                let (dx, dy) = (xrel as f32, yrel as f32);
                let xa = dx / sensitivity;
                let ya = dy / sensitivity;
                camera.rotate(Vector3::new(-ya, -xa, 0.0));
            }
            _ => {}
        }
    }
    if state.has_pending_input_events() {
        let events = state.get_input_events().clone();
        for e in events {
            let mpos = state.get_mouse_pos().clone();
            let mut camera = &mut state.get_world().get_facets().cameras[0];
            match e {
                InputEvent::JoyButtonUp(_src, _id, _btn) => {
                    camera.movement_dir = None;
                }
                InputEvent::JoyButtonDown(_src, _id, btn) => match btn {
                    JoyButton::DPadUp => camera.movement_dir = Some(Direction::Forward),
                    JoyButton::DPadLeft => camera.movement_dir = Some(Direction::Left),
                    JoyButton::DPadDown => camera.movement_dir = Some(Direction::Backward),
                    JoyButton::DPadRight => camera.movement_dir = Some(Direction::Right),
                    _ => {}
                },
                evt => {
                    println!("event {:?}", evt);
                }
            }
        }
    }

    let camera = &mut state.get_world().get_facets().cameras[0];
    camera.update(dt);
    state.clear_input_events();
}

#[no_mangle]
pub extern "C" fn mod_input_unload(state: &mut State) {
    state.on_input_unload();
}
