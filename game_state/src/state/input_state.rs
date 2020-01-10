use crate::input::events::InputEvent;
use crate::input::screen::ScreenPoint;
use std::collections::VecDeque;
use std::error::Error;

pub struct InputState {
    events: VecDeque<InputEvent>,
    mouse_pos: ScreenPoint,
}

impl Default for InputState {
    fn default() -> Self {
        InputState {
            events: Default::default(),
            mouse_pos: ScreenPoint::new(0, 0),
        }
    }
}

impl InputState {
    pub fn clear(&mut self) {
        self.events.clear();
    }
    pub fn send(&mut self, event: InputEvent) -> Result<(), Box<dyn Error>> {
        self.events.push_back(event);
        Ok(())
    }
    pub fn get_input_events(&self) -> &VecDeque<InputEvent> {
        &self.events
    }

    pub fn get_mouse_pos(&self) -> &ScreenPoint {
        &self.mouse_pos
    }

    pub fn set_mouse_pos(&mut self, sp: ScreenPoint) {
        self.mouse_pos = sp;
    }
}
