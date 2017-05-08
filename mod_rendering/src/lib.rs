extern crate game_state;

extern crate cgmath;

// OpenGL Renderer
#[macro_use] extern crate glium;
#[macro_use] extern crate vulkano;
extern crate winit;
extern crate vulkano_win;
extern crate glsl_to_spirv;
extern crate vulkano_shaders;
extern crate time;
extern crate image;

use std::sync::Arc;

use game_state::state;
use game_state::model::{ Model };
use game_state::tree::{ Node };
use game_state::state::{ SceneGraph };

use cgmath::Matrix4;
use cgmath::Vector3;
use cgmath::Rad;

mod renderer;

use renderer::vulkan::{
    VulkanRenderer,
    DrawMode, // TODO: extract this to renderer::
};

#[no_mangle]
pub extern "C" fn mod_rendering_load( s: &mut state::State ) {
    assert!(s.renderers.len() == 0);
    s.renderers.push(
        Box::new(VulkanRenderer::new("Wireframe Renderer (vulkan)", 640, 480, DrawMode::Wireframe )),
    );
    s.renderers.push(
        Box::new(VulkanRenderer::new("Textured Renderer (vulkan)", 640, 480, DrawMode::Colored )),
    );

    for i in 0..s.renderers.len() {
        s.renderers[i].load();
    }

    assert!(s.render_layers.len() == 0);
    let mx =  Matrix4::from_translation(Vector3::new(0.0, -7.0, 0.0)) * Matrix4::from_scale(0.8);
    let root = Node::create( Box::new(Model::create("assets\\models\\pship.obj", mx)), None );
    s.render_layers.push(Arc::new(SceneGraph{root:root}));
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
    s.renderers.clear();
}
