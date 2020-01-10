use super::model::Model;
use super::Renderer;
use crate::thing::World;

pub use self::access::{
    InputAccess, ModelAccess, RenderAccess, RenderLayerAccess, WindowAccess, WorldAccess,
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

    /// Root container of the Thing/Facet system (game world state)
    world: World,

    /// Container for all rendering state
    render_state: RenderState,

    /// Container for all input state
    input_state: InputState,

    /// Container for all UI related state
    ui_state: UIState,
}

impl Default for State {
    fn default() -> Self {
        Self {
            sdl_context: sdl2::init().expect("unable to create sdl2 context"),
            world: Default::default(),
            render_state: Default::default(),
            input_state: Default::default(),
            ui_state: Default::default(),
        }
    }
}
