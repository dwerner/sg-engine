use glium::{
    DisplayBuild,
    Surface,
};
use glium::backend::glutin_backend::GlutinFacade;
use glium::glutin;

extern crate game_state;

use game_state::state::SceneGraph;
use game_state::Renderable;
use game_state::Renderer;

use std::sync::Arc;
use std::collections::VecDeque;

pub struct OpenGLRenderer {
    display: GlutinFacade,
    w:u32,
    h:u32,
    title: String,
    render_layer_queue: VecDeque<Arc<SceneGraph>>
}
impl OpenGLRenderer {

    pub fn new(title: &str, w: u32, h: u32) -> Self {
        let display = glutin::WindowBuilder::new()
            .with_vsync()
            .with_title(title)
            .with_dimensions(w,h)
            .build_glium()
            .unwrap();

        OpenGLRenderer{
            display: display,
            w: w,
            h: h,
            title: title.to_string(),
            render_layer_queue: VecDeque::new()
        }
    }
    fn render(&mut self){}
}

impl Renderer for OpenGLRenderer {

    fn load(&mut self) {
    }

    fn unload(&mut self) {
    }

    fn queue_render_layer(&mut self, layer: Arc<SceneGraph>) {
        self.render_layer_queue.push_back(layer);
    }

    fn present(&mut self) {
        self.render();
    }
}