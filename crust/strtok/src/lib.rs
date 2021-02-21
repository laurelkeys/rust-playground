use std::marker::PhantomData;

// @Note: Rust differs between three types of variance:
// covariance, contravariance and invariance.
//
// See https://doc.rust-lang.org/nomicon/subtyping.html
// See https://doc.rust-lang.org/reference/subtyping.html

struct Foo<T> /* is covariant in T */ {
    // ... some fields
    _t: PhantomData<T>, // "owns a T"
}

struct Bar<T> /* is covariant in T */ {
    // ... some fields
    _t: PhantomData<fn() -> T>, // doesn't "own a T"
}

struct Baz<T> /* is contravariant in T */ {
    // ... some fields
    _t: PhantomData<fn(T)>, // also doesn't "own a T"
}

struct Qux<T> /* is invariant in T */ {
    // ... some fields
    _t: PhantomData<fn() -> T>,
    _t2: PhantomData<fn(T)>,
    // @Note: we could also use a single `PhantomData<fn(T) -> T>`.
}

// @Note: since `Qux<T>` can't be both covariant and contravariant
// in T at the same time we force it to be invariant, but we could
// also achieve this by using a single `PhantomData<*mut T>`.
// Also, for Bar<T> we could use a `PhantomData<*const T>` instead
// (i.e. to make it covariant in T).

// pub fn strtok<'a>(s: &'a mut &'a str, delim: char) -> &'a str {
pub fn strtok<'a, 'b>(s: &'a mut &'b str, delim: char) -> &'b str {
    if let Some(i) = s.find(delim) {
        let token = &s[..i];
        *s = &s[(i + delim.len_utf8())..];
        token
    } else {
        let token = *s;
        *s = "";
        token
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut x = "hello world";
        let hello = strtok(&mut x, ' ');
        assert_eq!(hello, "hello");
        assert_eq!(x, "world");

        // @Note: by using a single lifetime 'a in the function signature:
        //  |
        //  |   strtok<'a>(&'a mut &'a str, char) -> &'a str
        //
        // the lifetime of the mut borrow of `x` is inferred to be 'static,
        // since `x` is a `&'static str`, i.e.:
        //  |
        //  |   strtok<'a>(&'a mut &'a           str, char) -> &'a      str
        //  |   strtok    (&'static mut &'static str, char) -> &'static str
        //
        // which leads to the error "cannot borrow `x` as immutable because
        // it is also borrowed as mutable", since `x`s borrow would have to
        // last for the entire duration of the program (because of 'static).
        //
        // However, once we introduce a second lifetime annotation, 'b:
        //  |
        //  |   strtok<'a, 'b>(&'a mut &'b str, char) -> &'b str
        //
        // the compiler can now make 'a be the lifetime of the borrow of `x`:
        //  |
        //  |   strtok<'a, 'b>(&'a mut &'b      str, char) -> &'b      str
        //  |   strtok        (&'x mut &'static str, char) -> &'static str
        //
        // and so, everything works as we'd expect :)
    }
}
