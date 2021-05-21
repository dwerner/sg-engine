use std::time::Duration;

use game_state::sdl2::video::Window;
use game_state::state::{InputAccess, State, VariableAccess, WindowAccess, WorldAccess};
use game_state::thing::{CameraFacet, Direction};

use game_state::sdl2::{
    event::Event as SdlEvent, keyboard::Keycode, mouse::MouseUtil, video::FullscreenType,
};

// this module's purpose is to turn input events into meaningful application input
// this might include closing windows, keyboard presses, mouse drags
// mapping user settings to keyboard and mouse bindings

//
// TODO:
// 1. simultaneous keypresses
// 2. FPS camera rotation, clamp camera angles
//
fn grab_cursor(grab: bool, mouse: &MouseUtil) {
    mouse.show_cursor(!grab);
    mouse.set_relative_mouse_mode(grab);
}

#[no_mangle]
pub extern "C" fn mod_input_load(state: &mut State) {
    state.on_input_load();

    let mouse = state.sdl_context.mouse();
    let mouse_grabbed = state.get_bool("mouse_grabbed").unwrap_or(true);
    grab_cursor(mouse_grabbed, &mouse);
}

#[no_mangle]
pub extern "C" fn mod_input_update(state: &mut State, dt: &Duration) {
    let frame_events = state
        .sdl_subsystems
        .event_pump
        .poll_iter()
        .collect::<Vec<_>>();

    // TODO: wrap unsafe call in State, particularly WindowAccess
    let (sdlwin, _) = state.get_windows()[0].clone();
    let mut window = unsafe { Window::from_ref(sdlwin) };
    //

    let mut paused = state.get_bool("paused").unwrap_or(false);
    let mouse = state.sdl_context.mouse();
    let mut mouse_grabbed = state.get_bool("mouse_grabbed").unwrap_or(true);
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

                        // re-grab the cursor if we are unpausing and it was grabbed
                        if mouse_grabbed {
                            grab_cursor(true, &mouse);
                        }
                    } else {
                        println!("user pressed 'Esc' : paused.");
                        camera.movement_dir = None;
                        paused = true;

                        // un-grab the cursor if we are paused
                        if mouse_grabbed {
                            grab_cursor(false, &mouse);
                        }
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
                Keycode::C if !paused => camera.movement_dir = Some(Direction::Down),
                Keycode::W if !paused => camera.movement_dir = Some(Direction::Forward),
                Keycode::A if !paused => camera.movement_dir = Some(Direction::Left),
                Keycode::S if !paused => camera.movement_dir = Some(Direction::Backward),
                Keycode::D if !paused => camera.movement_dir = Some(Direction::Right),
                Keycode::G if !paused => {
                    mouse_grabbed = !mouse_grabbed;
                    grab_cursor(mouse_grabbed, &mouse);
                }
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
                    // println!( "mod_input: pos {:?} pitch {}, yaw {}", camera.pos, camera.pitch, camera.yaw);
                    let sensitivity = 100.0;
                    let (dx, dy) = (xrel as f32, yrel as f32);
                    let xa = dx / sensitivity;
                    let ya = dy / sensitivity;

                    #[inline(always)]
                    fn _rotate_camera(camera: &mut CameraFacet, xa: f32, ya: f32) {
                        const TWO_PI: f32 = 2.0 * std::f32::consts::PI;
                        const HALF_PI: f32 = 0.5 * std::f32::consts::PI;

                        camera.yaw += xa;

                        // round-robin yaw motion
                        if camera.yaw.abs() > TWO_PI {
                            camera.yaw = 0.0;
                        }

                        camera.pitch += -ya;
                        // Clamp up/down rotation of the camera
                        pub fn clamp(val: f32, min: f32, max: f32) -> f32 {
                            assert!(min <= max);
                            let mut x = val;
                            if x < min {
                                x = min;
                            }
                            if x > max {
                                x = max;
                            }
                            x
                        }
                        camera.pitch = clamp(camera.pitch, -HALF_PI, HALF_PI);
                    }

                    _rotate_camera(camera, xa, ya);
                }
            }
            _ => {}
        }
    }
    camera.update(dt);
    state.set_bool("paused", paused);
    state.set_bool("mouse_grabbed", mouse_grabbed);
}

#[no_mangle]
pub extern "C" fn mod_input_unload(state: &mut State) {
    state.on_input_unload();
}
