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
/// Altogether these tests prove jack shit of functionality, really they were about me learning
/// HOW one goes about supporting upcasting... in this case with AsBase, however they type information
/// injected into the trait results in a single concrete type, however implemented with dynamic dispatch... :(
/// So basically, I just tied my dick in a knot. i.e., I could just use a single struct with impl. I still
/// don't get dynamic dispatch over many different types of handlers that I care about.

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
        AsBase,
    };
    use super::*;

    struct KeydownHandler { keys_down: KeysDown }
    impl KeydownHandler { fn keys(&self) -> &KeysDown { &self.keys_down } }
    impl AsBase<KeydownHandler> for KeydownHandler {
        fn as_base(&self) -> &KeydownHandler { self }
    }
    impl EventHandler<InputEvent, KeydownHandler> for KeydownHandler {
        fn id(&self) -> u32 { 0 }
        fn event(&mut self, event: &InputEvent) {
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
    }

    struct KeydownProducer {
        handlers: Vec<WeakEventHandler<InputEvent, KeydownHandler>>,
    }

    impl EventProducer<InputEvent, KeydownHandler> for KeydownProducer {
        fn add_handler(&mut self, handler: &ArcEventHandler<InputEvent, KeydownHandler>) {
            let wh: WeakEventHandler<InputEvent, KeydownHandler> = Arc::downgrade(handler);
            self.handlers.push(wh);
        }

        fn remove_handler(&mut self, handler: &ArcEventHandler<InputEvent, KeydownHandler>) {
            // TODO: complete this using EventHandler::id when appropriate
        }

        fn publish(&mut self, event: &InputEvent) {
            for handler in self.handlers.iter() {
                match handler.upgrade() {
                    Some(a) => {
                        a.lock().unwrap().event(event);
                    },
                    None => {} //arc has been dropped
                }
            }
        }
    }

    #[test]
    fn input_publisher_publishes_events() {

        let mut producer = KeydownProducer{ handlers: Vec::new() };

        let h1 = KeydownHandler{ keys_down:[false; 256] };
        let h2 = KeydownHandler{ keys_down:[false; 256] };

        let bh1: Box<KeydownHandler> = Box::new(h1);
        let bh2: Box<KeydownHandler> = Box::new(h2);

        // need these type hints for trait object
        let b1: Box<EventHandler<InputEvent, KeydownHandler>> = bh1;
        let b2: Box<EventHandler<InputEvent, KeydownHandler>> = bh2;

        let mut handler1 = Arc::new( Mutex::new(b1) );
        let mut handler2 = Arc::new( Mutex::new(b2) );

        producer.add_handler( &handler1 );
        producer.add_handler( &handler2 );

        let event = InputEvent::KeyDown(42);
        producer.publish(&event);

        // TODO: along with test-only keys() hook, fix this test to not require it
        let pressed = handler1.lock().unwrap().as_base().keys()[42];
        let pressed2 = handler2.lock().unwrap().as_base().keys()[42];
        assert!(pressed && pressed2);

    }

    #[test]
    fn input_handler_handles_events() {
        let mut handler = KeydownHandler{ keys_down: [false; 256] };
        let down_event = InputEvent::KeyDown(42);
        handler.event(&down_event);

        let up_event = InputEvent::KeyUp(42);
        assert_eq!(handler.keys()[42], true);

        handler.event(&up_event);
        assert_eq!(handler.keys()[42], false);
    }
}

