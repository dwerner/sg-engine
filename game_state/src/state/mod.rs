use std::collections::HashMap;

use super::model::Model;
use super::Renderer;
use crate::thing::World;

pub use self::access::{
    InputAccess, ModelAccess, RenderAccess, RenderLayerAccess, VariableAccess, WindowAccess,
    WorldAccess,
};
pub use self::input_state::InputState;
pub use self::render_state::{DrawMode, RenderState, SceneGraph};
use self::ui_state::UIState;

mod access;
mod input_state;
mod render_state;
mod ui_state;

///
/// This is the central, and global, state passed to each mod during the main loop
///
pub struct State {
    pub sdl_context: sdl2::Sdl,
    pub sdl_subsystems: SdlSubsystems,

    /// Root container of the Thing/Facet system (game world state)
    world: World,

    /// Container for all rendering state
    render_state: RenderState,

    /// Container for all input state
    input_state: InputState,

    /// Container for all UI related state
    ui_state: UIState,

    // TEMPORARY container for general purpose state variables
    pub variables: HashMap<&'static str, Variable>,
}

#[derive(Copy, Clone)]
pub enum Variable {
    Bool(bool),
}

pub struct SdlSubsystems {
    pub video: sdl2::VideoSubsystem,
    pub event_pump: sdl2::EventPump,
}

impl Default for State {
    fn default() -> Self {
        let ctx = sdl2::init().expect("unable to create sdl2 context");
        let video = ctx.video().expect("unable to create video subsystem");
        let event_pump = ctx.event_pump().expect("unable to create event pump");
        Self {
            sdl_context: ctx,
            sdl_subsystems: SdlSubsystems { video, event_pump },
            world: Default::default(),
            render_state: Default::default(),
            input_state: Default::default(),
            ui_state: Default::default(),
            variables: HashMap::new(),
        }
    }
}
