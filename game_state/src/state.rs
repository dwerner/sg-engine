use super::{ Renderer, Renderable }; //, Physical, Syncable, Identifyable };

use input::events::InputEvent;
use ui::events::UIEvent;

use std::collections::VecDeque;


pub struct State {
    pub renderers: Vec<Box<Renderer>>,
    pub renderables: Vec<Box<Renderable>>,
    pub blob: u64,
    pub input_state: InputState,
    pub ui_state: UIState,
}

impl State {
    pub fn new(renderers: Vec<Box<Renderer>>) -> Self {
        State{
            renderers: renderers,
            renderables: Vec::new(),
            blob: 0,
            input_state: InputState {
                pending_input_events: VecDeque::new()
            },
            ui_state: UIState {
                pending_ui_events: VecDeque::new()
            }
        }
    }
}

pub struct InputState {
    pub pending_input_events: VecDeque<(String, InputEvent)>
}

pub struct UIState {
    pub pending_ui_events: VecDeque<(String, UIEvent)>
}
