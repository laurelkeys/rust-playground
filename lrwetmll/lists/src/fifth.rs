// https://rust-unofficial.github.io/too-many-lists/fifth.html
//
// @Note: the first section of this chapter (6.1. Layout) is really
// interesting as it does.. then "undoes".. many design decisions.
// Worth reading it again.

use std::ptr;

pub struct List<T> {
    head: Link<T>,
    tail: *mut Node<T>, // @Note: stored to push a new element in O(1)
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List {
            head: None,
            tail: ptr::null_mut(),
        }
    }

    pub fn push(&mut self, elem: T) {
        let mut new_tail = Box::new(Node {
            elem: elem,
            next: None, // @Note: this is always None since we push onto the tail
        });

        let raw_tail: *mut _ = &mut *new_tail; // @Note: `&mut T` coerces into `*mut T` (pointer weakening)

        if !self.tail.is_null() {
            // @Safety: `tail` is not NULL, so it's safe to dereference it.
            unsafe {
                (*self.tail).next = Some(new_tail);
            }
        } else {
            self.head = Some(new_tail);
        }

        self.tail = raw_tail;
    }

    pub fn pop(&mut self) -> Option<T> {
        // @Note: this is a queue, so, because we are pushing
        // onto the tail, pop needs to grab the current head.

        self.head.take().map(|head| {
            let head = *head;
            self.head = head.next;
            if self.head.is_none() {
                self.tail = ptr::null_mut();
            }

            head.elem
        })
    }
}

//
// Test functions.
//

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
        let mut list = List::new();

        assert_eq!(list.pop(), None);

        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), Some(2));

        list.push(4);
        list.push(5);

        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(4));

        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), None);

        list.push(6);
        list.push(7);

        assert_eq!(list.pop(), Some(6));
        assert_eq!(list.pop(), Some(7));
        assert_eq!(list.pop(), None);
    }
}
