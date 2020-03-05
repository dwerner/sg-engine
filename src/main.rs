#[macro_use]
extern crate engine;
use engine::libloader::LibLoader;

use game_state::state::DrawMode;
use game_state::state::State;
use std::time::{Duration, Instant};

use std::thread;

use game_state::state::WindowAccess;

fn main() {
    let mut state = State::default();

    // TODO mod_audio
    // TODO mod_gui
    // TODO mod_network

    state.add_window(
        800,
        600,
        "sg-shell 1 (vulkano) wireframe",
        0,
        720,
        DrawMode::Wireframe(3.0),
    );
    state.add_window(
        1280,
        720,
        "sg-shell 1 (vulkano) textured",
        0,
        0,
        DrawMode::Textured,
    );

    let mut mods = Vec::new();
    mods.push(load_mod!(gamepad));
    mods.push(load_mod!(asset_loader));
    mods.push(load_mod!(simulation));

    mods.push(load_mod!(rendering_vulkano));
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

        let wait = if frame_budget >= total_time {
            (frame_budget - total_time) / 1000
        } else {
            0
        };

        if wait > 0 {
            thread::sleep(Duration::from_millis(wait as u64));
        }
    }
}
