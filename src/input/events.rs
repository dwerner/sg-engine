use super::screen::{
    DeltaVector,
    ScreenPoint,
    ScreenRect,
};

use std::sync::{
    Arc,
    Mutex,
    Weak,
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

/// Keep track of all keys t
type KeysDown = [bool; 256];

/// Since we want cross-thread handling of input, we choose Arc
type ArcInputHandler = Arc<Mutex<Box<InputHandler>>>;
type WeakInputHandler = Weak<Mutex<Box<InputHandler>>>;

pub trait InputHandler {
    fn event(&mut self, target_id: u32, event: &InputEvent);

    #[cfg(test)] fn keys(&self) -> &KeysDown;
}

pub trait InputEventProducer {
    fn add_handler(&mut self, handler: &ArcInputHandler);
    // fn remove_handler(&mut self, handler: ArcInputHandler);
    fn publish(&mut self, event: &InputEvent);
}


#[cfg(test)]
mod tests {
    use super::*;

    struct KeydownHandler { keys_down: KeysDown }
    impl KeydownHandler { fn keys(&self) -> &KeysDown { &self.keys_down } }
    impl InputHandler for KeydownHandler {

        fn event(&mut self, target_id: u32, event: &InputEvent) {
            match event {
                &InputEvent::KeyDown(code) => {
                    self.keys_down[code as usize] = true;
                },
                &InputEvent::KeyUp(code) => {
                    self.keys_down[code as usize] = false;
                },
                _ => { panic!("Nope!"); }
            }
        }

        #[cfg(test)] fn keys(&self) -> &KeysDown {
            &self.keys_down
        }
    }

    struct KeydownProducer {
        handlers: Vec<WeakInputHandler>,
    }

    impl InputEventProducer for KeydownProducer {
        fn add_handler(&mut self, handler: &ArcInputHandler) {
            let wh: WeakInputHandler = Arc::downgrade(handler);
            self.handlers.push(wh);
        }

        fn publish(&mut self, event: &InputEvent) {
            for handler in self.handlers.iter() {
                match handler.upgrade() {
                    Some(a) => {
                        a.lock().unwrap().event(0, event);
                    },
                    None => {} //dropped
                }
            }
        }
    }

    #[test]
    fn input_publisher_publishes_events() {

        let mut producer = KeydownProducer{ handlers: Vec::new() };

        // need these type hints for trait object
        let b1: Box<InputHandler> = Box::new(KeydownHandler{ keys_down:[false; 256] });
        let b2: Box<InputHandler> = Box::new(KeydownHandler{ keys_down:[false; 256] });

        let mut handler1 = Arc::new( Mutex::new(b1) );
        let mut handler2 = Arc::new( Mutex::new(b2) );

        producer.add_handler( &handler1 );
        producer.add_handler( &handler2 );

        let event = InputEvent::KeyDown(42);
        producer.publish(&event);

        let pressed = handler1.lock().unwrap().keys()[42];
        let pressed2 = handler2.lock().unwrap().keys()[42];
        assert!(pressed && pressed2);

    }

    #[test]
    fn input_handler_handles_events() {
        let mut handler = KeydownHandler{ keys_down: [false; 256] };
        let down_event = InputEvent::KeyDown(42);
        handler.event(0, &down_event);

        let up_event = InputEvent::KeyUp(42);
        assert_eq!(handler.keys()[42], true);

        handler.event(0, &up_event);
        assert_eq!(handler.keys()[42], false);
    }
}

