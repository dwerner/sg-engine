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

use std::sync::{
    Arc,
    Mutex
};

mod renderer;

use game_state::state::{
    State,
    RenderAccess,
    WindowAccess,
};

use renderer::vulkan::{
    VulkanRenderer,
    DrawMode, // TODO: extract this to renderer::
};

#[no_mangle]
pub extern "C" fn mod_rendering_load( state: &mut State ) {
    assert!(state.get_renderers().len() == 0);

    let events_loop = state.get_events_loop().clone();
    let windows = state.get_windows().iter().map(|x| x.clone()).collect::<Vec<Arc<_>>>();

    for w in windows {
        println!("Adding renderer for window provided.");
        state.add_renderer(
            Box::new(
                VulkanRenderer::new((w, events_loop.clone()), DrawMode::Colored)
            ),
        );
    }

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
