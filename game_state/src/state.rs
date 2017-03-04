use super::{ Renderer, Renderable }; //, Physical, Syncable, Identifyable };
use std::rc::{ Rc, Weak };
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
    root: Rc<RefCell<Node>>
}

pub struct Node { // le cliche classname
    id: u32,
    parent: Option<Weak<RefCell<Node>>>,
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

    pub fn create(id: u32, parent: Option<Rc<RefCell<Node>>>) -> Rc<RefCell<Node>> {
        let prt = match parent {
            Some(ref p) => {
                Some(Rc::downgrade(p))
            },
            None => None
        };

        let node = Rc::new(
            RefCell::new(
                Node::new(id, prt)
            )
        );

        match parent {
            Some(ref p) => {
                p.borrow_mut().add_child(node.clone());
            },
            None => {}
        };

        node
    }

    pub fn find_root(node: Rc<RefCell<Node>>) -> Rc<RefCell<Node>> {
        match node.borrow().parent() {
            Some(p) => Node::find_root(p.clone()),
            None => return node.clone()
        }
    }

    fn new(id: u32, parent: Option<Weak<RefCell<Node>>>) -> Self {
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

    pub fn reparent(child: Rc<RefCell<Node>>, to: Rc<RefCell<Node>>) {
        match child.borrow().parent() {
            Some(old_parent) => {
                old_parent.borrow_mut().remove_child(child.clone());
            },
            None => {}
        }
        child.borrow_mut().parent = Some(Rc::downgrade(&to.clone()));
        to.borrow_mut().add_child(child.clone());
    }

    pub fn remove_child(&mut self, child: Rc<RefCell<Node>>) {
        let mut idx: Option<usize> = None;
        for i in 0usize..self.children.len() {
            let child_id = self.children[i].borrow().id;
            if child_id == child.borrow().id {
                idx = Some(i);
                break;
            }
        }
        match idx {
           Some(i) => {
               self.children.remove(i);
           },
           _ => {}
        };
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
        if self.id != child.borrow().id {
            self.children.push(child);
        }
    }

    pub fn parent(&self) -> Option<Rc<RefCell<Node>>> {
        match self.parent {
            Some(ref p) => Some(p.upgrade().unwrap()),
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

}

#[test]
fn traverse_nodes() {
    let root_ptr = Node::create(0, None);
    let child_ptr = Node::create(42, Some(root_ptr.clone()));

    let parent: Rc<RefCell<Node>> = child_ptr.borrow().parent().unwrap();
    parent.borrow_mut().add_child(child_ptr);

    let found_child = parent.borrow().find_child(42).unwrap();
    assert!(found_child.borrow().id == 42);
}

#[test]
fn find_root() {
    let root = Node::create(0, None);
    let child = Node::create(1, Some(root.clone()));
    let found_root = Node::find_root(child);
    assert!(root.borrow().id == found_root.borrow().id);
}

#[test]
fn reparentable() {
    let root = Node::create(0, None);

    let child = Node::create(2, Some(root.clone()));
    let original_child = root.borrow().find_child(2);
    assert!(original_child.is_some());

    let root2 = Node::create(1, None);
    Node::reparent(child.clone(), root2.clone());

    let found_root = Node::find_root(child);
    assert!(root2.borrow().id == found_root.borrow().id);

    let stale_child = root.borrow().find_child(2);
    assert!(stale_child.is_none());

    let actual_child = root2.borrow().find_child(2);
    assert!(actual_child.is_some());
}
