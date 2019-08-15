pub mod events;
pub mod screen;

use crate::Identifyable;

use std::collections::VecDeque;

pub trait InputSource: Identifyable {
    // get_input_events() <- renderer should be tracking input events
    fn get_input_events(&mut self) -> VecDeque<events::InputEvent>;
}
