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

    for i in 0..s.renderers.len() {
        s.renderers[i].unload();
    }
    s.renderers.clear();
}
