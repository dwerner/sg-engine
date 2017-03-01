use super::{ Renderer, Renderable }; //, Physical, Syncable, Identifyable };
use std::rc::Rc;

use std::cell::RefCell;


use cgmath::Matrix;

pub struct State {
    pub renderers: Vec<Box<Renderer>>,
    pub renderables: Vec<Box<Renderable>>,
    pub blob: u64,
}
impl State {}

pub struct GameObj { // le cliche classname
    id: u32,
    parent: Option<*mut GameObj>,
    children: Vec<Box<GameObj>>,
    // TODO: figure out type signatures
 //   local_mat: Matrix<f32>,
 //  global_mat: Matrix<f32>,
}

impl GameObj {

    fn new(id: u32, parent: Option<*mut GameObj>) -> Self {
        GameObj {
            id: id,
            parent: parent,
            children: Vec::new(),
            // local_mat: Matrix<f32>::one(),
            // global_mat: Matrix<f32>::one()
        }
    }

    fn find_child(&self, id: u32) -> Option<&Box<GameObj>> {
        let option: Option<&Box<GameObj>> = self.children.iter().find(|ref x| x.id == id);
        match option {
            Some(obj_ref) => Some(obj_ref.clone()),
            None => None
        }
    }

    fn add_child(&mut self, child: Box<GameObj>) {
        self.children.push(child);
    }
}

#[test]
fn do_shit_with_gameobjects () {
    let mut root = GameObj::new(0, None);
    let mut root_ptr = Box::new(root);

    let child = GameObj::new(42, Some(Box::into_raw(root_ptr)));
    let child_ptr = Box::new(child);

    let mut parent = unsafe {
        Box::from_raw(child_ptr.parent.unwrap())
    };
    parent.add_child(child_ptr);

    let found_child = parent.find_child(42).unwrap();
    assert!(found_child.id == 42);
}

