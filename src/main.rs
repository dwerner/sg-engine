extern crate winit;
extern crate vulkano;
extern crate time;
extern crate cgmath;

#[macro_use]
extern crate engine;

use engine::renderer::{Vertex, VulkanRenderer};
use engine::libloader::LibLoader;

use cgmath::Vector3;

extern crate game_state;
use std::time::Duration;
use game_state::{state, Renderable, Renderer};

use std::thread;

struct DummyRenderer { }

impl Renderer for DummyRenderer {
	fn draw(&mut self, renderables: &Vec<Box<Renderable>>) {
		for ref renderable in renderables {
			println!("DummyRenderer - Rendering something.. I swear.");
		}
	}
}

fn main() {
	let mut state = state::State {
		renderers: vec![
            Box::new(VulkanRenderer::new("title", 320, 240)),
            Box::new(VulkanRenderer::new("title2", 320, 240)),
        ],
		renderables: Vec::new(),
		blob: 42,
	};

	// because of #[no_mangle], each library needs it's own unique method name as well... sigh
	let mut sim = load_mod!(simulation);
	let mut rendering = load_mod!(rendering);

    sim.check_update(&mut state);
    rendering.check_update(&mut state);

	let mut frame = 0;
	loop {
        // TODO: gather delta time instead
		thread::sleep(Duration::from_millis(16));

		let sim_time = sim.tick(&mut state);
		let render_time = rendering.tick(&mut state);

        frame += 1;
        if frame % 60 == 0 {
            println!(
                "Sim time: {}, render time: {}",
                sim_time.num_microseconds().unwrap(),
                render_time.num_microseconds().unwrap()
            );
            sim.check_update(&mut state);
            rendering.check_update(&mut state);
        }
	}
}

