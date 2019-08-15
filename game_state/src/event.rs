use std::sync::{Arc, Mutex, Weak};

use std::collections::HashMap;

/// Since we want cross-thread handling of input, we choose Arc
pub type EventHandler<T> = Fn(T) -> ();
pub type ArcEventHandler<T> = Arc<Mutex<Box<EventHandler<T>>>>;
pub type WeakEventHandler<T> = Weak<Mutex<Box<EventHandler<T>>>>;

pub trait EventProducer<T>
where
    T: Copy + Clone,
{
    fn add_handler(&mut self, id: String, handler: &ArcEventHandler<T>);
    fn remove_handler(&mut self, id: &str);
    fn publish(&mut self, event: T);
}

pub struct CopyingEventProducer<T> {
    handlers: HashMap<String, WeakEventHandler<T>>,
}

impl<T> CopyingEventProducer<T> {
    pub fn new() -> Self {
        CopyingEventProducer {
            handlers: HashMap::new(),
        }
    }
    pub fn create_handler<F: Fn(T) -> ()>(func: F) -> ArcEventHandler<T>
    where
        F: 'static,
    {
        Arc::new(Mutex::new(Box::new(func)))
    }
}

impl<T> EventProducer<T> for CopyingEventProducer<T>
where
    T: Copy + Clone,
{
    fn add_handler(&mut self, id: String, handler: &ArcEventHandler<T>) {
        let wh: WeakEventHandler<T> = Arc::downgrade(handler);
        self.handlers.entry(id).or_insert(wh);
    }

    fn remove_handler(&mut self, id: &str) {
        self.handlers.remove(id);
    }

    fn publish(&mut self, event: T) {
        for (_id, handler) in self.handlers.iter() {
            match handler.upgrade() {
                Some(a) => {
                    (*a.lock().unwrap())(event);
                }
                None => {} //arc has been dropped, TODO: notify or log?
            }
        }
    }
}
