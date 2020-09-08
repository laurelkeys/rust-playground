use std::cell::RefCell;
use std::rc::{Rc, Weak};

// @Note: we want a `Node` to own its children, and we want to share that
// ownership with variables so we can access each `Node` in the tree directly.
// To do this, we define the `Vec<T>` items to be values of type `Rc<Node>`.
//
// We also want to modify which nodes are children of another node, so we have
// a `RefCell<T>` in `children` around the `Vec<Rc<Node>>`.
//
// By making the type of `parent` use `Weak<T>` instead of `Rc<T>`, we avoid a
// reference cycle, as only the parent has ownership over its children, but
// child nodes do not own its parents.

#[derive(Debug)]
struct Node<T> {
    value: T,
    parent: RefCell<Weak<Node<T>>>,
    children: RefCell<Vec<Rc<Node<T>>>>,
}

fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!(
        "`leaf` parent = {:?}",
        leaf.parent.borrow().upgrade() // None
    );
    println!(
        "`leaf` strong = {}, weak = {}",
        Rc::strong_count(&leaf), // 1
        Rc::weak_count(&leaf),   // 0
    );

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!(
            "`leaf` parent = {:?}",
            leaf.parent.borrow().upgrade() // Some(branch)
        );
        println!(
            "`leaf` strong = {}, weak = {}",
            Rc::strong_count(&leaf), // 2
            Rc::weak_count(&leaf),   // 0
        );
        println!(
            "`branch` strong = {}, weak = {}",
            Rc::strong_count(&branch), // 1
            Rc::weak_count(&branch),   // 1
        );
    } // @Note: `branch` is dropped here because its strong count becomes 0.

    println!(
        "`leaf` parent = {:?}",
        leaf.parent.borrow().upgrade() // None
    );
    println!(
        "`leaf` strong = {}, weak = {}",
        Rc::strong_count(&leaf), // 1
        Rc::weak_count(&leaf),   // 0
    );
}
