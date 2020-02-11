use std::sync::Arc;
use std::time::Duration;

// TODO: switch to nalgebra
use game_state::nalgebra::{Matrix4, Vector3};

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

    let origin = Vector3::new(0.0, 0.0, 0.0);
    let mx = Matrix4::new_translation(&origin) * Matrix4::new_scaling(1.0);

    let helper = Model::load("assets/models/helper-cube.obj", mx)
        .unwrap()
        .pop()
        .unwrap();

    let am = Arc::new(helper);
    state.add_model(am.clone());

    let world = state.get_world();
    // build the actual entity within the world
    let _camera = world
        .start_thing()
        .with_camera(CameraFacet::new(
            Vector3::new(0.0, 0.0, 0.0), // pos
            0.0,                         // pitch
            0.0,                         // yaw
        ))
        .build();

    let _helper_cube = world.start_thing().with_model(mx, am.clone()).build();

    let root = Node::create(None, None);
    let _camera_node = Node::create(None, Some(&root));
    let _helper_node = Node::create(Some(am), Some(&root));

    // NOTE: there's some index-mirroring happening here, we probably want to associate somehow
    // other than this - it's going to be easy to get wrong
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
