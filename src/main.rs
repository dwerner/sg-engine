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

    state.add_window(640, 480, "Main Window".to_string());

    // TODO mod_audio
    // TODO mod_gui
    // TODO mod_network

	// because of #[no_mangle], each library needs it's own unique method name as well... sigh
	let mut sim = load_mod!(simulation);
    let mut assets = load_mod!(asset_loader);
	let mut rendering = load_mod!(rendering);
    let mut input = load_mod!(input);

    assets.check_update(&mut state);
    sim.check_update(&mut state);
    rendering.check_update(&mut state);
    input.check_update(&mut state);

	let mut frame = 0;
    let frame_budget = 16000;// for 60 fps

	loop {
        // TODO: gather delta time instead

        let asset_time = assets.tick(&mut state);
		let sim_time = sim.tick(&mut state);
		let render_time = rendering.tick(&mut state);
        let input_time = input.tick(&mut state);

        let wait = (frame_budget - (
            asset_time.num_microseconds().unwrap() +
                sim_time.num_microseconds().unwrap() +
                input_time.num_microseconds().unwrap() +
                render_time.num_microseconds().unwrap()
            )) / 1000;
        if wait > 0 {
            thread::sleep(Duration::from_millis(wait as u64));
        }

        frame += 1;
        if frame % 60 == 0 {
            if frame % 100 == 0 {
                println!(
                    "asset_loader: {asset:<6}μs, simulation: {sim:<6}μs, rendering: {render:<6}μs, input: {input:<6}μs",
                    asset = asset_time.num_microseconds().unwrap(),
                    sim = sim_time.num_microseconds().unwrap(),
                    render = render_time.num_microseconds().unwrap(),
                    input = input_time.num_microseconds().unwrap()
                );
            }
            assets.check_update(&mut state);
            sim.check_update(&mut state);
            rendering.check_update(&mut state);
            input.check_update(&mut state);
        }
	}
}

