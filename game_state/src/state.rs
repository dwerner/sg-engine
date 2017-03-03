use super::{ Renderer, Renderable }; //, Physical, Syncable, Identifyable };
use std::rc::Rc;
use std::fmt;
use std::cell::RefCell;


//use cgmath::Matrix;

pub struct State {
    pub renderers: Vec<Box<Renderer>>,
    pub renderables: Vec<Box<Renderable>>,
    pub blob: u64,
}
impl State {}

pub struct Graph {
    root: Box<Node>
}

pub struct Node { // le cliche classname
    id: u32,
    parent: Option<Rc<RefCell<Node>>>,
    children: Vec<Rc<RefCell<Node>>>,
    // TODO: figure out type signatures
 //   local_mat: Matrix<f32>,
 //  global_mat: Matrix<f32>,
}

impl Drop for Node {
    fn drop(&mut self) {
        println!("Dropping {}", self.id);
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let p = match self.parent {
            Some(_) => "*",
            None => "*root"
        };
        write!(f, "{} ->(id: {})", p, self.id);
        Ok(())
    }
}

impl Node {

    fn new(id: u32, parent: Option<Rc<RefCell<Node>>>) -> Self {
        Node {
            id: id,
            parent: match parent {
                Some(p) => Some(p),
                None => None
            },
            children: Vec::new(),
            // local_mat: Matrix<f32>::one(),
            // global_mat: Matrix<f32>::one()
        }
    }

    pub fn create(id: u32, parent: Option<Rc<RefCell<Node>>>) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node::new(id, parent)))
    }

    // FIX THIS - needs to add to children etc
    pub fn reparent(&mut self, parent: Rc<RefCell<Node>>) {
        self.parent = Some(parent);
    }

    pub fn find_child(&self, id: u32) -> Option<Rc<RefCell<Node>>> {
        let option: Option<&Rc<RefCell<Node>>> =
            self.children.iter().find(|x| x.borrow().id == id);
        match option {
            Some(obj_ref) => Some(obj_ref.clone()),
            None => None
        }
    }

    pub fn children(&mut self) -> &mut Vec<Rc<RefCell<Node>>> {
        &mut self.children
    }

    pub fn add_child(&mut self, child: Rc<RefCell<Node>>) {
        self.children.push(child);
    }


    pub fn parent(&self) -> Option<Rc<RefCell<Node>>> {
        match self.parent {
            Some(ref p) => Some(p.clone()),
            None => None
        }
    }

    pub fn debug_draw(&self, lvl: u32) {
        if lvl == 0 {
            println!("-- Hierarchy Dump --");
        }
        let c = if self.children.len() > 0 {
            "..."
        } else {
            ".leaf*"
        };
        println!(
            "{}{}{}",
            (0..lvl).map(|_| "....").collect::<String>(),
            self,
            c
        );
        for child in &self.children {
            child.borrow().debug_draw(lvl+1);
        }
    }

    //fn root(&self) -> Box<Node> {
    //}

}

#[test]
fn traverse_nodes () {
    let root_ptr = Node::create(0, None);
    let child_ptr = Node::create(42, Some(root_ptr.clone()));

    let mut parent = &child_ptr.borrow().parent().unwrap();

    &parent.borrow_mut().add_child(child_ptr);

    let found_child = &parent.borrow().find_child(42).unwrap();
    assert!(found_child.borrow().id == 42);
}
