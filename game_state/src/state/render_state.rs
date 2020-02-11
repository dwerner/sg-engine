use std::sync::Arc;

use sdl2::video::Window;

use super::{Model, Renderer};
use crate::tree::RcNode;

#[derive(Default)]
pub struct SceneGraph<T = Option<Arc<Model>>> {
    pub root: RcNode<T>,
}

#[derive(Copy, Clone)]
pub enum DrawMode {
    Wireframe(f32),
    Points,
    Textured,
}

pub struct WindowWithAttrs {
    pub window: Window,
    pub draw_mode: DrawMode,
}

pub struct RenderState {
    pub models: Vec<Arc<Model>>,
    pub windows: Vec<WindowWithAttrs>,
    pub renderers: Vec<Box<dyn Renderer>>,
    pub render_layers: Vec<Arc<SceneGraph>>,
}

impl Default for RenderState {
    fn default() -> Self {
        Self {
            models: Vec::new(),
            windows: Vec::new(),
            renderers: Vec::new(),
            render_layers: Vec::new(),
        }
    }
}
