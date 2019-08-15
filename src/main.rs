#[macro_use]
extern crate engine;
use engine::libloader::LibLoader;

use game_state::state;
use std::time::{Duration, Instant};

use std::thread;

use game_state::state::WindowAccess;

fn main() {
    let mut state = state::State::new();

    // TODO mod_audio
    // TODO mod_gui
    // TODO mod_network

    // because of #[no_mangle], each library needs it's own unique method name as well... sigh

    let mut mods = Vec::new();
    mods.push(load_mod!(gamepad));
    mods.push(load_mod!(asset_loader));
    mods.push(load_mod!(simulation));

    state.add_window(800, 600, "sg-shell 1 (vulkano)".to_string());
    //state.add_window(800, 600, "sg-shell 2 (voodoo)".to_string());
    mods.push(load_mod!(rendering_vulkano));

    //mods.push(load_mod!(rendering_voodoo));

    //state.add_window_builder(800, 600, "sg-shell (OpenGL/glutin)".to_string());

    // For now this is incompatible
    //mods.push(load_mod!(rendering_opengl));

    mods.push(load_mod!(input));

    for m in mods.iter_mut() {
        m.check_update(&mut state);
    }

    let mut frame = 0;
    let frame_budget = 16000u128; // for 60 fps
    let mut last_update = Instant::now();

    loop {
        // TODO: gather delta time instead

        let mut total_time = 0;
        for m in mods.iter() {
            let start_update = Instant::now();
            let duration = m.update(&mut state, &(start_update - last_update));
            if frame % 300 == 0 {
                print!(
                    "|> {}: {total_time:>6} μs ",
                    name = m.get_name(),
                    total_time = duration.as_micros()
                );
            }
            total_time += duration.as_micros();
        }
        last_update = Instant::now();
        if frame % 300 == 0 {
            println!(
                "|>= total time: {total_time:>6} μs",
                total_time = total_time
            );
        }
        if frame % 30 == 0 {
            for m in mods.iter_mut() {
                m.check_update(&mut state);
            }
        }
        frame += 1;

        let wait = (frame_budget - total_time) / 1000;
        if wait > 0 {
            thread::sleep(Duration::from_millis(wait as u64));
        }
    }
}
