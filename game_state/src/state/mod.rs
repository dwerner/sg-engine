use super::{
    Renderer,
    Renderable
}; //, Physical, Syncable, Identifyable };

use std::sync::Arc;
use std::collections::VecDeque;
use tree::{ RcNode };

use input::events::InputEvent;
use ui::events::UIEvent;
use input::InputSource;

use winit::EventsLoop;

mod access;
pub use state::access::{
    WindowAccess,
    RenderAccess,
    RenderLayerAccess,
    InputAccess,
    SimulationAccess
};

use winit::Window;

use std::sync::Mutex;

#[allow(dead_code)]
pub enum DrawMode {
    Wireframe,
    Points,
    Colored
}

///
/// This is the central, and global, state passed to each mod during the main loop
///
pub struct State {
    events_loop: Arc<Mutex<EventsLoop>>,
    windows: Vec<Arc<Mutex<Window>>>,
    renderers: Vec<Box<Renderer>>,
    render_layers: Vec<Arc<SceneGraph>>,

    input_state: InputState,
    gui_state: UIState,
    // simulation state
    _syncable_state: SyncState, // stub for a struct atm, networking state?
    simulation_state: Simulation,
}

impl State {
    pub fn new() -> Self {
        State{
            events_loop: Arc::new(Mutex::new(EventsLoop::new())), // app-wide wm events loop
            windows: Vec::new(),
            renderers: Vec::new(),
            render_layers: Vec::new(),
            input_state: InputState {
                pending_input_events: VecDeque::new(),
                other_input_sources: Vec::new() // input sources added at runtime
            },
            gui_state: UIState {
                pending_ui_events: VecDeque::new(),
            },
            _syncable_state: SyncState,
            simulation_state: Simulation::new(),
        }
    }
}

// All global state for simulation
pub struct Simulation {
    layers: Vec<SimulationLayer>
}

impl Simulation {
    pub fn new() -> Self{
        Simulation{layers:Vec::new()}
    }
}

pub struct SimulationLayer{}

pub struct SyncState; // Stub type for now

pub struct InputState {
    pub pending_input_events: VecDeque<InputEvent>,
    pub other_input_sources: Vec<Box<InputSource>>,
}

impl InputState {
    pub fn clear(&mut self) {
        // TODO add any useful clearing of state here
        self.pending_input_events.clear();
    }
}

pub struct SceneGraph {
    pub root: RcNode<Box<Renderable>>,
}

pub struct UIState {
    pub pending_ui_events: VecDeque<UIEvent>,
    //pub scene: SceneGraph
}
