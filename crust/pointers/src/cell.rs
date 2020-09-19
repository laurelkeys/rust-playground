use std::cell::UnsafeCell;

pub struct Cell<T> {
    value: UnsafeCell<T>,
}

// @Note: using `UnsafeCell<T>` already implies:
//  |
//  |   impl<T> !Sync for Cell<T> {}
//

impl<T> Cell<T> {
    pub fn new(value: T) -> Self {
        Cell {
            value: UnsafeCell::new(value),
        }
    }

    pub fn set(&self, value: T) {
        // @Safety: we know no one else is concurrently mutating `self.value`
        // because our `Cell<T>` is `!Sync`. Also, we know we're not invalidating
        // any references, because we never give any out.
        unsafe { *self.value.get() = value };
    }

    pub fn get(&self) -> T
    where
        T: Copy,
    {
        // @Safety: we know no one else is modifying this value, since only
        // this thread can mutate (because `!Sync`), and it is executing this
        // function instead.
        unsafe { *self.value.get() }
    }
}

//
// Test functions.
//

#[cfg(test)]
mod test {
    use super::Cell;
}
