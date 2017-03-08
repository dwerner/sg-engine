extern crate game_state;

use std::cell::RefCell;
use std::rc::Rc;

use game_state::tree::{ Node };

fn main () {
    let root_rc = Node::create(0, 0, None);
    // Child consumes ownership of parent as *mut, now lost by lifetimes
    for x in 1..10 {
        let child = Node::create(x, 0, Some(root_rc.clone()));
        for y in 1..x {
            let subchild = Node::create(y*10, 0, Some(child.clone()));
        }
    }

    let pb = root_rc.borrow();
    pb.debug_draw(0);
}