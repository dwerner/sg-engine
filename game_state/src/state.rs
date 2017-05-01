use super::{
    Renderer,
    //Renderable
}; //, Physical, Syncable, Identifyable };

use input::events::InputEvent;
use ui::events::UIEvent;
// use ui::view::UIView;

use std::collections::VecDeque;
use tree::{ RcNode };
use std::sync::Arc;
use input::screen::ScreenRect;
use model::Model;

pub struct State {
    pub renderers: Vec<Box<Renderer>>,
    pub render_layers: Vec<Arc<SceneGraph>>,
    pub input_state: InputState,
  //  pub ui_state: UIState,
}

impl State {
    pub fn new() -> Self {
        State{
            renderers: Vec::new(),
            render_layers: Vec::new(),
            input_state: InputState {
                pending_input_events: VecDeque::new()
            },
   /*         ui_state: UIState {
                pending_ui_events: VecDeque::new(),
                scene: SceneGraph {
                    root: Node::create(
                            Box::new(UIView::new(0, "id".to_string(), ScreenRect::new(0,0,5,5))
                        ), None)
                }
            }
            */
        }
    }
}

pub struct InputState {
    pub pending_input_events: VecDeque<(String, InputEvent)>
}

pub struct SceneGraph {
    pub root: RcNode<Box<Model>>,
}

pub struct UIState {
    pub pending_ui_events: VecDeque<(String, UIEvent)>,
    pub scene: SceneGraph
}
