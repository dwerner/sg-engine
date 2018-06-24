use super::{
    Renderer,
};

use std::sync::Arc;
use std::collections::VecDeque;

use winit::EventsLoop;
use winit::WindowBuilder;

mod ui_state;
use self::ui_state::{
    UIState
};

mod input_state;
pub use self::input_state::{
    InputState,
};

mod access;
pub use self::access::{
    WorldAccess,
    ModelAccess,
    WindowAccess,
    RenderAccess,
    RenderLayerAccess,
    InputAccess,
};

mod render_state;
pub use self::render_state::{
    SceneGraph,
};


use winit::Window;
use std::sync::Mutex;
use thing;

#[allow(dead_code)]
pub enum DrawMode {
    Wireframe,
    Points,
    Colored
}

pub struct World {
    things: Vec<Arc<thing::Thing>>
}
impl World {
    pub fn add_thing(&mut self, thing: Arc<thing::Thing>) {
        self.things.push(thing);
    }
}

pub struct RenderState {
    models: Vec<Arc<Model>>,
    windows: Vec<WindowWithEvents>,
    window_builders: Vec<WindowBuilder>,
    renderers: Vec<Box<Renderer>>,
    render_layers: Vec<Arc<SceneGraph>>,
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

///
/// This is the central, and global, state passed to each mod during the main loop
///
use super::model::Model;
pub struct State {
    world: World,
    render_state: RenderState,
    input_state: InputState,
    ui_state: UIState,
}

impl State {
    pub fn new() -> Self {
        State{

            world: World{ things: Vec::new() },

            render_state: RenderState::new(),
            input_state: InputState {
                pending_input_events: VecDeque::new(),
                other_input_sources: Vec::new() // input sources added at runtime
            },
            ui_state: UIState::new(),
        }
    }
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
