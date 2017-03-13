use input::screen::{ScreenPoint, ScreenRect};

pub struct UIView {
    pub id: String,
    pub children: Vec<UIView>,
    bounds: ScreenRect
}
impl UIView {
    pub fn new(id: String, bounds: ScreenRect) -> Self {
        UIView {
            id: id,
            children: Vec::new(),
            bounds: bounds
        }
    }

    fn add_child(&mut self, child: UIView) {
        self.children.push(child);
    }

    fn get_children(&self) -> &Vec<UIView> {
        &self.children
    }

    fn get_child_by_id(&self, id: u32) -> Option<&UIView> {
        None
    }

    fn hit_test(&self, point: &ScreenPoint) -> bool {
        self.bounds.intersects(point)
    }

    fn get_bounds(&self) -> &ScreenRect {
        &self.bounds
    }

    fn move_to_point(&mut self, point: &ScreenPoint) {
        self.bounds.x = point.x;
        self.bounds.y = point.y;
    }
}

pub struct UIWindow {
    pub id: String,
    pub view: UIView,
}
impl UIWindow {
    pub fn new(id: String, bounds: ScreenRect) -> Self {
        UIWindow{ id:id.clone(), view: UIView::new(id, bounds) }
    }
    pub fn click(&mut self, p: &ScreenPoint) {
        let hit = self.view.hit_test(p);
    }
}

pub struct UIButton {
    pub text: UIView,
    pub background: UIView,
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_window_hit_test() {
        let mut window = UIWindow::new("window1".to_string(), ScreenRect::new(10,10,20,20));
        let generic = UIView::new("dunno".to_string(), ScreenRect::new(5,5,5,5));
        window.view.add_child(generic);

        let hit = ScreenPoint::new(15,15);
        let miss = ScreenPoint::new(5,5);

        assert!( window.view.hit_test(&hit) );
        assert!( !window.view.hit_test(&miss) );
    }
}