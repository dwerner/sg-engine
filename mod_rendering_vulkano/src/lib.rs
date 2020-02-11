use std::time::Duration;

use game_state::sdl2::video::Window;
use game_state::state::ModelAccess;
use game_state::state::{RenderAccess, State, WindowAccess};

mod renderer;
use renderer::vulkano::VulkanoRenderer;

#[no_mangle]
pub extern "C" fn mod_rendering_vulkano_load(state: &mut State) {
    let windows = state.get_windows().clone();

    for (w, draw_mode) in windows {
        // hack for sdl to own this "window", but pass it's surface to the underlying swapchain
        let win_ptr = {
            let sdlwin = unsafe { Window::from_ref(w) };
            let c = unsafe { &*sdlwin.raw() };
            crate::renderer::vulkano::vulkano_sdl2::WinPtr { raw: c as *const _ }
        };
        let maybe_renderer = VulkanoRenderer::new(win_ptr, draw_mode, state.get_models());

        match maybe_renderer {
            Ok(mut renderer) => {
                for model in state.get_models().iter() {
                    renderer.upload_model(model.clone());
                }
                state.add_renderer(Box::new(renderer));
            }
            Err(err) => println!("Failed to load renderer. {}", err),
        }
    }

    state.on_render_load();
}

#[no_mangle]
pub extern "C" fn mod_rendering_vulkano_update(state: &mut State, _dt: &Duration) {
    // queue each existing render layers for rendering
    state.push_render_layers();
    state.present_all();
}

#[no_mangle]
pub extern "C" fn mod_rendering_vulkano_unload(state: &mut State) {
    state.on_render_unload();
}
