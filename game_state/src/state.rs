use super::{ Renderer, Renderable }; //, Physical, Syncable, Identifyable };
use std::rc::Rc;

use std::cell::RefCell;


//use cgmath::Matrix;

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

impl Drop for GameObj {
    fn drop(&mut self) {
        println!("Dropping {}", self.id);
    }
}

impl GameObj {

    fn new(id: u32, parent: Option<Box<GameObj>>) -> Self {
        GameObj {
            id: id,
            parent: match parent {
                Some(p) => Some(Box::into_raw(p)),
                None => None
            },
            children: Vec::new(),
            // local_mat: Matrix<f32>::one(),
            // global_mat: Matrix<f32>::one()
        }
    }

    fn reparent(&mut self, parent: Option<Box<GameObj>>) {
        self.parent = match parent {
            Some(p) => {
                Some(Box::into_raw(p))
            },
            None => None
        };
    }

    fn find_child(&self, id: u32) -> Option<&Box<GameObj>> {
        let option: Option<&Box<GameObj>> = self.children.iter().find(|ref x| x.id == id);
        match option {
            Some(obj_ref) => Some(obj_ref.clone()),
            None => None
        }
    }

    fn children(&mut self) -> &mut Vec<Box<GameObj>> {
        &mut self.children
    }

    fn add_child(&mut self, child: Box<GameObj>) {
        self.children.push(child);
    }

    fn parent(&self) -> Option<Box<GameObj>> {
        match self.parent {
            Some(p) => Some(unsafe { Box::from_raw(p) }),
            None => None
        }
    }

    //fn root(&self) -> Box<GameObj> {
    //}

}

#[test]
fn do_shit_with_gameobjects () {
    let mut root = GameObj::new(0, None);
    let mut root_ptr = Box::new(root);

    // Child consumes ownership of parent as *mut, now lost by lifetimes
    let child = GameObj::new(42, Some(root_ptr));
    let child_ptr = Box::new(child);

    // Gather the *mut lost to the child and own it here
    let mut parent = child_ptr.parent().unwrap();

    // parent consumes ownership of child
    parent.add_child(child_ptr);

    let found_child: &Box<GameObj> = parent.find_child(42).unwrap();
    assert!(found_child.id == 42);
}

#[test]
fn do_shit_with_gameobjects2 () {
    let mut root = GameObj::new(0, None);
    let mut root_ptr = Box::new(root);

    // Child consumes ownership of parent as *mut, now lost by lifetimes
    let child = GameObj::new(42, Some(root_ptr));
    let child_ptr = Box::new(child);

    // Gather the *mut lost to the child and own it here
    let mut parent = child_ptr.parent().unwrap();

    // parent consumes ownership of child
    parent.children.push(child_ptr);

    let found_child: &Box<GameObj> = parent.find_child(42).unwrap();
    assert!(found_child.id == 42);
}
