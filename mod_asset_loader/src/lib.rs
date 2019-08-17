use std::sync::Arc;
use std::time::Duration;

// TODO: switch to nalgebra
use cgmath::Matrix4;
use cgmath::Vector3;

use game_state::model::Model;
use game_state::state::ModelAccess;
use game_state::state::RenderLayerAccess;
use game_state::state::SceneGraph;
use game_state::state::State;
use game_state::state::WorldAccess;
use game_state::thing::CameraFacet;
use game_state::tree::Node;

#[no_mangle]
pub extern "C" fn mod_asset_loader_load(state: &mut State) {
    assert!(state.get_render_layers().is_empty());

    let mx = Matrix4::from_translation(Vector3::new(0.0, 0.0, 0.0)) * Matrix4::from_scale(1.0);

    // Conceptually here, we are loading one model, but we might instance it from many entities
    let model = Model::create("assets/models/pship.obj", mx);
    let thing2 = Model::create("assets/models/textured_thing.obj", mx);
    let am = Arc::new(model);

    // state.models is rendering state - we upload these to the GPU when appropriate
    // models are only ever added once
    state.add_model(am.clone());
    state.add_model(Arc::new(thing2));

    {
        let world = state.get_world();
        // build the actual entity
        let _thing = world
            .start_thing()
            .with_camera(CameraFacet::new(
                cgmath::Vector3::new(0.0, 0.0, 0.0), // pos
                cgmath::Vector3::new(0.0, 1.0, 0.0), // rotation
            ))
            .with_model(mx, am)
            .build();
    }

    let root = Node::create(0, None);
    let _child = Node::create(1, Some(root.clone()));

    state.add_render_layer(Arc::new(SceneGraph { root }));
}

#[no_mangle]
pub extern "C" fn mod_asset_loader_update(_state: &mut State, _dt: &Duration) {
    //
    // this module might look for unused assets, or requests for loading new ones?
    // for instance, instead of blindly loading an asset and pushing it into state, we COULD be loading files
    // in a multithreaded context, pushing them in on this thread when we are ticked
    //
}

#[no_mangle]
pub extern "C" fn mod_asset_loader_unload(state: &mut State) {
    state.clear_render_layers();
}
