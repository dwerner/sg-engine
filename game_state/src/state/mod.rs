use std::collections::VecDeque;

mod access;
mod input_state;
mod render_state;
mod ui_state;

use super::model::Model;
use super::Renderer;
use crate::thing::World;

pub use self::access::{
    InputAccess, ModelAccess, RenderAccess, RenderLayerAccess, WindowAccess, WorldAccess,
};
pub use self::input_state::InputState;
pub use self::render_state::{DrawMode, RenderState, SceneGraph, WindowWithEvents};
use self::ui_state::UIState;

///
/// This is the central, and global, state passed to each mod during the main loop
///
#[derive(Default)]
pub struct State {
    /// Root container of the Thing/Facet system (game world state)
    world: World,

    /// Container for all rendering state
    render_state: RenderState,

    /// Container for all input state
    input_state: InputState,

    /// Container for all UI related state
    ui_state: UIState,
}
