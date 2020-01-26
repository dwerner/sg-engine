use std::collections::VecDeque;
use std::time::Duration;

use gilrs::{Axis, Button, Event, EventType, Gilrs};

use game_state::input::events::{DeviceId, InputEvent, JoyAxis, JoyButton};
//use game_state::input::InputSource;
use game_state::state::{InputAccess, State};
use game_state::thing::Direction;
use game_state::{Identifyable, Identity};

struct GamepadInput {
    id: Identity,
    gilrs: Gilrs,
}

impl GamepadInput {
    pub fn new() -> Self {
        GamepadInput {
            id: game_state::create_next_identity(),
            gilrs: Gilrs::new().unwrap(), // TODO: bubble error
        }
    }
}

impl Identifyable for GamepadInput {
    fn identify(&self) -> Identity {
        self.id
    }
}

fn convert_button(b: &Button) -> JoyButton {
    match b {
        Button::South => JoyButton::South,
        Button::East => JoyButton::East,
        Button::North => JoyButton::North,
        Button::West => JoyButton::West,
        Button::C => JoyButton::C,
        Button::Z => JoyButton::Z,
        Button::LeftTrigger => JoyButton::LeftTrigger,
        Button::LeftTrigger2 => JoyButton::LeftTrigger2,
        Button::RightTrigger => JoyButton::RightTrigger,
        Button::RightTrigger2 => JoyButton::RightTrigger2,
        Button::Select => JoyButton::Select,
        Button::Start => JoyButton::Start,
        Button::Mode => JoyButton::Mode,
        Button::LeftThumb => JoyButton::LeftThumb,
        Button::RightThumb => JoyButton::RightThumb,
        Button::DPadUp => JoyButton::DPadUp,
        Button::DPadDown => JoyButton::DPadDown,
        Button::DPadLeft => JoyButton::DPadLeft,
        Button::DPadRight => JoyButton::DPadRight,
        Button::Unknown => JoyButton::Unknown,
    }
}

fn convert_axis(a: &Axis) -> JoyAxis {
    match a {
        Axis::LeftStickX => JoyAxis::LeftStickX,
        Axis::LeftStickY => JoyAxis::LeftStickY,
        Axis::LeftZ => JoyAxis::LeftZ,
        Axis::RightStickX => JoyAxis::RightStickX,
        Axis::RightStickY => JoyAxis::RightStickY,
        Axis::RightZ => JoyAxis::RightZ,
        Axis::DPadX => JoyAxis::DPadX,
        Axis::DPadY => JoyAxis::DPadY,
        Axis::Unknown => JoyAxis::Unknown,
    }
}

/*
// TODO: implement from<T> instead
// TODO: VecDeque -> (Event Channel)
impl InputSource for GamepadInput {
    fn get_input_events(&mut self) -> VecDeque<InputEvent> {
        let mut converted_events = VecDeque::new();
        //...
        while let Some(Event { id, event, time }) = self.gilrs.next_event() {
            let event = match event {
                EventType::ButtonPressed(button, _code) => {
                    let b = convert_button(&button);
                    converted_events.push_back(InputEvent::JoyButtonDown(self.identify(), id, b));
                }
                EventType::ButtonRepeated(button, _code) => {
                    let b = convert_button(&button);
                    converted_events.push_back(InputEvent::JoyButtonRepeated(
                        self.identify(),
                        id,
                        b,
                    ));
                }
                EventType::ButtonReleased(button, _code) => {
                    let b = convert_button(&button);
                    converted_events.push_back(InputEvent::JoyButtonUp(self.identify(), id, b));
                }
                EventType::ButtonChanged(button, value, _code) => {
                    let b = convert_button(&button);
                    converted_events.push_back(InputEvent::JoyButtonChanged(
                        self.identify(),
                        id,
                        b,
                        value,
                    ));
                }
                EventType::AxisChanged(axis, value, _code) => {
                    let a = convert_axis(&axis);
                    converted_events.push_back(InputEvent::JoyAxisChanged(
                        self.identify(),
                        id,
                        a,
                        value,
                    ));
                }
                EventType::Connected => {
                    converted_events.push_back(InputEvent::JoyConnected(self.identify(), id));
                }
                EventType::Disconnected => {
                    converted_events.push_back(InputEvent::JoyDisconnected(self.identify(), id));
                }
                EventType::Dropped => {
                    converted_events.push_back(InputEvent::JoyEventDropped(self.identify(), id));
                }
            };
        }
        converted_events
    }
}
*/

#[no_mangle]
pub extern "C" fn mod_gamepad_load(state: &mut State) {
  /*  let gamepad_input = GamepadInput::new();
    let gamepad_input = Box::new(gamepad_input) as Box<InputSource>;
    state.add_input_source(gamepad_input);
  */
}

#[no_mangle]
pub extern "C" fn mod_gamepad_update(state: &mut State, dt: &Duration) {}

#[no_mangle]
pub extern "C" fn mod_gamepad_unload(state: &mut State) {
    unimplemented!()
}
