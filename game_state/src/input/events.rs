use crate::input::screen::{DeltaVector, ScreenPoint};
use crate::Identity;

#[derive(Debug, Clone)]
pub enum MouseButton {
    Left,
    Middle,
    Right,
    Other(u8),
}

pub type DeviceId = usize;

#[derive(Debug, Clone)]
pub enum JoyAxis {
    LeftStickX,
    LeftStickY,
    LeftZ,
    RightStickX,
    RightStickY,
    RightZ,
    DPadX,
    DPadY,
    Unknown,
}

#[derive(Debug, Clone)]
pub enum JoyButton {
    South,
    East,
    North,
    West,
    C,
    Z,
    LeftTrigger,
    LeftTrigger2,
    RightTrigger,
    RightTrigger2,
    Select,
    Start,
    Mode,
    LeftThumb,
    RightThumb,
    DPadUp,
    DPadDown,
    DPadLeft,
    DPadRight,
    Unknown,
}

#[derive(Debug, Clone)]
pub enum InputEvent {
    KeyDown(Identity, u32),
    KeyUp(Identity, u32),

    MouseDown(Identity, MouseButton),
    MouseUp(Identity, MouseButton),
    MouseMove(Identity, ScreenPoint),
    MouseWheel(Identity, DeltaVector),

    MouseEntered(Identity),
    MouseLeft(Identity),

    JoyAxisChanged(Identity, DeviceId, JoyAxis, f32),
    JoyButtonDown(Identity, DeviceId, JoyButton),
    JoyButtonRepeated(Identity, DeviceId, JoyButton),
    JoyButtonChanged(Identity, DeviceId, JoyButton, f32),
    JoyButtonUp(Identity, DeviceId, JoyButton),
    JoyConnected(Identity, DeviceId),
    JoyDisconnected(Identity, DeviceId),
    JoyEventDropped(Identity, DeviceId),

    CloseRequested(Identity),
    Destroyed(Identity),
    Resized(Identity, f32, f32),
    GainedFocus(Identity),
    LostFocus(Identity),
    Moved(Identity, ScreenPoint),
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::event::{CopyingEventProducer, EventProducer};

    use std::sync::{Arc, Mutex};

    pub type KeysDown = [bool; 256];

    struct KeydownState {
        keys_down: KeysDown,
    }
    impl KeydownState {
        fn get_keys(&self) -> &KeysDown {
            &self.keys_down
        }
        //      fn set_keys(&mut self, keys: KeysDown) { self.keys_down = keys; }
    }

    #[test]
    fn input_publisher_publishes_events() {
        let id = 0u64 as Identity;
        let s1 = KeydownState {
            keys_down: [false; 256],
        };
        let state = Arc::new(Mutex::new(s1));
        let closed_state = state.clone();

        let handler1 = CopyingEventProducer::<InputEvent>::create_handler(
            move |event: InputEvent| match event {
                InputEvent::KeyDown(_id, code) => {
                    closed_state.lock().unwrap().keys_down[code as usize] = true;
                }
                InputEvent::KeyUp(_id, code) => {
                    closed_state.lock().unwrap().keys_down[code as usize] = false;
                }
                _ => {
                    panic!("Nope!");
                }
            },
        );

        let down_event = InputEvent::KeyDown(id, 42);
        let up_event = InputEvent::KeyUp(id, 42);

        let mut producer = CopyingEventProducer::<InputEvent>::new();

        let handler_id = "input_handler_id".to_string();
        producer.add_handler(handler_id.clone(), &handler1);

        // Initial state is unpressed
        let not_pressed = !state.lock().unwrap().get_keys()[42];
        assert!(not_pressed);

        producer.publish(down_event.clone());
        let pressed = state.lock().unwrap().get_keys()[42];
        assert!(pressed);

        producer.publish(up_event);
        let not_pressed = !state.lock().unwrap().get_keys()[42];
        assert!(not_pressed);

        producer.remove_handler(&handler_id);
        producer.publish(down_event);
        let pressed = state.lock().unwrap().get_keys()[42];
        assert!(!pressed);

        let evt = InputEvent::KeyDown(id, 69);
        producer.publish(evt);
    }

    #[test]
    fn input_handler_handles_events() {
        // TODO:....
    }
}
