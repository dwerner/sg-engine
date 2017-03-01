extern crate winit;
extern crate vulkano;
extern crate time;
extern crate cgmath;

extern crate engine;
use engine::renderer::{Vertex, VulkanRenderer};
use engine::libloader::LibLoader;

use cgmath::Vector3;

extern crate game_state;
use std::time::Duration;
use game_state::{state, Renderable, Renderer};

use std::thread;

// Platform-specific wiring for simulation and simulation2 dynamically loaded libs (hot loaded)
#[cfg(target_os = "windows")] const SIM_LIBPATH: &'static str = "mod_simulation/target/debug/mod_simulation.dll";
#[cfg(target_os = "linux")] const SIM_LIBPATH: &'static str = "mod_simulation/target/debug/deps/libmod_simulation.so";

#[cfg(target_os = "windows")] const RENDERING_LIBPATH: &'static str = "mod_rendering/target/debug/mod_rendering.dll";
#[cfg(target_os = "linux")] const RENDERING_LIBPATH: &'static str = "mod_rendering/target/debug/libmod_rendering.so";

struct DummyRenderer { }

impl Renderer for DummyRenderer {
	fn draw(&mut self, renderables: &Vec<Box<Renderable>>) {
		for ref renderable in renderables {
			println!("DummyRenderer - Rendering something.. I swear.");
		}
	}
}

fn main() {
    // TODO: merge these two loops
	let mut state = state::State {
		renderers: vec![
            Box::new(VulkanRenderer::new("title", 320, 240)),
            Box::new(VulkanRenderer::new("another title", 320, 240)),
            Box::new(VulkanRenderer::new("title2", 320, 240)),
        ],
		renderables: Vec::new(),
		blob: 42,
	};

	// because of #[no_mangle], each library needs it's own unique method name as well... sigh
	let mut sim = LibLoader::new(SIM_LIBPATH, "simulation");
	let mut rendering = LibLoader::new(RENDERING_LIBPATH, "rendering");
    sim.check_update(&mut state);
    rendering.check_update(&mut state);

	let mut frame = 0;
	loop {
        // TODO: gather delta time instead
		thread::sleep(Duration::from_millis(16));


        let start = time::PreciseTime::now();
		sim.tick(&mut state);

        let sim_time = start.to(time::PreciseTime::now());

		rendering.tick(&mut state);

        let rendering_time = start.to(time::PreciseTime::now());

        frame += 1;
        if frame % 60 == 0 {
            println!(
                "Sim time: {}, render time: {}",
                sim_time.num_microseconds().unwrap(),
                rendering_time.num_microseconds().unwrap()
            );
            sim.check_update(&mut state);
            rendering.check_update(&mut state);
        }
	}
}

