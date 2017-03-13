
use std::sync::{
    Arc,
    Mutex,
    Weak,
};
// HACK TODO: FIXME
pub type KeysDown = [bool; 256];

/// Since we want cross-thread handling of input, we choose Arc
pub type ArcEventHandler<T> = Arc<Mutex<Box<EventHandler<T>>>>;
pub type WeakEventHandler<T> = Weak<Mutex<Box<EventHandler<T>>>>;

pub trait EventHandler<T> where T: Copy+Clone {
    fn id(&self) -> u32;
    fn event(&mut self, event: &T);

    // TODO: remove the need for this test-only hook into state
    #[cfg(test)] fn keys(&self) -> &KeysDown;
}

pub trait EventProducer<T> where T: Copy+Clone {
    fn add_handler(&mut self, handler: &ArcEventHandler<T>);
    fn remove_handler(&mut self, handler: &ArcEventHandler<T>);
    fn publish(&mut self, event: &T);
}

#[cfg(test)]
mod tests {

    #[derive(Copy, Clone]
    enum TestEvent {
        Yup(i32),
        Nope(String),
        None
    }

    struct TestEventHandler {
        last_event: TestEvent
    }
    impl EventHandler<TestEvent> for TestEventHandler {
        fn id(&self)->u32 { 0 }
        fn event(&mut self, event: &TestEvent) {
            self.last_event = event;
        }
        fn keys(&self) -> &KeysDown {
            // TODO:: FFFFUUUUUUUU
            // Essentially, asking the Box to consume the original type,
            // we lose access to it. Why not just make EventHandler<T> a struct?
        }
    }

    use super::*;

    #[test]
    fn event_handler_producer_test(){

    }
}

