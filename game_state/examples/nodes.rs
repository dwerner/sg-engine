extern crate game_state;

use std::cell::RefCell;
use std::rc::Rc;

use game_state::state::{ Graph, Node };

fn main () {
    let root_rc = Node::create(0, None);
    // Child consumes ownership of parent as *mut, now lost by lifetimes
    for x in 1..10 {
        let child = Node::create(x, Some(root_rc.clone()));
        root_rc.borrow_mut().add_child(child.clone());
    }

    let pb = root_rc.borrow();
    pb.debug_draw(0);
}