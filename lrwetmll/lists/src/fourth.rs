// https://rust-unofficial.github.io/too-many-lists/fourth.html

use std::cell::{Ref, RefCell, RefMut};
use std::rc::Rc;

// @Note: 'RefCell' is used to enforce borrow rules at runtime.
//
// See https://doc.rust-lang.org/std/cell/index.html

pub struct List<T> {
    head: Link<T>,
    tail: Link<T>,
}

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
    prev: Link<T>,
}

impl<T> Node<T> {
    fn new(elem: T) -> Rc<RefCell<Node<T>>> {
        Rc::new(RefCell::new(Node {
            elem: elem,
            next: None,
            prev: None,
        }))
    }
}

impl<T> List<T> {
    pub fn new() -> Self {
        List {
            head: None,
            tail: None,
        }
    }

    pub fn push_front(&mut self, elem: T) {
        let new_head = Node::new(elem);

        match self.head.take() {
            Some(old_head) => {
                old_head.borrow_mut().prev = Some(new_head.clone());
                new_head.borrow_mut().next = Some(old_head);
                self.head = Some(new_head);
            }
            None => {
                self.tail = Some(new_head.clone());
                self.head = Some(new_head);
            }
        }
    }

    pub fn push_back(&mut self, elem: T) {
        let new_tail = Node::new(elem);

        match self.tail.take() {
            Some(old_tail) => {
                old_tail.borrow_mut().next = Some(new_tail.clone());
                new_tail.borrow_mut().prev = Some(old_tail);
                self.tail = Some(new_tail);
            }
            None => {
                self.head = Some(new_tail.clone());
                self.tail = Some(new_tail);
            }
        }
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|old_head| {
            match old_head.borrow_mut().next.take() {
                Some(new_head) => {
                    // Not emptying list.
                    new_head.borrow_mut().prev.take();
                    self.head = Some(new_head);
                }
                None => {
                    // Emptying list.
                    self.tail.take();
                }
            }
            // @Note: we use `ok()` below to convert the 'Result' into an 'Option'
            // because calling `unwrap()` on it requires that we can debug-print the error case,
            // but `RefCell<T>` only implements `Debug` if `T` does (and `Node` does not).
            Rc::try_unwrap(old_head).ok().unwrap().into_inner().elem
        })
    }

    pub fn pop_back(&mut self) -> Option<T> {
        self.tail.take().map(|old_tail| {
            match old_tail.borrow_mut().prev.take() {
                Some(new_tail) => {
                    // Not emptying list.
                    new_tail.borrow_mut().next.take();
                    self.tail = Some(new_tail);
                }
                None => {
                    // Emptying list.
                    self.head.take();
                }
            }
            // @Note: we use `ok()` for the same reason mentioned in `pop_front()`, above.
            Rc::try_unwrap(old_tail).ok().unwrap().into_inner().elem
        })
    }

    pub fn peek_front(&self) -> Option<Ref<T>> {
        self.head
            .as_ref()
            .map(|node| Ref::map(node.borrow(), |node| &node.elem))
    }

    pub fn peek_back(&self) -> Option<Ref<T>> {
        self.tail
            .as_ref()
            .map(|node| Ref::map(node.borrow(), |node| &node.elem))
    }

    pub fn peek_front_mut(&mut self) -> Option<RefMut<T>> {
        self.head
            .as_mut()
            .map(|node| RefMut::map(node.borrow_mut(), |node| &mut node.elem))
    }

    pub fn peek_back_mut(&mut self) -> Option<RefMut<T>> {
        self.tail
            .as_mut()
            .map(|node| RefMut::map(node.borrow_mut(), |node| &mut node.elem))
    }
}

//
// Drop trait.
//

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        while self.pop_front().is_some() {}
    }
}

//
// Iterator trait.
//

pub struct IntoIter<T>(List<T>);

impl<T> List<T> {
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        self.0.pop_front()
    }
}

impl<T> DoubleEndedIterator for IntoIter<T> {
    fn next_back(&mut self) -> Option<T> {
        self.0.pop_back()
    }
}

// @Note: we've only implemented `IntoIter`, not `Iter` and `IterMut`.
//
// See https://rust-unofficial.github.io/too-many-lists/fourth-iteration.html#iter

//
// Test functions.
//

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
        let mut list = List::new();

        // Front operations.
        assert_eq!(list.pop_front(), None);

        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.pop_front(), Some(2));

        list.push_front(4);
        list.push_front(5);

        assert_eq!(list.pop_front(), Some(5));
        assert_eq!(list.pop_front(), Some(4));

        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), None);

        // Back operations.
        assert_eq!(list.pop_back(), None);

        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        assert_eq!(list.pop_back(), Some(3));
        assert_eq!(list.pop_back(), Some(2));

        list.push_back(4);
        list.push_back(5);

        assert_eq!(list.pop_back(), Some(5));
        assert_eq!(list.pop_back(), Some(4));

        assert_eq!(list.pop_back(), Some(1));
        assert_eq!(list.pop_back(), None);
    }

    #[test]
    fn peek() {
        let mut list = List::new();

        assert!(list.peek_front().is_none());
        assert!(list.peek_back().is_none());
        assert!(list.peek_front_mut().is_none());
        assert!(list.peek_back_mut().is_none());

        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        assert_eq!(&*list.peek_front().unwrap(), &3);
        assert_eq!(&mut *list.peek_front_mut().unwrap(), &mut 3);
        assert_eq!(&*list.peek_back().unwrap(), &1);
        assert_eq!(&mut *list.peek_back_mut().unwrap(), &mut 1);
        // @Note: while removing the `&`s (and `&mut`s) above would still make
        // the tests pass, it wouldn't be asserting whether or not the references
        // we get are mutable (exclusive) or shared.
        //
        // We can see this by noting that, while all the following would pass
        // for `peek_front_mut()`, or `peek_back_mut()`:
        //  |
        //  | assert_eq!(&mut *list.peek_front_mut().unwrap(), &mut 3);
        //  | assert_eq!(&*list.peek_front_mut().unwrap(), &3);
        //  | assert_eq!(*list.peek_front_mut().unwrap(), 3);
        //
        //  Only these are valid for the non-`_mut` versions:
        //  |
        //  | assert_eq!(&*list.peek_front().unwrap(), &3);
        //  | assert_eq!(*list.peek_front().unwrap(), 3);
        //
        //  i.e. trying to use `&mut` leads to a "cannot borrow as mutable" error:
        //  |
        //  | assert_eq!(&mut *list.peek_front().unwrap(), &mut 3);
        //
    }

    #[test]
    fn into_iter() {
        let mut list = List::new();

        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        let mut iter = list.into_iter();

        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next_back(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next_back(), None);
        assert_eq!(iter.next(), None);
    }
}
