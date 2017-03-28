extern crate game_state;
extern crate cgmath;

use std::sync::Arc;

use game_state::state;
use game_state::model::GVertex;
use game_state::model::{ Model, Mesh };
use game_state::tree::{RcNode, Node};
use game_state::{ SceneGraph, Renderer, Renderable, Identifyable };

use cgmath::{
    Matrix,
    Matrix4
};

#[no_mangle]
pub extern "C" fn mod_rendering_load( s: &mut state::State ) {
    assert!(s.render_layers.len() == 0);
    s.render_layers.push({
        let graph = SceneGraph { root: Node::create(Model::create("somefile"), None) };
        Arc::new(graph)
    });
}

#[no_mangle]
pub extern "C" fn mod_rendering_tick(s: &mut state::State) {
    for i in 0..s.renderers.len() {
        for r in &s.render_layers {
            s.renderers[i].queue_render_layer(r.clone());
        }
        s.renderers[i].present();
    }
}

#[no_mangle]
pub extern "C" fn mod_rendering_unload(s: &mut state::State ) {
    s.render_layers.clear();
}
