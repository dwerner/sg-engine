extern crate game_state;
extern crate cgmath;

use game_state::state::State;
use game_state::Renderable;
use game_state::model::{ Model };
use game_state::tree::{ Node };
use game_state::state::{ SceneGraph };

use std::sync::Arc;

use cgmath::Matrix4;
use cgmath::Vector3;
use cgmath::Rad;

#[no_mangle]
pub extern "C" fn mod_asset_loader_load( state: &mut State ) {
    assert!(state.render_layers.len() == 0);
    let mx =  Matrix4::from_translation(Vector3::new(0.0, -7.0, 0.0)) * Matrix4::from_scale(0.8);
    let ship = Box::new(Model::create("assets\\models\\pship.obj", mx));
    let root = Node::create(ship as Box<Renderable>, None );
    state.render_layers.push(Arc::new(SceneGraph{root:root}));
}

#[no_mangle]
pub extern "C" fn mod_asset_loader_tick( _state: &mut State ) {
    //
    // this module might look for unused assets, or requests for loading new ones?
    // for instance, instead of blindly loading an asset and pushing it into state, we COULD be loading files
    // in a multithreaded context, pushing them in on this thread when we are ticked
    //
}

#[no_mangle]
pub extern "C" fn mod_asset_loader_unload( state: &mut State ) {
    state.render_layers.clear();
}
