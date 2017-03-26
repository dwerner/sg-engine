extern crate game_state;
extern crate cgmath;

use game_state::state;
use game_state::model::GVertex;
use game_state::{ Model, Mesh };
use game_state::{ Renderer, Renderable, Identifyable };

use cgmath::{
    Matrix,
    Matrix4
};

#[no_mangle]
pub extern "C" fn mod_rendering_load( s: &mut state::State ) {
    assert!(s.renderables.len() == 0);
    s.renderables.push(
        Box::new(Model::create("somefile"))
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
