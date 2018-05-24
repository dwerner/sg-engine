use super::{
    Renderer,
}; //, Physical, Syncable, Identifyable };

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
use thing;

#[allow(dead_code)]
pub enum DrawMode {
    Wireframe,
    Points,
    Colored
}

pub struct World {
    things: Vec<thing::Thing>
}

pub struct RenderState {
    windows: Vec<Arc<Window>>,
    window_builders: Vec<WindowBuilder>,
    renderers: Vec<Box<Renderer>>,
    render_layers: Vec<Arc<SceneGraph>>,
}
impl RenderState {
    pub fn new() -> Self {
        RenderState{
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
pub struct State {
    world: World,
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
            world: World{ things: Vec::new() },

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



