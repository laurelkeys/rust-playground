use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        // @Note: we use a `Arc<Mutex<T>>` to share ownership of the
        // receiver across the pool's worker threads, and also allow
        // them to mutate its value.
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender }
    }

    // @Note: `FnOnce()` represents a closure that takes no parameters
    // and returns the unit type `()`.
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.send(job).unwrap();
    }
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            // @Note: acquiring a lock might fail if the mutex is in
            // a poisoned state, which can happen if some other thread
            // panicked while holding the lock rather than releasing it.

            // @Note: the call to `recv` blocks, so if there is no job yet,
            // the current thread will wait until a job becomes available.
            // The `Mutex<T>` ensures that only one `Worker` thread at a time
            // is trying to request a job.

            let job = receiver.lock().unwrap().recv().unwrap();

            println!("Worker {} got a job; executing.", id);

            // @Note: since we acquired the lock without assigning to a variable,
            // the temporary `MutexGuard` returned from the `lock` method is
            // dropped as soon as the `let job` statement ends.
            //
            // This ensures that the lock is held during the call to `recv`,
            // but it is released before the call to `job()`, allowing multiple
            // requests to be serviced concurrently.

            job();
        });

        Worker { id, thread }
    }
}
