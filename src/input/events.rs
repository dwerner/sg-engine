use input::screen::{
    DeltaVector,
    ScreenPoint,
    ScreenRect,
};

#[derive(Copy, Clone)]
pub enum MouseButton {
    Left,
    Middle,
    Right,
    ScrollWheel
}

#[derive(Copy, Clone)]
pub struct JoystickButton(u32);

#[derive(Copy, Clone)]
pub enum InputEvent {
    KeyDown(u8),
    KeyUp(u8),

    MouseDown(MouseButton, ScreenPoint),
    MouseUp(MouseButton, ScreenPoint),
    MouseMove(ScreenPoint, DeltaVector),

    JoystickMove(Device, DeltaVector),
    JoystickButtonDown(Device, JoystickButton),
    JoystickButtonUp(Device, JoystickButton),

    WmQuit,
    WmResize(ScreenRect),
    WmMove(ScreenPoint)
}

#[derive(Copy, Clone)]
pub struct Device {
    id: u32,
}

#[cfg(test)]
mod tests {
    use std::sync::{
        Arc,
        Mutex,
        Weak,
    };
    use event::{
        EventHandler,
        EventProducer,
        ArcEventHandler,
        WeakEventHandler,
        KeysDown,
    };
    use super::*;

    struct KeydownState { keys_down: KeysDown }
    impl KeydownState {
        fn get_keys(&self) -> &KeysDown { &self.keys_down }
        fn set_keys(&mut self, keys: KeysDown) { self.keys_down = keys; }
    }


    struct KeydownProducer<'a> {
        handlers: Vec<WeakEventHandler<&'a InputEvent>>,
    }

    impl <'a> EventProducer<&'a InputEvent> for KeydownProducer<'a> {
        fn add_handler(&mut self, handler: &ArcEventHandler<&'a InputEvent>) {
            let wh: WeakEventHandler<&InputEvent> = Arc::downgrade(handler);
            self.handlers.push(wh);
        }

        fn remove_handler(&mut self, handler: &ArcEventHandler<&'a InputEvent>) {
            // TODO: complete this using EventHandler::id when appropriate
        }

        fn publish(&mut self, event: &'a InputEvent) {
            for handler in self.handlers.iter() {
                match handler.upgrade() {
                    Some(a) => {
                        (*a.lock().unwrap())(event);
                    },
                    None => {} //arc has been dropped
                }
            }
        }
    }

    #[test]
    fn input_publisher_publishes_events() {


        let mut s1 = KeydownState{ keys_down:[false; 256] };
        let mut state = Arc::new( Mutex::new(s1) );
        let closed_state = state.clone();

        let handler = move |event: &InputEvent| {
            match event {
                &InputEvent::KeyDown(code) => {
                    closed_state.lock().unwrap().keys_down[code as usize] = true;
                },
                &InputEvent::KeyUp(code) => {
                    closed_state.lock().unwrap().keys_down[code as usize] = false;
                },
                _ => { panic!("Nope!"); }
            }
        };

        let event = InputEvent::KeyDown(42);
        let event_ref = &event;
        let mut producer = KeydownProducer{ handlers: Vec::new() };

        let b1: Box<EventHandler<&InputEvent>> = Box::new(handler);
        let mut handler1 = Arc::new(Mutex::new(b1));
        producer.add_handler(&handler1);
        producer.publish(event_ref);

        // TODO: along with test-only keys() hook, fix this test to not require it
        let pressed = state.lock().unwrap().get_keys()[42];
        assert!(pressed);

    }

    #[test]
    fn input_handler_handles_events() {
        // TODO:....
    }
}

