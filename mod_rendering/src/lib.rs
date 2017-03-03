extern crate game_state;

use game_state::state;
use game_state::ColoredVertex;
use game_state::{ Renderer, Renderable, Physical, Syncable, Identifyable };


struct Blobject {
    id: u64,
    x: u32,
    y: String
}

impl Renderable for Blobject {
    fn get_geometry(&self) -> Vec<ColoredVertex> {
        let red  = [1.0, 0.0, 0.0, 1.0];
        let green = [0.0, 1.0, 0.0, 1.0];
        let blue = [0.0, 0.0, 1.0, 1.0];
        let items = vec![

            ColoredVertex::new([0.5, 0.25, 0.0], red),
            ColoredVertex::new([0.0, -0.5, 0.0], blue),
            ColoredVertex::new([-0.25, 0.1, 0.0], green),

        ];
        items
    }
}

impl Identifyable for Blobject {
    fn identify(&self) -> u64 { self.id }
}

#[no_mangle]
pub extern "C" fn mod_rendering_load( s: &mut state::State ) {
    assert!(s.renderables.len() == 0);
    s.renderables.push(
        Box::new(
            Blobject{id: s.blob, x: s.blob as u32, y: ":)".to_string()
        })
    );
}

#[no_mangle]
pub extern "C" fn mod_rendering_tick( s: &mut state::State) {
    // to avoid borrowing from s.renderers...
    for i in 0..s.renderers.len() {
        s.renderers[i].draw(&s.renderables);
    }
    /*
    for ref renderer in s.renderers {
        renderer.draw(&s.renderables)
    }
    */
}

#[no_mangle]
pub extern "C" fn mod_rendering_unload( s: &mut state::State ) {
    s.blob = 0;
    s.renderables.clear();
}
