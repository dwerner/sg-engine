
use std::sync::{
    Arc,
    Mutex,
    Weak,
};
// HACK TODO: FIXME
pub type KeysDown = [bool; 256];

/// Since we want cross-thread handling of input, we choose Arc
pub type ArcEventHandler<T, H> = Arc<Mutex<Box<EventHandler<T, H>>>>;
pub type WeakEventHandler<T, H> = Weak<Mutex<Box<EventHandler<T, H>>>>;

pub trait AsBase<T> {
    fn as_base(&self) -> &T;
}

pub trait EventHandler<T, H> : AsBase<H> where T: Copy+Clone {
    fn id(&self) -> u32;
    fn event(&mut self, event: &T);
}

pub trait EventProducer<T, H> where T: Copy+Clone {
    fn add_handler(&mut self, handler: &ArcEventHandler<T, H>);
    fn remove_handler(&mut self, handler: &ArcEventHandler<T, H>);
    fn publish(&mut self, event: &T);
}

#[cfg(test)]
mod tests {

    #[derive(Copy, Clone)]
    enum TestEvent {
        Yup(i32),
        Nope,
    }

    struct TestEventHandler {
        last_event: TestEvent
    }
    impl AsBase<TestEventHandler> for TestEventHandler {
        fn as_base(&self) -> &TestEventHandler { self }
    }
    impl EventHandler<TestEvent, TestEventHandler> for TestEventHandler {
        fn id(&self)->u32 { 0 }
        fn event(&mut self, event: &TestEvent) {
            self.last_event = *event;
        }
    }

    use super::*;

    #[test]
    fn event_handler_producer_test(){

        let handler = TestEventHandler{ last_event: TestEvent::Nope };
    }
}

