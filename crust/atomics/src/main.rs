use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::{cell::UnsafeCell, thread::spawn};

const LOCKED: bool = true;
const UNLOCKED: bool = false;

pub struct Mutex<T> {
    locked: AtomicBool,
    v: UnsafeCell<T>,
}

// @Note: we implement Sync for Mutex so that we can use it with std::thread::spawn.
unsafe impl<T> Sync for Mutex<T> where T: Send {}

impl<T> Mutex<T> {
    pub fn new(t: T) -> Self {
        Self {
            locked: AtomicBool::new(UNLOCKED),
            v: UnsafeCell::new(t),
        }
    }

    pub fn with_lock<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
        // @Note: this is just a spin-lock... you shouldn't ever actually do this
        // https://matklad.github.io/2020/01/02/spinlocks-considered-harmful.html
        while self
            .locked
            .compare_exchange_weak(UNLOCKED, LOCKED, Ordering::Acquire, Ordering::Relaxed)
            .is_err()
        {
            while self.locked.load(Ordering::Relaxed) == LOCKED {}
        }

        // @Safety: we hold the lock, therefore we can create a mutable reference.
        let ret = f(unsafe { &mut *self.v.get() });

        self.locked.store(UNLOCKED, Ordering::Release);

        ret
    }
}

fn main() {
    let x: &'static _ = Box::leak(Box::new(AtomicBool::new(false)));
    let y: &'static _ = Box::leak(Box::new(AtomicBool::new(false)));
    let z: &'static _ = Box::leak(Box::new(AtomicUsize::new(0)));

    let _tx = spawn(move || {
        x.store(true, Ordering::Release);
    });
    let _ty = spawn(move || {
        y.store(true, Ordering::Release);
    });
    let t1 = spawn(move || {
        while !x.load(Ordering::Acquire) { /*spin-lock*/ }
        if y.load(Ordering::Acquire) {
            z.fetch_add(1, Ordering::Relaxed);
        }
    });
    let t2 = spawn(move || {
        while !y.load(Ordering::Acquire) { /*spin-lock*/ }
        if x.load(Ordering::Acquire) {
            z.fetch_add(1, Ordering::Relaxed);
        }
    });
    t1.join().unwrap();
    t2.join().unwrap();

    let z = z.load(Ordering::SeqCst);
    println!("z = {}", z);
    // @Note: what are the possible values for z?
    //
    //  - Is 2 possible?
    //      Yes, e.g.: _tx, _ty, t1, t2.
    //
    //  - Is 1 possible?
    //      Yes, e.g.: _tx, t1, _ty, t2.
    //
    //  - Is 0 possible?
    //      We have a couple of restrictions: we know that t1 must run "after" _tx,
    //      and the same thing is true for t2, i.e., t2 must run "after" _ty. Given
    //      that, we must have something like "... _tx ... t1 ...":
    //
    //          - If t2 goes at the start:
    //              _ty t2 _tx t1 -> t1 will increment z
    //          - If t2 goes at the middle:
    //              _ty _tx t2 t1 -> t1 and t2 will increment z
    //              _tx _ty t2 t1 -> t1 and t2 will increment z
    //          - If t2 goes at the end:
    //              _ty _tx t1 t2 -> t1 and t2 will increment z
    //              _tx _ty t1 t2 -> t1 and t2 will increment z
    //              _tx t1 _ty t2 -> t2 will increment z
    //
    //      It therefore *seems* impossible to have a thread schedule where z == 0.
    //
    //      However, given the acquire and release semantics, when t1 observes the
    //      value of x from an Acquire (which was written by a store with Release),
    //      it will see all operations that happened before the store. Note, though,
    //      that the store happens in thread _tx, and there are no operations prior
    //      to the store of x (except for its initialization on the main thread).
    //      Now, once in t1 we get to the load of y, its Acquire synchronizes with
    //      whichever store the value it gets, y, stored. There's no requirement
    //      that it's any particular store of y. If there was a y.store() in _tx,
    //      then it must observe it if it happened strictly before the x.store()
    //      (which, in turn, happened before the x.load() on t1 because of Acquire
    //      and Release)... but there isn't, so it is allowed to see any value for
    //      y, regardless of whether _ty has run or not (in wall time), i.e, there
    //      is no "happens-before" relationship between y's _ty store and t1 load.
    //
    //      Hence, t1 will see x as true, but it is allowed to see either true or
    //      false for y. Similarly, t2 sees y as true, but either value for x.
    //
    //      Because of this, we could actually get z == 0 with the ordering above!
    //
    //      SeqCst is AcqRel with the additional guarantee that all sequentially
    //      consistent operations must be seen as happenning in the same order on
    //      all threads. Hence, if we replace the Release's on store's and the
    //      Acquire's on load's with SeqCst's, then z == 0 is not be possible!
}

#[cfg(test)]
mod test {
    use super::*;
    use std::thread::spawn;

    #[test]
    fn test_mutex() {
        let l: &'static _ = Box::leak(Box::new(Mutex::new(0)));
        let handles: Vec<_> = (0..10)
            .map(|_| {
                spawn(move || {
                    for _ in 0..100 {
                        l.with_lock(|v| {
                            *v += 1;
                        });
                    }
                })
            })
            .collect();

        for handle in handles {
            handle.join().unwrap();
        }

        assert_eq!(l.with_lock(|v| *v), 10 * 100);
    }

    #[test]
    fn too_relaxed() {
        let x: &'static _ = Box::leak(Box::new(AtomicUsize::new(0)));
        let y: &'static _ = Box::leak(Box::new(AtomicUsize::new(0)));
        let t1 = spawn(move || {
            let r1 = y.load(Ordering::Relaxed);
            x.store(r1, Ordering::Relaxed);
            r1
        });
        let t2 = spawn(move || {
            let r2 = x.load(Ordering::Relaxed);
            y.store(42, Ordering::Relaxed);
            r2
        });
        let _r1 = t1.join().unwrap();
        let _r2 = t2.join().unwrap();
    }
}
