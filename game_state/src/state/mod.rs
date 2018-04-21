use super::{
    Renderer,
}; //, Physical, Syncable, Identifyable };

use std::sync::Arc;
use std::collections::VecDeque;

use winit::EventsLoop;

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
    WindowAccess,
    RenderAccess,
    RenderLayerAccess,
    InputAccess,
    SimulationAccess
};

mod simulation_state;
use self::simulation_state::{
    Simulation,
    SimulationLayer,
};

mod render_state;
pub use self::render_state::{
    SceneGraph,
};

use winit::Window;

use std::sync::Mutex;

#[allow(dead_code)]
pub enum DrawMode {
    Wireframe,
    Points,
    Colored
}

pub struct RenderState {
    windows: Vec<Arc<Window>>,
    renderers: Vec<Box<Renderer>>,
    render_layers: Vec<Arc<SceneGraph>>,
}
impl RenderState {
    pub fn new() -> Self {
        RenderState{
            windows: Vec::new(),
            renderers: Vec::new(),
            render_layers: Vec::new()
        }
    }
}
///
/// This is the central, and global, state passed to each mod during the main loop
///
pub struct State {
    events_loop: Arc<Mutex<EventsLoop>>,
    render_state: RenderState,
    input_state: InputState,
    ui_state: UIState,
    // simulation state
    simulation_state: Simulation,
}

impl State {
    pub fn new() -> Self {
        State{
            events_loop: Arc::new(Mutex::new(EventsLoop::new())), // app-wide wm events loop
            render_state: RenderState::new(),
            input_state: InputState {
                pending_input_events: VecDeque::new(),
                other_input_sources: Vec::new() // input sources added at runtime
            },
            ui_state: UIState::new(),
            simulation_state: Simulation::new(),
        }
    }
}



