use std::collections::VecDeque;

use crate::input::events::InputEvent;
use crate::input::InputSource;

#[derive(Default)]
pub struct InputState {
    pub pending_input_events: VecDeque<InputEvent>,
    pub other_input_sources: Vec<Box<dyn InputSource>>,
}

impl InputState {
    pub fn clear(&mut self) {
        // TODO add any useful clearing of state here
        self.pending_input_events.clear();
    }
}
