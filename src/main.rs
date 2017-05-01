extern crate time;

extern crate game_state;

#[macro_use]
extern crate engine;
use engine::libloader::LibLoader;

use std::time::Duration;
use game_state::state;

use std::thread;

fn main() {
	let mut state = state::State::new();

	// because of #[no_mangle], each library needs it's own unique method name as well... sigh
	let mut sim = load_mod!(simulation);
    let mut assets = load_mod!(asset_loader);
	let mut rendering = load_mod!(rendering);

    assets.check_update(&mut state);
    sim.check_update(&mut state);
    rendering.check_update(&mut state);

	let mut frame = 0;
    let frame_budget = 16000;// for 60 fps

	loop {
        // TODO: gather delta time instead

        let asset_time = assets.tick(&mut state);
		let sim_time = sim.tick(&mut state);
		let render_time = rendering.tick(&mut state);

        let wait = (frame_budget - (
            asset_time.num_microseconds().unwrap() +
                sim_time.num_microseconds().unwrap() +
                render_time.num_microseconds().unwrap()
            )) / 1000;
        if wait > 0 {
            thread::sleep(Duration::from_millis(wait as u64));
        }

        frame += 1;
        if frame % 60 == 0 {
            if frame % 100 == 0 {
                println!(
                    "Asset time: {} Sim time: {}, render time: {}",
                    asset_time.num_microseconds().unwrap(),
                    sim_time.num_microseconds().unwrap(),
                    render_time.num_microseconds().unwrap()
                );
            }
            assets.check_update(&mut state);
            sim.check_update(&mut state);
            rendering.check_update(&mut state);
        }
	}
}

