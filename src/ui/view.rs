use input::screen::{ScreenPoint, ScreenRect};

use std::rc::{ Weak, Rc };
use std::cell::RefCell;


pub struct View {
    pub children: Vec<View>,
    bounds: ScreenRect
}
impl View {
    pub fn new(bounds: ScreenRect) -> Self {
        View {
            children: Vec::new(),
            bounds: bounds
        }
    }

    fn add_child(&mut self, child: View) { self.children.push(child); }
    fn get_children(&self) -> &Vec<View> { &self.children }
    fn get_child_by_id(&self, id: u32) -> Option<&View> { None }

    fn hit_test(&self, point: &ScreenPoint) -> bool {
        point.x >= self.bounds.x &&
            point.x < self.bounds.x + self.bounds.w &&
            point.y >= self.bounds.y &&
            point.y < self.bounds.y + self.bounds.h
    }
    fn get_bounds(&self) -> &ScreenRect { &self.bounds }
    fn move_to_point(&mut self, point: &ScreenPoint) {
        self.bounds.x = point.x;
        self.bounds.y = point.y;
    }
}

pub struct Window {
    pub view: View,
}
impl Window {
    pub fn new(bounds: ScreenRect) -> Self {
        Window{ view: View::new(bounds) }
    }
}

pub struct Button {
    pub text: View,
    pub background: View,
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_ui_composition() {}
}