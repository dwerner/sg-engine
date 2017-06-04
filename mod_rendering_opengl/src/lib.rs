extern crate game_state;

// OpenGL Renderer
#[macro_use]
extern crate glium;

extern crate cgmath;
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
    DrawMode
};

use renderer::opengl::{
    OpenGLRenderer,
};

#[no_mangle]
pub extern "C" fn mod_rendering_opengl_load( state: &mut State ) {
    assert!(state.get_renderers().len() == 0);

    let events_loop = state.get_events_loop().clone();
    let windows = state.get_windows().iter().map(|x| x.clone()).collect::<Vec<Arc<_>>>();

    for w in windows {
        state.add_renderer(
            Box::new(
                OpenGLRenderer::new("title", 640, 480)
            ),
        );
    }

    state.on_render_load();
}

#[no_mangle]
pub extern "C" fn mod_rendering_opengl_tick(state: &mut State) {
    // queue each existing render layers for rendering
    state.push_render_layers();
    state.present_all();
}


#[no_mangle]
pub extern "C" fn mod_rendering_opengl_unload(state: &mut State ) {

    state.on_render_unload();
}
