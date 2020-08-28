use std::io;

fn main() {
    let mut n = String::new();

    println!("Please input a value for `n`:");
    io::stdin().read_line(&mut n).expect("Failed to read line");

    // @Note: `unwrap` panics if `parse` returns an `Err`.
    let n = n.trim().parse::<u32>().unwrap();

    println!(
        "The {}{} Fibonacci number is: {}",
        n,
        match n {
            1 => "st",
            2 => "nd",
            3 => "rd",
            _ => "th",
        },
        fibonacci(n)
    );
}

fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

//
// Test functions.
//

#[cfg(test)]
mod test {
    use super::fibonacci;

    #[test]
    fn basics() {
        // https://en.wikipedia.org/wiki/Fibonacci_number#Sequence_properties

        assert_eq!(fibonacci(0), 0);
        assert_eq!(fibonacci(1), 1);
        assert_eq!(fibonacci(2), 1);
        assert_eq!(fibonacci(3), 2);
        assert_eq!(fibonacci(4), 3);
        assert_eq!(fibonacci(5), 5);
        assert_eq!(fibonacci(6), 8);
        assert_eq!(fibonacci(7), 13);
        assert_eq!(fibonacci(8), 21);
        assert_eq!(fibonacci(9), 34);
        assert_eq!(fibonacci(10), 55);
        assert_eq!(fibonacci(11), 89);
        assert_eq!(fibonacci(12), 144);
        assert_eq!(fibonacci(13), 233);
        assert_eq!(fibonacci(14), 377);
        assert_eq!(fibonacci(15), 610);
        assert_eq!(fibonacci(16), 987);
        assert_eq!(fibonacci(17), 1597);
        assert_eq!(fibonacci(18), 2584);
        assert_eq!(fibonacci(19), 4181);
        assert_eq!(fibonacci(20), 6765);
    }
}
