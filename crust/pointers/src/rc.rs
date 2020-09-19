use std::marker::PhantomData;
use std::ptr::NonNull;

// Shared state between `Rc` instances.
struct RcInner<T> {
    value: T,
    refcount: crate::cell::Cell<usize>,
}

pub struct Rc<T> {
    // @Note: `Rc` cannot be `Send`, because `Cell` is not thread-safe.
    // Though, since `NonNull` is not `Send`, the compiler won't try to
    // automatically implement it for us, so we don't have to worry about it.
    inner: NonNull<RcInner<T>>, // `*mut RcInner<T>` that is never `ptr::null()`

    // See https://doc.rust-lang.org/nomicon/dropck.html
    // See https://doc.rust-lang.org/nomicon/phantom-data.html
    _marker: PhantomData<RcInner<T>>, // makes the compiler treat `Rc<T>` as owning a `RcInner<T>`
}

impl<T> Rc<T> {
    pub fn new(value: T) -> Self {
        // Heap-allocate the shared inner state:
        let inner = Box::new(RcInner {
            value,
            refcount: crate::cell::Cell::new(1),
        });

        // @Note: we use `Box::into_raw(inner)`, instead of simply using `&*inner`
        // because, in this case, `inner`'s `Box` would get dropped at the end of
        // this scope, thus freeing its memory and invalidating the `Rc`'s `inner`.
        Rc {
            // @Safety: `Box` does not give us a null pointer.
            inner: unsafe { NonNull::new_unchecked(Box::into_raw(inner)) },
            _marker: PhantomData,
        }
    }
}

impl<T> std::ops::Deref for Rc<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // @Safety: `self.inner` is a `Box` that is only deallocated when
        // the last `Rc` goes away. We have an `Rc`, therefore it has not
        // been deallocated, so deref is fine.
        &unsafe { self.inner.as_ref() }.value
    }
}

impl<T> Clone for Rc<T> {
    fn clone(&self) -> Self {
        let inner = unsafe { self.inner.as_ref() };
        let count = inner.refcount.get();
        inner.refcount.set(count + 1);
        Rc {
            inner: self.inner,
            _marker: PhantomData,
        }
    }
}

impl<T> Drop for Rc<T> {
    fn drop(&mut self) {
        let inner = unsafe { self.inner.as_ref() };
        let count = inner.refcount.get();
        if count == 1 {
            // Move ownership of the pointer to the `Box`, which will immediately
            // be dropped (at the end of this scope).
            //
            // @Safety: we are the only `Rc` left, and we are being dropped.
            // So, after us, there will be no `Rc`s, and no references `T`.
            let _ = unsafe { Box::from_raw(self.inner.as_ptr()) };
        } else {
            // There are other `Rc`s, so don't drop the `Box`!
            inner.refcount.set(count - 1);
        }
    }
}
