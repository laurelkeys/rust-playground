// Common.
use std::thread;

// Used for the message-passing example.
use std::sync::mpsc;
use std::time::Duration;

// Used for the shared-state example.
use std::sync::{Arc, Mutex};

fn main() {
    //
    // Message-passing concurrency.
    //

    // @Note: `mpsc` stands for "multiple producer, single consumer", and
    // `tx` and `rx` are common abbreviations for "transmitter" and "receiver".
    let (tx, rx) = mpsc::channel();

    let tx1 = mpsc::Sender::clone(&tx);

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }

    println!("---");

    //
    // Shared-state concurrency.
    //

    // @Note: `Rc<T>` cannot be sent between threads safely as it doesn't
    // implement the `Send` trait. instead, we use `Arc<T>` which is an
    // *atomically reference counted* type.
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
