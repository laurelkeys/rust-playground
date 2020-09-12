use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

enum Message {
    NewJob(Job), // holds the `Job` the thread should run
    Terminate,   // causes the thread to exit its loop and stop
}

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

        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        // @Note: to prevent a deadlock scenario, we first put all
        // of our `Terminate` messages on the channel in one loop;
        // then we join on all the threads in another loop.
        //
        // Each worker will stop receiving requests on the channel once
        // it gets a terminate message. So, we can be sure that if we send
        // the same number of terminate messages as there are workers, each
        // worker will receive a terminate message before `join` is called on its thread.

        println!("Sending terminate message to all workers.");

        for _ in &self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        println!("Shutting down all workers.");

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move ||
            loop {
                // @Note: the call to `recv` blocks, so if there is no job yet,
                // the current thread will wait until a job becomes available.
                // The `Mutex<T>` ensures that only one `Worker` thread at a time
                // is trying to request a job.

                // @Note: acquiring a lock might fail if the mutex is in
                // a poisoned state, which can happen if some other thread
                // panicked while holding the lock rather than releasing it.

                let message = receiver.lock().unwrap().recv().unwrap();

                match message {
                    Message::NewJob(job) => {
                        println!("Worker {} got a job; executing.", id);

                        // @Note: since we acquired the lock without assigning to a variable,
                        // the temporary `MutexGuard` returned from the `lock` method is
                        // dropped as soon as the `let job` statement ends.
                        //
                        // This ensures that the lock is held during the call to `recv`,
                        // but it is released before the call to `job()`, allowing multiple
                        // requests to be serviced concurrently.

                        job();
                    }
                    Message::Terminate => {
                        println!("Worker {} was told to terminate.", id);

                        break;
                    }
                }
            }
        );

        Worker {
            id,
            thread: Some(thread),
        }
    }
}
