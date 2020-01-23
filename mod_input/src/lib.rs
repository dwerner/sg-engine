use std::time::Duration;

use game_state::input::events::InputEvent;
use game_state::input::events::JoyButton;
use game_state::sdl2::video::Window;
use game_state::state::{InputAccess, State, VariableAccess, WindowAccess, WorldAccess};
use game_state::thing::Direction;

/*
 * TODO:
 * 1. simultaneous keypresses
 * 2. FPS camera rotation, clamp camera angles
*/

use game_state::nalgebra::{Matrix4, Vector3};
use game_state::sdl2::event::Event as SdlEvent;
use game_state::sdl2::keyboard::Keycode;
use game_state::sdl2::video::FullscreenType;

use game_state::state::Variable;

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

    let mut paused = state.get_bool("paused").unwrap_or(false);

    let frame_events = state
        .sdl_subsystems
        .event_pump
        .poll_iter()
        .collect::<Vec<_>>();

    let mut camera = &mut state.get_world().get_facets().cameras[0];
    for event in frame_events {
        match event {
            SdlEvent::Quit { .. } => {
                println!("quitting...");
                std::process::exit(0);
            }
            SdlEvent::KeyDown {
                keycode: Some(code),
                ..
            } => match code {
                Keycode::Escape => {
                    if paused {
                        println!("user pressed 'Esc' : unpaused.");
                        paused = false;
                    } else {
                        println!("user pressed 'Esc' : paused.");
                        camera.movement_dir = None;
                        paused = true;
                    }
                }

                Keycode::Q => {
                    if paused {
                        println!("user pressed 'q' while paused : hard exit.");
                        std::process::exit(0);
                    }
                }

                //
                // TODO: pausing should prevent changes to the world, rather than guard input
                //
                Keycode::E if !paused => camera.movement_dir = Some(Direction::Up),
                Keycode::W if !paused => camera.movement_dir = Some(Direction::Forward),
                Keycode::A if !paused => camera.movement_dir = Some(Direction::Left),
                Keycode::S if !paused => camera.movement_dir = Some(Direction::Backward),
                Keycode::D if !paused => camera.movement_dir = Some(Direction::Right),
                Keycode::F => match window.fullscreen_state() {
                    FullscreenType::Off => window
                        .set_fullscreen(FullscreenType::Desktop)
                        .expect("unable to set fs"),
                    _ => window
                        .set_fullscreen(FullscreenType::Off)
                        .expect("unable to set fs"),
                },

                Keycode::Num9 => {
                    println!("{}", camera.perspective.fovy());
                }
                Keycode::Num0 => camera.perspective.set_fovy(camera.perspective.fovy() - 1.0),

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
                if !paused {
                    println!("TODO: Clamping and pitch rotation {:?}", camera.rotation);
                    let sensitivity = 100.0;
                    let (dx, dy) = (xrel as f32, yrel as f32);
                    let xa = dx / sensitivity;
                    let ya = dy / sensitivity;

                    camera.rotation += Vector3::new(0.0, xa, 0.0);
                    let rot = Matrix4::new_rotation(camera.rotation);
                    let trans = Matrix4::new_translation(&-camera.pos);

                    camera.view = trans * rot;
                    camera.update_view_matrix();
                }
            }
            _ => {}
        }
    }
    camera.update(dt);
    state.set_bool("paused", paused);
}

#[no_mangle]
pub extern "C" fn mod_input_unload(state: &mut State) {
    state.on_input_unload();
}
