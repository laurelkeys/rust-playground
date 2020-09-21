//
// Flavors:
//
//  * Synchronous channels
//      Channel where `send()` can block. Usually has *limited capacity*.
//      Also known as "bounded channels".
//      See https://youtu.be/b4mS5UPHh20?t=4966
//        - Mutex + Condvar + VecDeque
//        - Atomic VecDeque (atomic queue) + thread::park + thread::Thread::notify
//
//  * Asynchronous channels
//      Channel where `send()` cannot block. Usually *unbounded*.
//      Also known as "unbounded channels".
//      See https://youtu.be/b4mS5UPHh20?t=5148
//        - Mutex + Condvar + VecDeque
//        - Mutex + Condvar + LinkedList
//        - Atomic linked list, linked list of T
//        - Atomic block linked list, linked list of atomic VecDeque<T>
//
//  * Rendezvous channels
//      Synchronous channel with capacity = 0 (i.e. you can only send if there is
//      currently a blocking receiver, since you can't store anything in the channel
//      itself, data has to be handed to a thread that is currently waiting).
//      Used for (two-way) thread synchronization.
//      See https://youtu.be/b4mS5UPHh20?t=5344
//
//  * Oneshot channels
//      Any capacity channels that, in practice, you can only call `send()` on once.
//      See https://youtu.be/b4mS5UPHh20?t=5383
//

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
        Receiver { shared: Arc::clone(&shared), buffer: VecDeque::default() },
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
    buffer: VecDeque<T>, // "local buffer" to spare calls to `recv()`
}

impl<T> Receiver<T> {
    pub fn recv(&mut self) -> Option<T> {
        // Return values from the local buffer, if there are any
        // (thus, avoiding a mutex lock).
        if let Some(t) = self.buffer.pop_front() {
            return Some(t);
        }

        let mut inner = self.shared.inner.lock().unwrap();
        // @Note: this loop is not a spinlock because, using `Condvar`, we can block a
        // thread such that it consumes no CPU time while waiting for an event to occur.
        loop {
            match inner.queue.pop_front() {
                Some(t) => {
                    // Store remaining messages into the receiver's local buffer.
                    if !inner.queue.is_empty() {
                        // @Note: because of this, the lock will be taken fewer times,
                        // so this optimization reduces the amount of contention.
                        std::mem::swap(&mut self.buffer, &mut inner.queue)
                    }
                    return Some(t);
                }
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

    #[test]
    fn iterator() {
        let (mut tx, rx) = channel();

        tx.send(42);
        tx.send(43);
        tx.send(44);

        assert_eq!(rx.take(3).collect::<Vec<i32>>(), vec![42, 43, 44]);
    }
}
