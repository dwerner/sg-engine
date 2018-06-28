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
    DrawMode,
    SceneGraph,
    RenderState,
    WindowWithEvents,
};


use winit::Window;
use std::sync::Mutex;

use thing;
use thing::World;


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

            world: World::new(),

            render_state: RenderState::new(),
            input_state: InputState {
                pending_input_events: VecDeque::new(),
                other_input_sources: Vec::new() // input sources added at runtime
            },
            ui_state: UIState::new(),
        }
    }
}
