extern crate game_state;

#[cfg(test)]
mod tests {

    use game_state::tree::{ Node, NodeVisitor, RcNode };

    use std::rc::{ Rc, Weak };
    use std::cell::RefCell;


    #[test]
    fn traverse_nodes() {
        let root_ptr = Node::create(0, None);
        let child_ptr = Node::create(0, Some(root_ptr.clone()));
        let found_child = root_ptr.borrow().find_child(child_ptr.borrow().id);
        assert!(found_child.is_some());
    }

    #[test]
    fn find_root() {
        let root = Node::create(0, None);
        let child = Node::create(0, Some(root.clone()));
        let found_root = Node::find_root(child.clone());
        assert!(root.borrow().id == found_root.borrow().id);
    }

    #[test]
    fn is_child_of() {
        let root = Node::create(0, None);
        let child = Node::create(0, Some(root.clone()));
        assert!(Node::is_child_of(child.clone(), root.clone()));
        let r = Node::create(0, None);
        assert!( !Node::is_child_of(child.clone(), r.clone()) );
    }

    #[test]
    fn reparent() {
        let root = Node::create(0,None);

        let child = Node::create(0, Some(root.clone()));
        let original_child = root.borrow().find_child(child.borrow().id);
        assert!(original_child.is_some());

        let root2 = Node::create(0, None);
        Node::reparent(child.clone(), root2.clone());

        let found_root = Node::find_root(child.clone());
        assert!(root2.borrow().id == found_root.borrow().id);

        let stale_child = root.borrow().find_child(child.borrow().id);
        assert!(stale_child.is_none());

        let actual_child = root2.borrow().find_child(child.borrow().id);
        assert!(actual_child.is_some());
    }

    #[test]
    fn is_child_of_reparent() {
        let root = Node::create(0, None);
        let im = Node::create(0, Some(root.clone()));
        let child = Node::create(0, Some(im.clone()));
        assert!( Node::is_child_of(child.clone(), im.clone()) );
        assert!( Node::is_child_of(child.clone(), root.clone()) );

        let root2 = Node::create(0, None);
        let result = Node::reparent(im.clone(), root2.clone());

        assert!( result.is_ok() );
        assert!( Node::is_child_of(child.clone(), root2.clone()) );
    }


    #[test]
    fn fails_to_reparent_to_self() {
        let root = Node::create(0, None);
        let result = Node::reparent(root.clone(), root.clone());
        assert!( result.is_err() );
    }

    #[test]
    fn fails_to_reparent_causing_a_cycle() {
        let root = Node::create(0, None);
        let child = Node::create(0, Some(root.clone()));
        let sibling = Node::create(0, Some(root.clone()));

        let result = Node::reparent(root.clone(), sibling.clone());
        assert!( result.is_err() );
    }

    #[test]
    fn siblings_as_expected() {
        let root = Node::create(0, None);
        let child = Node::create(0, Some(root.clone()));
        let sibling = Node::create(0, Some(root.clone()));

        let maybe_siblings = child.borrow().siblings();
        let siblings = maybe_siblings.unwrap();
        assert!( siblings.len() == 1 );
        assert!( sibling.borrow().id == siblings[0].borrow().id );
    }

    #[test]
    fn visitor() {

        // master branch
        let root = Node::create(5u32, None);
        let grandparent = Node::create(4, Some(root.clone()));
        let parent = Node::create(3, Some(grandparent.clone()));
        let child = Node::create(2, Some(parent.clone()));
        let grandchild = Node::create(1, Some(child.clone()));

        // misfits. Sibling branch to those in master
        let great_aunt = Node::create(4, Some(root.clone()));
        let uncle = Node::create(3, Some(great_aunt.clone()));
        let cousin = Node::create(2, Some(uncle.clone()));
        let niece = Node::create(1, Some(cousin.clone()));


        struct SummingVisitor {
            x: u32,
            current_node: Option<RcNode<u32>>
        }
        impl NodeVisitor<u32> for SummingVisitor {
            fn visit(&mut self) {
                self.x += match self.current_node {
                    Some(ref n) => n.borrow().data,
                    None => 0
                }
            }
            fn next(&mut self) -> bool {
                let maybe_parent = match self.current_node {
                    Some(ref n) => {
                        n.borrow().parent()
                    },
                    None => None
                };
                self.current_node = maybe_parent;
                self.current_node.is_some()
            }
        }

        let mut v = SummingVisitor {
            x: 0,
            current_node: Some(grandchild.clone())
        };

        loop {
            v.visit();
            if !v.next() { break; }
        }

        assert_eq!(v.x, 15);
    }
}
