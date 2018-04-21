extern crate game_state;
extern crate cgmath;

use game_state::state::State;
use game_state::Renderable;
use game_state::state::RenderLayerAccess;

use game_state::model::{ Model };
use game_state::tree::{ Node };
use game_state::state::{ SceneGraph };

use game_state::time::Duration;

use std::sync::Arc;

use cgmath::Matrix4;
use cgmath::Vector3;

#[no_mangle]
pub extern "C" fn mod_asset_loader_load( state: &mut State ) {
    assert!(state.get_render_layers().len() == 0);

    let mx = Matrix4::from_translation(Vector3::new(0.0, -20.0, 1.0)) * Matrix4::from_scale(1.5);
    let mut model = Model::create("assets/models/textured_thing.obj", mx);
    let blob = Box::new(model.clone());
    let root =  Node::create(blob as Box<Renderable>, None );

    let mx = Matrix4::from_translation(Vector3::new(0.0, 2.0, 1.0)) * Matrix4::from_scale(0.75);
    model.set_model_matrix(mx);
    let blob = Box::new(model.clone());
    let mut child = Node::create(blob as Box<Renderable>, Some(root.clone()) );

    state.add_render_layer(Arc::new(SceneGraph{root:root}));
}

#[no_mangle]
pub extern "C" fn mod_asset_loader_update( _state: &mut State, dt: &Duration ) {
    //
    // this module might look for unused assets, or requests for loading new ones?
    // for instance, instead of blindly loading an asset and pushing it into state, we COULD be loading files
    // in a multithreaded context, pushing them in on this thread when we are ticked
    //
}

#[no_mangle]
pub extern "C" fn mod_asset_loader_unload( state: &mut State ) {
    state.clear_render_layers();
}
