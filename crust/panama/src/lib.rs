use std::collections::VecDeque;
use std::sync::{Arc, Condvar, Mutex};

// Holds the inner data that is shared between the sender(s) and receiver.
struct Shared<T> {
    inner: Mutex<Inner<T>>,
    available: Condvar,
}

struct Inner<T> {
    queue: VecDeque<T>,
    senders: usize,
}

pub fn channel<T>() -> (Sender<T>, Receiver<T>) {
    let inner = Inner { queue: VecDeque::default(), senders: 1 };
    let shared = Shared { inner: Mutex::new(inner), available: Condvar::new() };
    let shared = Arc::new(shared);

    (
        Sender { shared: Arc::clone(&shared) },
        Receiver { shared: Arc::clone(&shared) },
    )
}

//
// Sender.
//

pub struct Sender<T> {
    shared: Arc<Shared<T>>,
}

impl<T> Clone for Sender<T> {
    fn clone(&self) -> Self {
        let mut inner = self.shared.inner.lock().unwrap();
        inner.senders += 1;
        drop(inner);

        // @Note: we could write `self.shared.clone()` instead of using `Arc::clone()`,
        // but doing so could mean to call `clone()` on the type inside the `Arc`,
        // since it auto-derefs to the inner type. So it's better to be specific.
        Sender {
            shared: Arc::clone(&self.shared),
        }
    }
}

impl<T> Drop for Sender<T> {
    fn drop(&mut self) {
        let mut inner = self.shared.inner.lock().unwrap();
        inner.senders -= 1;

        let was_last = inner.senders == 0;
        drop(inner);
        if was_last {
            // @Note: when the last sender goes away, there can only be the
            // receiver left. So, there's at most *one* thread waiting.
            self.shared.available.notify_one()
        }
    }
}

impl<T> Sender<T> {
    pub fn send(&mut self, t: T) {
        let mut inner = self.shared.inner.lock().unwrap();
        inner.queue.push_back(t);

        // Drop the lock before notifying a receiver that's waiting on the `available`
        // `Condvar`, so that it can wake up and immediately grab the lock.
        drop(inner);
        self.shared.available.notify_one(); // notifies a receiver (since we are the sender)
    }
}

//
// Receiver.
//

pub struct Receiver<T> {
    shared: Arc<Shared<T>>,
}

impl<T> Receiver<T> {
    pub fn recv(&mut self) -> Option<T> {
        let mut inner = self.shared.inner.lock().unwrap();
        // @Note: this loop is not a spinlock because, using `Condvar`, we can block a
        // thread such that it consumes no CPU time while waiting for an event to occur.
        loop {
            match inner.queue.pop_front() {
                Some(t) => return Some(t),
                None => {
                    if inner.senders == 0 {
                        return None;
                    } else {
                        inner = self.shared.available.wait(inner).unwrap();
                    }
                }
            }
        }
    }
}

impl<T> Iterator for Receiver<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.recv()
    }
}

//
// Test functions.
//

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ping_pong() {
        let (mut tx, mut rx) = channel();
        tx.send(42);
        assert_eq!(rx.recv(), Some(42));
    }

    #[test]
    fn closed_tx() {
        let (tx, mut rx) = channel::<()>();

        // @Note: assigning to underscore doesn't immediately drop.
        // So if we used this, instead of `drop()`, it'd hang forever:
        //  |
        //  |   let _ = tx;
        //
        drop(tx);
        assert_eq!(rx.recv(), None);
    }

    #[test]
    fn closed_rx() {
        let (mut tx, rx) = channel();

        // See https://youtu.be/b4mS5UPHh20?t=3282
        drop(rx);
        tx.send(42);
    }
}
