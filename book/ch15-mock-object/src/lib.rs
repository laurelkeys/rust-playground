pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger
                .send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger
                .send("Warning: You've used up over 75% of your quota!");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    // @Note: using `RefCell<T>` allows us to have *interior mutability*.
    //
    // With this, we can create a mock object that, instead of implementing
    // some logic to send messages when we call `send`, will only keep track
    // of the messages it's told to send.
    //
    // We can then create a new instance of the mock object, create a
    // `LimitTracker` that uses it, call the `set_value` method on `LimitTracker`,
    // and then check that the mock object has the messages we expect.
    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            self.sent_messages.borrow_mut().push(String::from(message));

            // @Note: if we try to have more than one mutable reference
            // at a time, `RefCell<T>` will panic, e.g.:
            //  |
            //  |   let mut one_borrow = self.sent_messages.borrow_mut();
            //  |   let mut two_borrow = self.sent_messages.borrow_mut();
            //  |
            //  |   one_borrow.push(String::from(message));
            //  |   two_borrow.push(String::from(message));
            //
            // The code above panics with: 'already borrowed: BorrowMutError'
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}
