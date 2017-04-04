extern crate game_state;

use game_state::state::SceneGraph;
use game_state::Renderable;
use game_state::Renderer;

use std::sync::Arc;
use std::collections::VecDeque;

pub struct SoftwareRenderer {
    w:u8,
    h:u8,
    title: String,
    render_layer_queue: VecDeque<Arc<SceneGraph>>
}
impl SoftwareRenderer {

    pub fn new(title: String, w: u32, h: u32) -> Self {
        SoftwareRenderer{
            w: w,
            h: h,
            title: title,
            render_layer_queue: VecDeque::new()
        }
    }
    fn render(&mut self){}
}

impl Renderer for SoftwareRenderer {
    fn load(&mut self) { }
    fn unload(&mut self) { }

    fn queue_render_layer(&mut self, layer: Arc<SceneGraph>) {
        self.render_layer_queue.push_back(layer);
    }

    fn present(&mut self) {
        self.render();
    }
}
