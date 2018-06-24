extern crate game_state;
use game_state::winit;

extern crate cgmath;
#[macro_use] extern crate vulkano;

extern crate glsl_to_spirv;
extern crate vulkano_shaders;
extern crate image;

use game_state::time::Duration;
use game_state::state::ModelAccess;

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
pub extern "C" fn mod_rendering_vulkano_load( state: &mut State ) {

    let windows = state.get_windows().clone();
    for w in windows {
        let maybe_renderer =
            VulkanoRenderer::new(
                w.get_window().clone(),
                w.get_event_loop().clone(),
                DrawMode::Colored
            );

        match maybe_renderer {
            Ok(mut renderer) => {
                for model in state.get_models().iter() {
                    renderer.upload_model( model.clone() );
                }
                state.add_renderer( Box::new(renderer) );
            }
            Err(err) => println!("Failed to load renderer. {}", err)
        }
    }

    state.on_render_load();
}

#[no_mangle]
pub extern "C" fn mod_rendering_vulkano_update(state: &mut State, dt: &Duration) {
    // queue each existing render layers for rendering
    state.push_render_layers();
    state.present_all();
}


#[no_mangle]
pub extern "C" fn mod_rendering_vulkano_unload(state: &mut State ) {

    state.on_render_unload();
}
