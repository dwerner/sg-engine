extern crate game_state;
extern crate cgmath;

use std::sync::Arc;

use game_state::state;
use game_state::model::GVertex;
use game_state::model::{ Model, Mesh };
use game_state::{ Renderer, Renderable, Identifyable };

use cgmath::{
    Matrix,
    Matrix4
};

#[no_mangle]
pub extern "C" fn mod_rendering_load( s: &mut state::State ) {
    assert!(s.renderables.len() == 0);
    s.renderables.push(
        Arc::new(Box::new(Model::create("somefile")))
    );
}

#[no_mangle]
pub extern "C" fn mod_rendering_tick( s: &mut state::State) {
    for i in 0..s.renderers.len() {
        for r in &s.renderables {
            s.renderers[i].queue_renderable(r.clone());
        }
        s.renderers[i].present();
    }
}

#[no_mangle]
pub extern "C" fn mod_rendering_unload( s: &mut state::State ) {
    s.renderables.clear();
}
