extern crate game_state;

// OpenGL Renderer
//#[macro_use]
extern crate glium;

extern crate cgmath;
#[macro_use] extern crate vulkano;
extern crate winit;
extern crate vulkano_win;
extern crate glsl_to_spirv;
extern crate vulkano_shaders;
extern crate time;
extern crate image;

mod renderer;

use game_state::state::{
    State,
    RenderAccess,
};

use renderer::vulkan::{
    VulkanRenderer,
    DrawMode, // TODO: extract this to renderer::
};

#[no_mangle]
pub extern "C" fn mod_rendering_load( state: &mut State ) {
    assert!(state.get_renderers().len() == 0);

    state.add_renderer(
        Box::new(VulkanRenderer::new("Wireframe Renderer (vulkan)", 640, 480, DrawMode::Wireframe )),
    );
    state.add_renderer(
        Box::new(VulkanRenderer::new("Textured Renderer (vulkan)", 640, 480, DrawMode::Colored )),
    );

    state.on_render_load();
}

#[no_mangle]
pub extern "C" fn mod_rendering_tick(state: &mut State) {
    // queue each existing render layers for rendering
    state.push_render_layers();
    state.present_all();
}


#[no_mangle]
pub extern "C" fn mod_rendering_unload(state: &mut State ) {
    state.on_render_unload();
}
