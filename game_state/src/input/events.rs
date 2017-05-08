use Identity;



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

    MouseDown(Identity, MouseButton, ScreenPoint),
    MouseUp(Identity, MouseButton, ScreenPoint),
    MouseMove(Identity, ScreenPoint, DeltaVector),

    JoystickMove(Identity, Device, DeltaVector),
    JoystickButtonDown(Identity, Device, JoystickButton),
    JoystickButtonUp(Identity, Device, JoystickButton),

    WmQuit(Identity),
    WmResize(Identity, ScreenRect),
    WmMove(Identity, ScreenPoint)
}


#[derive(Copy, Clone)]
pub struct Device {
    //id: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    use event::{
        EventProducer,
        CopyingEventProducer,
    };

    use std::sync::{
        Arc,
        Mutex,
    };

    pub type KeysDown = [bool; 256];

    struct KeydownState { keys_down: KeysDown }
    impl KeydownState {
        fn get_keys(&self) -> &KeysDown { &self.keys_down }
  //      fn set_keys(&mut self, keys: KeysDown) { self.keys_down = keys; }
    }

    #[test]
    fn input_publisher_publishes_events() {
        let s1 = KeydownState{ keys_down:[false; 256] };
        let state = Arc::new( Mutex::new(s1) );
        let closed_state = state.clone();

        let handler1 = CopyingEventProducer::<InputEvent>::create_handler(move |event: InputEvent| {
            match event {
                InputEvent::KeyDown(code) => {
                    closed_state.lock().unwrap().keys_down[code as usize] = true;
                },
                InputEvent::KeyUp(code) => {
                    closed_state.lock().unwrap().keys_down[code as usize] = false;
                },
                _ => { panic!("Nope!"); }
            }
        });

        let down_event = InputEvent::KeyDown(42);
        let up_event = InputEvent::KeyUp(42);

        let mut producer = CopyingEventProducer::<InputEvent>::new();

        let handler_id = "input_handler_id".to_string();
        producer.add_handler(handler_id.clone(), &handler1);

        // Initial state is unpressed
        let not_pressed = !state.lock().unwrap().get_keys()[42];
        assert!(not_pressed);

        producer.publish(down_event);
        let pressed = state.lock().unwrap().get_keys()[42];
        assert!(pressed);

        producer.publish(up_event);
        let not_pressed = !state.lock().unwrap().get_keys()[42];
        assert!(not_pressed);

        producer.remove_handler(&handler_id);
        producer.publish(down_event);
        let pressed = state.lock().unwrap().get_keys()[42];
        assert!(!pressed);

        let evt = InputEvent::KeyDown(69);
        producer.publish( evt );

    }

    #[test]
    fn input_handler_handles_events() {
        // TODO:....
    }
}

