use gl;
use gl::types::*;

use glutin;
use glutin::GlContext;

use game_state;
use game_state::state::SceneGraph;
use game_state::winit;
use game_state::Renderer;

use game_state::input;

use std::sync::{Arc, Mutex};

use game_state::thing::CameraFacet;
use game_state::Identity;

use std::collections::VecDeque;

pub struct OpenGLRenderer {
    id: Identity,
    gl_window: glutin::GlWindow,
    el: glutin::EventsLoop,
    render_layer_queue: VecDeque<Arc<SceneGraph>>,
}
impl OpenGLRenderer {
    pub fn new(
        window: winit::WindowBuilder,
        events_loop: Arc<Mutex<Option<winit::event::EventsLoop<()>>>>,
    ) -> Self {
        let context = glutin::ContextBuilder::new();
        let el = glutin::EventsLoop::new();

        let gl_window = glutin::GlWindow::new(window.clone(), context, &el).unwrap();

        unsafe {
            gl_window.make_current().unwrap();
        }

        unsafe {
            gl::load_with(|symbol| gl_window.get_proc_address(symbol) as *const _);
            gl::ClearColor(0.0, 0.1, 0.0, 1.0);
        }

        OpenGLRenderer {
            id: game_state::create_next_identity(),
            gl_window: gl_window,
            el,
            render_layer_queue: VecDeque::new(),
        }
    }

    fn render(&mut self, camera: &CameraFacet) {
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        loop {
            match self.render_layer_queue.pop_front() {
                Some(layer) => {

                    //TODO : gather needed resources from the layer and render

                }
                None => break,
            }
        }

        self.gl_window
            .swap_buffers()
            .expect("failed to swap buffers");
    }
}

impl game_state::Identifyable for OpenGLRenderer {
    fn identify(&self) -> Identity {
        self.id
    }
}

impl game_state::input::InputSource for OpenGLRenderer {
    fn get_input_events(&mut self) -> VecDeque<input::events::InputEvent> {
        VecDeque::new()
    }
}

impl Renderer for OpenGLRenderer {
    fn load(&mut self) {}

    fn unload(&mut self) {}

    fn queue_render_layer(&mut self, layer: Arc<SceneGraph>) {
        self.render_layer_queue.push_back(layer);
    }

    fn present(&mut self, camera: &CameraFacet) {
        self.render(camera);
    }
}
