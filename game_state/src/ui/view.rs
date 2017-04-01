use event::{
    ArcEventHandler,
    EventProducer,
    CopyingEventProducer
};

use input::screen::{
    ScreenPoint,
    ScreenRect
};

use cgmath::Matrix4;

use Renderable;
use model::Mesh;
use Identifyable;

use std::collections::HashMap;
use ui::events::UIEvent;

pub struct UIView {
    pub uid: u64,
    pub id: String,
    pub bounds: ScreenRect
}
impl UIView {
    pub fn new(uid: u64, id: String, bounds: ScreenRect) -> Self {
        UIView {
            uid: uid,
            id: id,
            bounds: bounds
        }
    }

    fn hit_test(&self, point: &ScreenPoint) -> bool {
        self.bounds.intersects(point)
    }

    pub fn move_to_point(&mut self, point: &ScreenPoint) {
        self.bounds.x = point.x;
        self.bounds.y = point.y;
    }

    pub fn id(&self) -> &str {
        &self.id
    }
}

impl Identifyable for UIView {
    fn identify(&self) -> u64 { self.uid }
}

impl Renderable for UIView {
    fn get_mesh(&self) -> &Mesh {
        panic!("nope!!!");
    }

    fn get_world_matrix(&self) -> &Matrix4<f32> {
        panic!("nope");
    }
    fn get_model_matrix(&self) -> &Matrix4<f32> {
        panic!("nope");
    }

    fn set_world_matrix(&mut self, mat: Matrix4<f32>) {
        panic!("nope");
    }
    fn set_model_matrix(&mut self, mat: Matrix4<f32>) {
        panic!("nope");
    }
}

pub struct UIWindow {
    pub id: String,
    //TODO: should make the view private
    pub view: UIView,
    event_producer: CopyingEventProducer<UIEvent>,
    active_handlers: HashMap<String, ArcEventHandler<UIEvent>>
}

impl UIWindow {
    pub fn new(id: String, bounds: ScreenRect) -> Self {
        UIWindow{
            id: id.clone(),
            view: UIView::new(0, id, bounds),
            event_producer: CopyingEventProducer::<UIEvent>::new(),
            active_handlers: HashMap::new(),
        }
    }
    pub fn perform_click(&mut self, p: &ScreenPoint) {
        if self.view.hit_test(p) {
            let event = UIEvent::Clicked;
            self.event_producer.publish(event);
        }
    }

    pub fn subscribe_click<F: Fn(UIEvent)->()>(&mut self, func:F) where F: 'static {
        let event_handler = CopyingEventProducer::<UIEvent>::create_handler(func);
        let click_id = format!("{}_onclick", self.id);
        // Window will own it's handlers:
        self.active_handlers.entry(click_id.to_string()).or_insert(event_handler.clone());
        self.event_producer.add_handler(click_id.to_string(), &event_handler);
    }
}

pub struct UIButton {
    pub text: UIView,
    pub background: UIView,
}

#[cfg(test)]
mod tests {

    use super::*;

    use std::sync::{
        Arc, Mutex
    };

    #[test]
    fn test_window_hit_test() {
        let mut window = UIWindow::new("window1".to_string(), ScreenRect::new(10,10,20,20));
        let generic = UIView::new(0, "dunno".to_string(), ScreenRect::new(5,5,5,5));
        let hit = ScreenPoint::new(15,15);
        let miss = ScreenPoint::new(5,5);
        window.view.move_to_point(&hit);
        assert!(generic.id() == "dunno".to_string());
        assert!( window.view.hit_test(&hit) );
        assert!( !window.view.hit_test(&miss) );
    }

    #[test]
    fn test_window_click () {
        let mut window = UIWindow::new("window1".to_string(), ScreenRect::new(10,10,20,20));
        let flag = Arc::new(Mutex::new(0_u32));
        let cflag = flag.clone();
        window.subscribe_click(move |_/*event:UIEvent*/| {
            *cflag.lock().unwrap() = 1;
        });

        assert!(*flag.lock().unwrap() == 0);

        let point = ScreenPoint::new(15,15);
        window.perform_click(&point);
        assert!(*flag.lock().unwrap() == 1);
    }

}