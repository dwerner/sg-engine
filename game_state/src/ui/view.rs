use std::collections::HashMap;

use crate::create_next_identity;
use crate::event::{ArcEventHandler, CopyingEventProducer, EventProducer};
use crate::input::screen::{ScreenPoint, ScreenRect};
use crate::ui::events::UIEvent;
use crate::Identifyable;
use crate::Identity;

pub struct UIView {
    pub id: Identity,
    pub tag: String,
    pub bounds: ScreenRect,
}
impl UIView {
    pub fn new(tag: String, bounds: ScreenRect) -> Self {
        UIView {
            id: create_next_identity(),
            tag,
            bounds,
        }
    }

    fn hit_test(&self, point: ScreenPoint) -> bool {
        self.bounds.intersects(&point)
    }

    pub fn move_to_point(&mut self, point: ScreenPoint) {
        self.bounds.x = point.x;
        self.bounds.y = point.y;
    }

    pub fn tag(&self) -> &str {
        &self.tag
    }
}

impl Identifyable for UIView {
    fn identify(&self) -> Identity {
        self.id
    }
}

pub struct UIWindow {
    id: Identity,
    pub tag: String,
    //TODO: should make the view private
    pub view: UIView,
    event_producer: CopyingEventProducer<UIEvent>,
    active_handlers: HashMap<String, ArcEventHandler<UIEvent>>,
}

impl UIWindow {
    pub fn new(tag: String, bounds: ScreenRect) -> Self {
        UIWindow {
            id: create_next_identity(),
            tag: tag.clone(),
            view: UIView::new(tag, bounds),
            event_producer: CopyingEventProducer::<UIEvent>::new(),
            active_handlers: HashMap::new(),
        }
    }
    pub fn perform_click(&mut self, p: ScreenPoint) {
        if self.view.hit_test(p) {
            let event = UIEvent::Clicked(self.id);
            self.event_producer.publish(event);
        }
    }

    pub fn subscribe_click<F: Fn(UIEvent) -> ()>(&mut self, func: F)
    where
        F: 'static,
    {
        let event_handler = CopyingEventProducer::<UIEvent>::create_handler(func);
        let tag = format!("{}_onclick", self.tag);
        // Window will own it's handlers:
        self.active_handlers
            .entry(tag.to_string())
            .or_insert_with(|| event_handler.clone());
        self.event_producer
            .add_handler(tag.to_string(), &event_handler);
    }
}

pub struct UIButton {
    pub text: UIView,
    pub background: UIView,
}

#[cfg(test)]
mod tests {

    use super::*;

    use std::sync::{Arc, Mutex};

    #[test]
    fn test_window_hit_test() {
        let mut window = UIWindow::new("window1".to_string(), ScreenRect::new(10, 10, 20, 20));
        let generic = UIView::new("dunno".to_string(), ScreenRect::new(5, 5, 5, 5));
        let hit = ScreenPoint::new(15, 15);
        let miss = ScreenPoint::new(5, 5);
        window.view.move_to_point(hit);
        assert!(window.view.hit_test(hit));
        assert!(!window.view.hit_test(miss));
    }

    #[test]
    fn test_window_click() {
        let mut window = UIWindow::new("window1".to_string(), ScreenRect::new(10, 10, 20, 20));
        let flag = Arc::new(Mutex::new(0_u32));
        let cflag = flag.clone();

        window.subscribe_click(move |_ /*event:UIEvent*/| {
            *cflag.lock().unwrap() = 1;
        });

        assert!(*flag.lock().unwrap() == 0);

        let point = ScreenPoint::new(15, 15);
        window.perform_click(point);

        assert!(*flag.lock().unwrap() == 1);
    }

}
