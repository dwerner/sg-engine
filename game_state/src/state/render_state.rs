use std::sync::{Arc, Mutex};

use winit::event_loop::EventLoop;
use winit::window::{Window, WindowBuilder};

use super::{Model, Renderer};
use crate::tree::RcNode;
use crate::Identity;

#[derive(Default)]
pub struct SceneGraph {
    pub root: RcNode<Identity>,
}

#[allow(dead_code)]
pub enum DrawMode {
    Wireframe,
    Points,
    Colored,
}

#[derive(Clone)]
pub struct WindowWithEvents {
    window: Arc<Window>,
    event_loop: Arc<Mutex<Option<EventLoop<()>>>>,
}

impl WindowWithEvents {
    pub fn new(window: Arc<Window>, event_loop: Arc<Mutex<Option<EventLoop<()>>>>) -> Self {
        WindowWithEvents { window, event_loop }
    }

    pub fn get_window(&self) -> &Arc<Window> {
        &self.window
    }

    pub fn get_event_loop(&self) -> &Arc<Mutex<Option<EventLoop<()>>>> {
        &self.event_loop
    }
}

#[derive(Default)]
pub struct RenderState {
    pub models: Vec<Arc<Model>>,
    pub windows: Vec<WindowWithEvents>,
    pub window_builders: Vec<WindowBuilder>,
    pub renderers: Vec<Box<dyn Renderer>>,
    pub render_layers: Vec<Arc<SceneGraph>>,
}
