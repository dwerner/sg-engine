extern crate time;

extern crate game_state;

#[macro_use]
extern crate engine;
use engine::libloader::LibLoader;

use std::time::Duration;
use game_state::state;

use std::thread;

use game_state::state::WindowAccess;

fn main() {
    let mut state = state::State::new();

    state.add_window(640, 800, "sg".to_string());

    // TODO mod_audio
    // TODO mod_gui
    // TODO mod_network

    // because of #[no_mangle], each library needs it's own unique method name as well... sigh

    let mut mods = Vec::new();
    mods.push(load_mod!(simulation));
    mods.push(load_mod!(asset_loader));
    mods.push(load_mod!(rendering_vulkan));
    //mods.push(load_mod!(rendering_opengl));
    mods.push(load_mod!(input));

    for mut m in mods.iter_mut() {
        m.check_update(&mut state);
    }

    let mut frame = 0;
    let frame_budget = 16000i64;// for 60 fps

    loop {
        // TODO: gather delta time instead

        let mut total_time = 0i64;
        for m in mods.iter() {
            let duration: time::Duration = m.tick(&mut state);
            total_time += duration.num_microseconds().unwrap_or(0);
        }


        let wait = (frame_budget - total_time) / 1000;
        if wait > 0 {
            thread::sleep(Duration::from_millis(wait as u64));
        }

        frame += 1;
        if frame % 60 == 0 {
            if frame % 100 == 0 {
                //println!( "frame time: {total_time:<6}μs", total_time=total_time );
            }

            for mut m in mods.iter_mut() {
                m.check_update(&mut state);
            }
        }
    }
}

