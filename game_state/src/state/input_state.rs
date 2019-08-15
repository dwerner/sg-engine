use std::collections::VecDeque;

use crate::input::events::InputEvent;
use crate::input::InputSource;

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
