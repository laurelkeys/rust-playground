use std::rc::Rc;

#[derive(Debug)]
enum List<T> {
    Cons(T, Rc<List<T>>),
    Nil,
}

#[allow(unused_variables)]
fn main() {
    use crate::List::{Cons, Nil};

    // By storing `Rc` instead of `Box` inside `List<T>`, we can create
    // two lists, `b` and `c`, that share ownership of a third list, `a`:
    //
    //      b ---> [3|\]
    //                 \
    //                  \
    //              a --->[5|-]-->[10|-]-->[Nil]
    //                  /
    //                 /
    //      c ---> [4|/]
    //
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

    println!("count after `c` goes out of scope = {}", Rc::strong_count(&a));
}
