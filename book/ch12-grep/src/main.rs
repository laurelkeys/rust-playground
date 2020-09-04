use std::env;
use std::process;

use ch12_grep as minigrep;
use minigrep::Config;

// @Note: the responsibilities of the `main` function are limited to the following:
//  * Calling the command line parsing logic with the argument values
//  * Setting up any other configuration
//  * Calling a `run` function in lib.rs
//  * Handling the error if `run` returns an error
//
// This pattern is about separating concerns: main.rs handles running the program,
// and lib.rs handles all the logic of the task at hand.
//
// See https://doc.rust-lang.org/book/ch12-03-improving-error-handling-and-modularity.html#separation-of-concerns-for-binary-projects

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(err) = minigrep::run(config) {
        eprintln!("Application error: {}", err);
        process::exit(1);
    }
}
