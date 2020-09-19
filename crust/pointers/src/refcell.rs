use std::cell::UnsafeCell;

#[derive(Copy, Clone)]
enum RefState {
    Unshared,
    Shared(usize),
    Exclusive,
}

pub struct RefCell<T> {
    value: UnsafeCell<T>,
    state: crate::cell::Cell<RefState>,
}

// @Note: using `UnsafeCell<T>` already implies:
//  |
//  |   impl<T> !Sync for RefCell<T> {}
//

impl<T> RefCell<T> {
    pub fn new(value: T) -> Self {
        RefCell {
            value: UnsafeCell::new(value),
            state: crate::cell::Cell::new(RefState::Unshared),
        }
    }

    pub fn borrow(&self) -> Option<Ref<'_, T>> {
        match self.state.get() {
            RefState::Unshared => {
                self.state.set(RefState::Shared(1));
                Some(Ref { refcell: self })
            }
            RefState::Shared(n) => {
                self.state.set(RefState::Shared(n + 1));
                Some(Ref { refcell: self })
            }
            RefState::Exclusive => None,
        }
    }

    pub fn borrow_mut(&self) -> Option<RefMut<'_, T>> {
        match self.state.get() {
            RefState::Unshared => {
                self.state.set(RefState::Exclusive);
                Some(RefMut { refcell: self })
            }
            _ => None,
        }
    }
}

//
// Ref
//

pub struct Ref<'a, T> {
    refcell: &'a RefCell<T>,
}

impl<T> std::ops::Deref for Ref<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // @Safety: a `Ref` is only created if no exclusive references
        // have been given out. Once it is given out, `state` is set
        // to `Shared`, so no exclusive references are given out.
        // Thus, dereferencing into a shared reference is fine.
        unsafe { &*self.refcell.value.get() }
    }
}

impl<T> Drop for Ref<'_, T> {
    fn drop(&mut self) {
        match self.refcell.state.get() {
            RefState::Shared(1) => {
                self.refcell.state.set(RefState::Unshared);
            }
            RefState::Shared(n) => {
                self.refcell.state.set(RefState::Shared(n - 1));
            }
            _ => unreachable!(),
        }
    }
}

//
// RefMut
//

pub struct RefMut<'a, T> {
    refcell: &'a RefCell<T>,
}

impl<T> std::ops::Deref for RefMut<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // @Safety: see @Safety for `DerefMut`.
        unsafe { &*self.refcell.value.get() }
    }
}

impl<T> std::ops::DerefMut for RefMut<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // @Safety: a `RefMut` is only created if no other references
        // have been given out. Once it is given out, `state` is set
        // to `Exclusive`, so no future references are given out.
        // Because of this, we have an exclusive lease on the inner value,
        // hence why mutably dereferencing is fine.
        unsafe { &mut *self.refcell.value.get() }
    }
}

impl<T> Drop for RefMut<'_, T> {
    fn drop(&mut self) {
        match self.refcell.state.get() {
            RefState::Exclusive => {
                self.refcell.state.set(RefState::Unshared);
            }
            _ => unreachable!(),
        }
    }
}

//
// Test functions.
//

#[cfg(test)]
mod test {
    use super::RefCell;
}
