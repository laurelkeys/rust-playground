use rand::Rng;
use std::{cmp::Ordering, io};

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: i32 = match guess.trim().parse() {
            Ok(number) => number,
            Err(_) => continue,
        };
        // @Note: we could also use a turbofish and not annotate `guess`'s type:
        //  |
        //  |   let guess = match guess.trim().parse::<i32>() { ... }
        //

        let guess = match guess {
            1..=100 => Guess::new(guess),
            _ => {
                println!("The secret number will be between 1 and 100.");
                continue;
            }
        };

        println!("You guessed: {}", guess.value());

        match guess.value().cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

//
// Test functions.
//

#[cfg(test)]
mod tests {
    use super::*;

    // @Note: using `should_panic` with `expected` will only pass if the test
    // function panics *and* the panic message contains the expected substring.
    #[test]
    #[should_panic(expected = "Guess value must be between 1 and 100")]
    fn greater_than_100() {
        Guess::new(200);
    }
}
