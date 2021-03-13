#![feature(dropck_eyepatch)] // $ rustup override set nightly

use std::marker::PhantomData;

pub struct Boks<T> {
    p: *mut T,
    _t: PhantomData<T>,
}

// @Note: by using `#[may_dangle] T` we "promisse" to the compiler that we do not
// access the T inside of `drop`. However, we do drop a T, so to ensure the
// compiler knows that we now have to introduce a `PhantomData<T>`.

unsafe impl<#[may_dangle] T> Drop for Boks<T> {
    fn drop(&mut self) {
        // @Note: using `from_raw` we create a Box that owns the raw pointer `p`,
        // which is then immediatly dropped at the end of the unsafe block, leading
        // its destructor to call the destructor of T and free the allocated memory.
        //
        // @Safety: `p` was constructed from a Box in the first place, and has not been
        // freed since `self` still exists (otherwise, `drop` could not be called).
        unsafe { Box::from_raw(self.p) };
    }
}

impl<T> Boks<T> {
    pub fn new(t: T) -> Self {
        Boks {
            p: Box::into_raw(Box::new(t)),
            _t: PhantomData,
        }
    }
}

impl<T> std::ops::Deref for Boks<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        // @Safety: is valid since it was constructed from a valid T
        // and turned into a Box which creates aligned pointers
        // (and hasn't been freed since `self` is alive).
        unsafe { &*self.p }
    }
}

impl<T> std::ops::DerefMut for Boks<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // @Safety: same argument above applies (see `impl<T> std::ops::Deref for Boks<T>`).
        // Also, since we have `&mut self`, no other mutable reference to `p` has been given out.
        unsafe { &mut *self.p }
    }
}

use std::fmt::Debug;
struct Oisann<T: Debug>(T);

impl<T: Debug> Drop for Oisann<T> {
    fn drop(&mut self) {
        println!("{:?}", self.0);
    }
}

fn main() {
    let x = 42;
    let b = Boks::new(x);
    println!("{:?}", *b);

    // @Note: since we used `#[may_dangle] T` in the implementation of
    // Drop  for Boks, the compiler can shorten the borrow of `y` and
    // allow us to use it in the call to `println!`.
    let mut y = 42;
    let b = Boks::new(&mut y);
    println!("{:?}", y);

    // @Note: if we did not have a `PhantomData<T>` inside of Boks,
    // then the following code would (incorrectly) compile:
    //  |
    //  |   let mut z = 42;
    //  |   let b = Boks::new(Oisann(&mut z));
    //  |   println!("{:?}", z);
    //
}
