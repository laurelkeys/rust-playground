// https://rust-unofficial.github.io/too-many-lists/second.html

pub struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn push(&mut self, elem: T) {
        let new_node = Box::new(Node {
            elem: elem,
            next: self.head.take(),
        });

        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.elem)
    }
}

//
// Drop trait.
//

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut curr_link = self.head.take();

        while let Some(mut boxed_node) = curr_link {
            curr_link = boxed_node.next.take();
        }
    }
}

//
// Iterator trait.
//

// @Note: there are 3 kinds of iterator:
//  * IntoIter - `T`        owned data (value)
//  * IterMut - `&mut T`    mutably borrowed data (exclusive reference)
//  * Iter - `&T`           immutably borrowed data (shared reference)
//
// See https://rust-lang.github.io/api-guidelines/naming.html (C-ITER)

// IntoIter - `T`

pub struct IntoIter<T>(List<T>);

impl<T> List<T> {
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

// Iter - `&T`

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<T> List<T> {
    // @Note: lifetime elision lets us avoid having to write this:
    //  |
    //  |   pub fn iter<'a>(&'a self) -> Iter<'a, T> {
    //
    // which could also be made more explicit, by using:
    //  |
    //  |   pub fn iter(&self) -> Iter<'_, T> {
    //
    pub fn iter(&self) -> Iter<T> {
        Iter {
            next: self.head.as_ref().map(|node| &**node),
        }
        // @Note: we could give the following hint for the compiler, so that it does an
        // implict conversion (applying "deref coercion"), thus letting us remove the `**`
        //  |
        //  |   Iter {
        //  |       next: self.head.as_ref().map::<&Node<T>, _>(|node| &node),
        //  |   }
        //
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    // @Note: the signature can be "desugarred" as shown below,
    // meaning that, there are no lifetime constraints between
    // the input and the output for the `next` function:
    //  |
    //  |   fn next<'b>(&'b mut self) -> Option<&'a T> {
    //
    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_ref().map(|node| &**node);
            &node.elem
        })
        // @Note: same as for `iter` above, this would let us omit the `**` dereferencing:
        //  |
        //  |   self.next.map(|node| {
        //  |       self.next = node.next.as_ref().map::<&Node<T>, _>(|node| &node);
        //  |       &node.elem
        //  |   })
        //
    }
}

// IterMut - `&mut T`

pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

impl<T> List<T> {
    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut {
            next: self.head.as_mut().map(|node| &mut **node),
        }
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_mut().map(|node| &mut **node);
            &mut node.elem
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

        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        list.push(4);
        list.push(5);

        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn peek() {
        let mut list = List::new();

        assert_eq!(list.peek(), None);
        assert_eq!(list.peek_mut(), None);

        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.peek(), Some(&3));
        assert_eq!(list.peek_mut(), Some(&mut 3));

        list.peek_mut().map(|value| *value = 42);

        assert_eq!(list.peek(), Some(&42));
        assert_eq!(list.pop(), Some(42));
    }

    #[test]
    fn into_iter() {
        let mut list = List::new();

        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.into_iter();

        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter() {
        let mut list = List::new();

        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.iter();

        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
    }

    #[test]
    fn iter_mut() {
        let mut list = List::new();

        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.iter_mut();

        assert_eq!(iter.next(), Some(&mut 3));
        assert_eq!(iter.next(), Some(&mut 2));
        assert_eq!(iter.next(), Some(&mut 1));
    }
}
