use std::sync::Arc;

use sdl2::video::Window;

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

pub struct RenderState {
    pub models: Vec<Arc<Model>>,
    pub windows: Vec<Arc<Window>>,
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
