extern crate game_state;

#[cfg(test)]
mod tests {

    use game_state::tree::{ Node, NodeVisitor, BreadthFirstVisitor, RcNode };

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
        Node::reparent(child.clone(), root2.clone()).unwrap();

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
        let _child = Node::create(0, Some(root.clone()));
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
        let _niece = Node::create(1, Some(cousin.clone()));


        struct SummingVisitor<T> {
            x: T,
            current_node: Option<RcNode<T>>
        }
        impl NodeVisitor<u32> for SummingVisitor<u32> {
            fn visit<F: FnMut(&u32)->()>(&mut self, mut func: F) {
                let (val, maybe_parent) = match self.current_node {
                    Some(ref n) => {
                        (n.borrow().data, n.borrow().parent())
                    },
                    None => (0, None)
                };
                self.x += val;
                (func)(&self.x);
                self.current_node = maybe_parent;
            }
            fn has_next(&self) -> bool {
                let maybe_parent = match self.current_node {
                    Some(ref n) => {
                        n.borrow().parent()
                    },
                    None => None
                };
                maybe_parent.is_some()
            }
        }

        let mut v = SummingVisitor {
            x: 0,
            current_node: Some(grandchild.clone())
        };

        while v.has_next() {
            v.visit(|x| assert!(*x > 0));
        }

        assert_eq!(v.x, 10);
    }

    #[test]
    fn breadth_first_visitor() {

        // master branch
        let root = Node::create(5u32, None);
        let grandparent = Node::create(4, Some(root.clone()));
        let parent = Node::create(3, Some(grandparent.clone()));
        let child = Node::create(2, Some(parent.clone()));
        let _grandchild = Node::create(1, Some(child.clone()));

        // misfits. Sibling branch to those in master
        let great_aunt = Node::create(4, Some(root.clone()));
        let uncle = Node::create(3, Some(great_aunt.clone()));
        let cousin = Node::create(2, Some(uncle.clone()));
        let _niece = Node::create(1, Some(cousin.clone()));

        let mut visitor = BreadthFirstVisitor::new(root.clone());

        let mut counter = 0;
        let mut loop_ctr = 0;
        while visitor.has_next() {
            loop_ctr += 1;
            visitor.visit(|x| counter += *x );
        }
        assert_eq!(loop_ctr, 9);
        assert_eq!(counter, 25);
    }
}
