extern crate game_state;

// OpenGL Renderer
extern crate gl;
extern crate glutin;

extern crate cgmath;
extern crate image;

use game_state::time;
use game_state::time::Duration;

use std::sync::{
    Arc,
    Mutex
};

mod renderer;

use game_state::state::{
    State,
    RenderAccess,
    WindowAccess,
    DrawMode
};

use renderer::opengl::{
    OpenGLRenderer,
};

#[no_mangle]
pub extern "C" fn mod_rendering_opengl_load( state: &mut State ) {
    //assert!(state.get_renderers().len() == 0);

    let events_loop = state.get_events_loop().clone();
    let windows = state.get_window_builders().iter().map(|x| x.clone()).collect::<Vec<_>>();

    for w in windows {
        state.add_renderer(
            Box::new(
                OpenGLRenderer::new(w, events_loop.clone())
            ),
        );
    }

    state.on_render_load();
}

#[no_mangle]
pub extern "C" fn mod_rendering_opengl_update(state: &mut State, dt: &Duration) {
    // queue each existing render layers for rendering
    state.push_render_layers();
    state.present_all();
}


#[no_mangle]
pub extern "C" fn mod_rendering_opengl_unload(state: &mut State ) {

    state.on_render_unload();
}
