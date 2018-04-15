use glium;
use glium::{
    glutin,
    Surface,
};

use game_state;
use game_state::state::SceneGraph;
use game_state::Renderable;
use game_state::Renderer;

use game_state::input;

use std::sync::Arc;
use game_state::Identity;
use std::collections::VecDeque;

pub struct OpenGLRenderer {
    id: Identity,
    display: glium::Display,
    w:u32,
    h:u32,
    title: String,
    render_layer_queue: VecDeque<Arc<SceneGraph>>
}
impl OpenGLRenderer {

    pub fn new(title: &str, w: u32, h: u32) -> Self {
        let events_loop = glutin::EventsLoop::new();
        let window = glutin::WindowBuilder::new()
            .with_title(title)
            .with_dimensions(w,h);
        let context = glutin::ContextBuilder::new();
        let display = glium::Display::new(window, context, &events_loop)
            .unwrap();

        OpenGLRenderer{
            id: game_state::create_next_identity(),
            display: display,
            w: w,
            h: h,
            title: title.to_string(),
            render_layer_queue: VecDeque::new()
        }
    }
    fn render(&mut self){}
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
    fn load(&mut self) {
        println!("Opengl renderer load");
    }

    fn unload(&mut self) {
        println!("Opengl renderer unload")
    }

    fn queue_render_layer(&mut self, layer: Arc<SceneGraph>) {
        self.render_layer_queue.push_back(layer);
    }

    fn present(&mut self) {
        self.render();
    }

    fn set_title(&mut self, title: &str) {
    }
}