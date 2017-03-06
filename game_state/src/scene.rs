use std::rc::{ Rc, Weak };
use std::fmt;
use std::cell::RefCell;

use cgmath::*;

pub struct SceneGraph {
    root: Rc<RefCell<Node>>
}

pub struct Node {
    pub id: u32,
    parent: Option<Weak<RefCell<Node>>>,
    children: Vec<Rc<RefCell<Node>>>,
    local_mat: Matrix4<f32>,
    global_mat: Matrix4<f32>,
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
                Node::new(id, prt, Matrix4::<f32>::identity(), Matrix4::<f32>::identity())
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

    fn new(id: u32, parent: Option<Weak<RefCell<Node>>>, local: Matrix4<f32>, global: Matrix4<f32>) -> Self {
        Node {
            id: id,
            parent: match parent {
                Some(p) => Some(p),
                None => None
            },
            children: Vec::new(),
            local_mat: local,
            global_mat: global
        }
    }

    /// Recursively find if this node is a child of another
    pub fn is_child_of(this: Rc<RefCell<Node>>, parent: Rc<RefCell<Node>>) -> bool {
        match this.borrow().parent() {
            Some(p) => {
                if p.borrow().id == parent.borrow().id {
                    true
                } else {
                    Node::is_child_of(p, parent)
                }
            },
            None => false
        }
    }

    pub fn reparent(child: Rc<RefCell<Node>>, target: Rc<RefCell<Node>>) -> Result<(), String> {
        if child.borrow().id == target.borrow().id {
            return Err("Cannot make node a child of itself.".to_string());
        }
        if  !Node::is_child_of(target.clone(), child.clone()) { // check for cycles
            match child.borrow().parent() {
                Some(old_parent) => {
                    old_parent.borrow_mut().remove_child(child.clone());
                },
                None => {}
            }
            child.borrow_mut().parent = Some(Rc::downgrade(&target.clone()));
            target.borrow_mut().add_child(child.clone());
            Ok(())
        } else {
            Err("Node cycle detected. Child is a parent of reparent target.".to_string()) // format for better debug msg
        }
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

    pub fn siblings(&self) -> Option<Vec<Rc<RefCell<Node>>>> {
        match self.parent() {
            Some(p) => {
                Some(
                    p.borrow().children.iter()
                        .filter(|x| x.borrow().id != self.id)
                        .map(|x| x.clone())
                        .collect()
                )
            },
            None => None
        }
    }

    pub fn children(&mut self) -> &mut Vec<Rc<RefCell<Node>>> {
        &mut self.children
    }

    pub fn is_leaf(&self) -> bool {
        self.children.len() == 0
    }

    fn add_child(&mut self, child: Rc<RefCell<Node>>) {
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
