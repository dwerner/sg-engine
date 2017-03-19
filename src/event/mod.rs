
use std::sync::{
    Arc,
    Mutex,
    Weak,
};
// HACK TODO: FIXME
pub type KeysDown = [bool; 256];

/// Since we want cross-thread handling of input, we choose Arc
pub type EventHandler<T>= Fn(T) -> ();
pub type ArcEventHandler<T> = Arc<Mutex<Box<EventHandler<T>>>>;
pub type WeakEventHandler<T> = Weak<Mutex<Box<EventHandler<T>>>>;

/*
pub trait EventHandler<T> where T: Copy+Clone {
    fn id(&self) -> u32;
    fn event(&mut self, event: &T);
}
*/


pub trait EventProducer<T> where T: Copy+Clone {
    fn add_handler(&mut self, handler: &ArcEventHandler<T>);
    fn remove_handler(&mut self, handler: &ArcEventHandler<T>);
    fn publish(&mut self, event: T);
}

#[cfg(test)]
mod tests {

    #[derive(Copy, Clone)]
    enum TestEvent {
        Yup(i32),
        Nope,
    }


    use super::*;

    #[test]
    fn event_handler_producer_test(){
    }
}

