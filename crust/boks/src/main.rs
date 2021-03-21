#![feature(dropck_eyepatch)] // $ rustup override set nightly

use std::{marker::PhantomData, ptr::NonNull};

pub struct Boks<T> {
    p: *mut T,
    _t: PhantomData<T>,
}

pub struct AnnenBoks<T> {
    // @Note: by making `p` be of type `NonNull<T>` instead of `*mut T`, we make this
    // other ("annen") Boks covariant in its contained type T, instead of invariant.
    p: NonNull<T>,
    _t: PhantomData<T>,
}

// @Note: by using `#[may_dangle] T` we "promisse" to the compiler that we do not
// access the T inside of `drop`. However, we do drop a T, so to ensure that the
// compiler knows that, we introduce a `PhantomData<T>`, which signals  to the
// compiler that Boks "holds" a T (hence, it drops it when `drop` is called).

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
unsafe impl<#[may_dangle] T> Drop for AnnenBoks<T> {
    fn drop(&mut self) {
        // @Safety: see `unsafe impl<#[may_dangle] T> Drop for Boks<T>`
        unsafe { Box::from_raw(self.p.as_mut()) };
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
impl<T> AnnenBoks<T> {
    pub fn new(t: T) -> Self {
        // @Safety: `Box` never creates a null pointer.
        AnnenBoks {
            p: unsafe { NonNull::new_unchecked(Box::into_raw(Box::new(t))) },
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
impl<T> std::ops::Deref for AnnenBoks<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        // @Safety: see `impl<T> std::ops::Deref for Boks<T>`.
        unsafe { &*self.p.as_ref() }
    }
}

impl<T> std::ops::DerefMut for Boks<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // @Safety: same argument above applies (see `impl<T> std::ops::Deref for Boks<T>`).
        // Also, since we have `&mut self`, no other mutable reference to `p` has been given out.
        unsafe { &mut *self.p }
    }
}
impl<T> std::ops::DerefMut for AnnenBoks<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // @Safety: see `impl<T> std::ops::DerefMut for Boks<T>`.
        unsafe { &mut *self.p.as_mut() }
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
    // because, while Boks does not access its generic type parameter
    // on `drop`, it does drop the inner type T.

    let s = String::from("hei");
    // @Note: because of the invariance that gets introduced by the use of `*mut T`,
    // a `Boks<&'static str>` cannot be treated as a `Boks<&'a str>`, where `'a` is
    // some lifetime shorter than `'static`, even though we can use `&'static str`
    // as `&'a str` and we can also treat `Box<&'static str>` as a `Box<&'a str>`:
    //  |
    //  |   let mut boks1 = Boks::new(&*s);
    //  |   let boks2: Boks<&'static str> = Boks::new("heisann");
    //  |   boks1 = boks2;
    //
    // because Box is covariant in its contained inner type T.
    let mut box1 = Box::new(&*s);
    let box2: Box<&'static str> = Box::new("heisann");
    box1 = box2;

    let mut annen_boks1 = AnnenBoks::new(&*s);
    let annen_boks2: AnnenBoks<&'static str> = AnnenBoks::new("heisann");
    annen_boks1 = annen_boks2;
}
