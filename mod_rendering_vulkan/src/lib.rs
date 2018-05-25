extern crate game_state;
use game_state::winit;

extern crate cgmath;
#[macro_use] extern crate vulkano;

extern crate glsl_to_spirv;
extern crate vulkano_shaders;
extern crate image;

use game_state::time::Duration;

use std::sync::{
    Arc,
};

mod renderer;

use game_state::state::{
    State,
    RenderAccess,
    WindowAccess,
    DrawMode,
};

use renderer::vulkano::{
    VulkanoRenderer,
};

#[no_mangle]
pub extern "C" fn mod_rendering_vulkan_load( state: &mut State ) {
    //assert!(state.get_renderers().len() == 0);

    let events_loop = state.get_events_loop().clone();
    let windows = state.get_windows().iter().map(|x| x.clone()).collect::<Vec<Arc<_>>>();

    for w in windows {
        let maybe_renderer = VulkanoRenderer::new(w.clone(), events_loop.clone(), DrawMode::Colored);
        match maybe_renderer {
            Ok(mut renderer) => {
                for model in state.models.iter() {
                  renderer.upload_model( model.clone() );
                }
                state.add_renderer(
                    Box::new(renderer)
                );
            }
            Err(err) => println!("aww fuck. {}", err)
        }
    }

    state.on_render_load();
}

#[no_mangle]
pub extern "C" fn mod_rendering_vulkan_update(state: &mut State, dt: &Duration) {
    // queue each existing render layers for rendering
    state.push_render_layers();
    state.present_all();
}


#[no_mangle]
pub extern "C" fn mod_rendering_vulkan_unload(state: &mut State ) {

    state.on_render_unload();
}
