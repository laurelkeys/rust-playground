pub trait HelloMacro {
    fn hello_macro();
}

//
// Test functions.
//

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn implement_trait_without_macro() {

        struct Pancakes;

        impl HelloMacro for Pancakes {
            fn hello_macro() {
                println!("Hello, Macro! My name is Pancakes!");
            }
        }

        Pancakes::hello_macro();
    }
}
