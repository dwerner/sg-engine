use super::{ Renderer, Renderable }; //, Physical, Syncable, Identifyable };

use input::events::InputEvent;
use ui::events::UIEvent;
use ui::view::UIView;

use std::collections::VecDeque;


pub struct State {
    pub renderers: Vec<Box<Renderer>>,
    pub blob: u64,
    pub input_state: InputState,
    pub ui_state: UIState,
}

impl State {
    pub fn new(renderers: Vec<Box<Renderer>>) -> Self {
        State{
            renderers: renderers,
            blob: 0,
            input_state: InputState {
                pending_input_events: VecDeque::new()
            },
            ui_state: UIState {
                pending_ui_events: VecDeque::new(),
                scene: SceneGraph {
                    root: tree::Node(0,0, None)
                }
            }
        }
    }
}

pub struct InputState {
    pub pending_input_events: VecDeque<(String, InputEvent)>
}

pub struct SceneGraph<T: Renderable> {
    root: tree::Node<T>,
}

pub struct UIState {
    pub pending_ui_events: VecDeque<(String, UIEvent)>,
    pub scene: SceneGraph
}
