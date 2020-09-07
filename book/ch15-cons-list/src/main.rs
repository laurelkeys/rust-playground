use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
enum List<T> {
    Cons(T, Rc<List<T>>),
    Nil,
}

// @Note: because `Rc<T>` holds only immutable values, we can't change any
// of the values in a `List` once we've created them. By adding in `RefCell<T>`
// we gain the ability to change the values of `ListMutHead`, while wrapping it with
// `Rc<_>` still allows us to have multiple lists sharing ownership of another list.

#[derive(Debug)]
enum ListMutHead<T> {
    Cons(Rc<RefCell<T>>, Rc<ListMutHead<T>>),
    Nil,
}

// @Note: in the following definition, the second element in the `Cons` variant is
// now wrapped by `RefCell<Rc<_>>`, meaning that instead of having the ability to
// modify the `T` value as we did in `ListMutHead`, we want to modify which `ListMutTail`
// value a `Cons` variant is pointing to.

#[derive(Debug)]
enum ListMutTail<T> {
    Cons(T, RefCell<Rc<ListMutTail<T>>>),
    Nil,
}

impl<T> ListMutTail<T> {
    fn tail(&self) -> Option<&RefCell<Rc<ListMutTail<T>>>> {
        use crate::ListMutTail::{Cons, Nil};

        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

#[allow(unused_variables)]
fn main() {
    // List with immutable values.
    {
        use crate::List::{Cons, Nil};

        // By storing `Rc` instead of `Box` inside `List<T>`, we can create
        // two lists, `b` and `c`, that share ownership of a third list, `a`:
        //
        //      b ---> [3|\]
        //                 \
        //                  \
        //              a ---> [5|-]--> [10|-]--> [Nil]
        //                  /
        //                 /
        //      c ---> [4|/]
        //

        // @Note: we wrap the list `a` in an `Rc<T>` so when we create lists
        // `b` and `c`, they can both refer to `a`.
        let a: Rc<List<_>> = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
        println!("count after creating `a` = {}", Rc::strong_count(&a));

        // @Note: instead of taking ownership of `a`, we clone the `Rc<List>`
        // it's holding, increasing the number of references from 1 to 2.
        let b: List<_> = Cons(3, Rc::clone(&a));
        println!("count after creating `b` = {}", Rc::strong_count(&a));

        {
            // @Note: we also clone `a` when creating `c`, increasing the number
            // of references from 2 to 3, letting they share ownership of the data.
            let c: List<_> = Cons(4, Rc::clone(&a));
            println!("count after creating `c` = {}", Rc::strong_count(&a));
        }

        println!(
            "count after `c` goes out of scope = {}",
            Rc::strong_count(&a)
        );
    }

    println!("---");

    // List with (inwardly) mutable Cons' head values.
    {
        use crate::ListMutHead::{Cons, Nil};

        let value: Rc<RefCell<_>> = Rc::new(RefCell::new(5));

        // @Note: we need to clone `value` so both it and `a` have ownership
        // of the inner `5` value, rather than transferring ownership or
        // having `a` borrow from `value`.
        let a: Rc<ListMutHead<_>> = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

        let b: ListMutHead<_> = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
        let c: ListMutHead<_> = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

        // @Note: calling `borrow_mut` on `value` uses Rust's automatic dereferencing
        // feature to dereference the `Rc<T>` to the inner `RefCell<T>` value.
        *value.borrow_mut() += 10;

        println!("`a` after = {:?}", a);
        println!("`b` after = {:?}", b);
        println!("`c` after = {:?}", c);
    }

    println!("---");

    // List with (inwardly) mutable Cons' tail values.
    {
        use crate::ListMutTail::{Cons, Nil};

        // a ---> [5|-]--> [Nil]
        let a: Rc<ListMutTail<_>> = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

        println!("`a` initial rc count = {}", Rc::strong_count(&a));
        println!("`a` next item = {:?}", a.tail());

        // b ---> [10|-]--> a ---> [5|-]--> [Nil]
        let b: Rc<ListMutTail<_>> = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

        println!("`a` rc count after `b` creation = {}", Rc::strong_count(&a));
        println!("`b` initial rc count = {}", Rc::strong_count(&b));
        println!("`b` next item = {:?}", b.tail());

        // ┌─> b ---> [10|-]--> a ---> [5|─]─┐
        // └─────────────────────────────────┘
        if let Some(link) = a.tail() {
            // @Note: we use `borrow_mut` on the `RefCell<Rc<ListMutTail>>`
            // to change the value inside from an `Rc<ListMutTail>` that holds
            // a `Nil` value to the `Rc<ListMutTail>` in `b`, creating a cycle.
            *link.borrow_mut() = Rc::clone(&b);
        }

        println!("`b` rc count after changing `a` = {}", Rc::strong_count(&b));
        println!("`a` rc count after changing `a` = {}", Rc::strong_count(&a));

        // Add the next line to see that we have a cycle; it will overflow the stack:
        //  |
        //  |   println!("`a` next item = {:?}", a.tail());
        //

        // @Note: since there is a reference cycle between `a` and `b`, when we get
        // out of this scope Rust will try to drop `b` first, which will decrease the
        // count of the `Rc<ListMutTail>` instance in `b` by 1.
        //
        // However, because `a` is still referencing the `Rc<ListMutTail>` that was in `b`,
        // that `Rc<ListMutTail>` has a count of 1 rather than 0, so the memory the
        // `Rc<ListMutTail>` has on the heap won't be dropped.
        //
        // The memory will just sit there with a count of 1, forever.
    }
}
