extern crate winit;
extern crate vulkano;
extern crate time;
extern crate cgmath;

#[macro_use]
extern crate engine;

use engine::renderer::{
    OpenGLRenderer,
};

use engine::renderer::vulkan::VulkanRenderer;

use engine::libloader::LibLoader;

extern crate game_state;
use std::time::Duration;
use game_state::state;

use std::thread;

fn main() {
	let mut state = state::State::new(
		vec![
            Box::new(VulkanRenderer::new("VulkanRenderer", 1920, 1080)),
        ]
    );

	// because of #[no_mangle], each library needs it's own unique method name as well... sigh
	let mut sim = load_mod!(simulation);
	let mut rendering = load_mod!(rendering);

    sim.check_update(&mut state);
    rendering.check_update(&mut state);

	let mut frame = 0;
    let frame_budget = 16000;// for 60 fps

	loop {
        // TODO: gather delta time instead

		let sim_time = sim.tick(&mut state);
		let render_time = rendering.tick(&mut state);

        let wait = (frame_budget - (sim_time.num_microseconds().unwrap() + render_time.num_microseconds().unwrap())) / 1000;
        if wait > 0 {
            thread::sleep(Duration::from_millis(wait as u64));
        }

        frame += 1;
        if frame % 60 == 0 {
            if frame % 100 == 0 {
                println!(
                    "Sim time: {}, render time: {}",
                    sim_time.num_microseconds().unwrap(),
                    render_time.num_microseconds().unwrap()
                );
            }
            sim.check_update(&mut state);
            rendering.check_update(&mut state);
        }
	}
}

