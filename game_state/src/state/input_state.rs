use std::collections::VecDeque;
use std::error::Error;

use crate::input::events::InputEvent;
use futures::channel::mpsc::{channel, Receiver, Sender};

pub struct InputState {
    sender: Sender<InputEvent>,
    receiver: Receiver<InputEvent>,
    pub pending_input_events: VecDeque<InputEvent>,
}

impl Default for InputState {
    fn default() -> Self {
        let (sender, receiver) = channel(1024);
        InputState {
            sender,
            receiver,
            pending_input_events: VecDeque::new(),
        }
    }
}

impl InputState {
    pub fn clear(&mut self) {
        self.pending_input_events.clear();
    }
    pub fn send(&self, event: InputEvent) -> Result<(), Box<dyn Error>> {
        Ok(self.sender.clone().try_send(event)?)
    }
}
