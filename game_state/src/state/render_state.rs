use Identity;

use std::sync::{
    Arc,
    Mutex,
};

use super::Model;
use super::Renderer;

use winit::{
    Window,
    EventsLoop,
    WindowBuilder,
};

use tree::{ RcNode };
pub struct SceneGraph {
    pub root: RcNode<Identity>,
}

#[allow(dead_code)]
pub enum DrawMode {
    Wireframe,
    Points,
    Colored
}

#[derive(Clone)]
pub struct WindowWithEvents {
    window: Arc<Window>,
    event_loop: Arc<Mutex<EventsLoop>>
}

impl WindowWithEvents {
    pub fn new( window: Arc<Window>, event_loop: Arc<Mutex<EventsLoop>> ) -> Self {
        WindowWithEvents { window, event_loop }
    }

    pub fn get_window(&self) -> &Arc<Window> {
        &self.window
    }

    pub fn get_event_loop(&self) -> &Arc<Mutex<EventsLoop>> {
        &self.event_loop
    }
}

pub struct RenderState {
    pub models: Vec<Arc<Model>>,
    pub windows: Vec<WindowWithEvents>,
    pub window_builders: Vec<WindowBuilder>,
    pub renderers: Vec<Box<Renderer>>,
    pub render_layers: Vec<Arc<SceneGraph>>,
}
impl RenderState {
    pub fn new() -> Self {
        RenderState{
            models: Vec::new(),
            windows: Vec::new(),
            window_builders: Vec::new(), // glutin requires a builder
            renderers: Vec::new(),
            render_layers: Vec::new()
        }
    }
}
