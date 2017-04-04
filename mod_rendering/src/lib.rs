extern crate game_state;
extern crate cgmath;

use std::sync::Arc;

use game_state::state;
use game_state::model::{ Model };
use game_state::tree::{ Node };
use game_state::state::{ SceneGraph };

use cgmath::Matrix4;
use cgmath::Vector3;
use cgmath::Rad;


#[no_mangle]
pub extern "C" fn mod_rendering_load( s: &mut state::State ) {
    assert!(s.render_layers.len() == 0);
    s.render_layers.push(create_tentacle(-5.0, 0.0, 2.0, 4));
    s.render_layers.push(create_tentacle(-2.5, 0.0, 1.0, 4));
    s.render_layers.push(create_tentacle(0.0, 0.0, 0.0, 40));
    s.render_layers.push(create_tentacle(2.5, 0.0, 1.0, 50));
    s.render_layers.push(create_tentacle(5.0, 0.0, 2.0, 50));

    for i in 0..s.renderers.len() {
        s.renderers[i].load();
    }
}

fn create_tentacle(x:f32, y:f32, z:f32, count:u32) -> Arc<SceneGraph> {
    let mx = Matrix4::from_translation(Vector3::new(x, y, z)) * Matrix4::from_scale(1.0);
    let root = Node::create( Box::new(Model::create("otherfile",mx)), None );
    let mx = Matrix4::from_translation(Vector3::new(0.0, 0.7, -0.7)) * Matrix4::from_scale(1.0);
    let mut child = Node::create( Box::new(Model::create("otherfile",mx)), Some(root.clone()) );
    for i in 0..count {
        let mx = Matrix4::from_translation(Vector3::new(0.7, 0.7, -0.7)) * Matrix4::from_scale(1.0);
        child = Node::create( Box::new( Model::create("otherfile", mx )), Some(child.clone()) );
    }
    let mx = Matrix4::from_translation(Vector3::new(0.0, 0.7, -0.7)) * Matrix4::from_scale(1.0);
    let _child = Node::create( Box::new( Model::create("otherfile", mx )), Some(child.clone()) );
    let graph = SceneGraph { root: root };
    Arc::new(graph)
}

#[no_mangle]
pub extern "C" fn mod_rendering_tick(s: &mut state::State) {
    // queue each existing render layers for rendering
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
    for i in 0..s.renderers.len() {
        s.renderers[i].unload();
    }
}
