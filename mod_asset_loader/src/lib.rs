extern crate game_state;
extern crate cgmath;

use game_state::state::State;
use game_state::state::RenderLayerAccess;

use game_state::model::{ Model };
use game_state::Identifyable;
use game_state::tree::{ Node };
use game_state::state::{ SceneGraph };

use game_state::time::Duration;

use game_state::thing::Thing;
use game_state::thing::ThingBuilder;
use game_state::thing::CameraFacet;

use game_state::state::ModelAccess;
use game_state::state::WorldAccess;

use std::sync::{
    Arc,
    Mutex,
};

use cgmath::Matrix4;
use cgmath::Vector3;

#[no_mangle]
pub extern "C" fn mod_asset_loader_load( state: &mut State ) {
    assert!(state.get_render_layers().len() == 0);

    let mx = Matrix4::from_translation(Vector3::new(0.0, 0.0, 0.0)) * Matrix4::from_scale(1.5);

    // Conceptually here, we are loading one model, but we might instance it from many entities
    let model = Model::create("assets/models/pship.obj", mx);
    let thing2 = Model::create("assets/models/textured_thing.obj", mx);
    let am = Arc::new(model);

    // state.models is rendering state - we upload these to the GPU when appropriate
    state.add_model(am.clone());
    state.add_model(Arc::new(thing2));

    // build the actual entity
    let thing = ThingBuilder::start()
        .with_camera(
            CameraFacet::new(
                cgmath::Matrix4::look_at(
                    cgmath::Point3::new(0.0, 0.0, -20.0),   // eye
                    cgmath::Point3::new(0.0, 0.0, 0.0),  // center
                    cgmath::Vector3::new(0.0, -1.0, 0.0)  // up
                ),
                Matrix4::from_translation( Vector3::new(0.0, 0.0, 0.0) ) * Matrix4::from_scale(1.0)
            )
        )
        .with_model(mx, am)
        .build();

    let thing = Arc::new(Mutex::new(thing));

    {
        let mut world = state.get_world();
        world.add_thing(thing.clone());
    }

    // previously we just agreed on an index, but is there a better way to relate 
    // Model + Material ==> DescriptorSet and CpuAccessibleBuffer?
    let root = Node::create(0, None );
    let child = Node::create( 1, Some(root.clone()) );

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
