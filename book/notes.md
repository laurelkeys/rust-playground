## Cargo
* A crate is a collection of Rust source code files
* Running `cargo doc --open` will build the documentation provided by all of your dependencies (locally) and open it in the browser [[ch02-00](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html#generating-a-random-number)]

## Functions
* The `::` syntax in `::new` indicates that `new` is an *associated function* of the `String` type:
    ```rust
    String::new()
    ```
* An *associated function* is implemented on a type, rather than on a particular instance of the type. Some languages call this a *static method* [[ch02-00](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html#storing-values-with-variables)] [[ch05-05](https://doc.rust-lang.org/book/ch05-03-method-syntax.html#associated-functions)]
* Function bodies are made up of a series of *statements* optionally ending in an *expression* [[ch03-03](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html#function-bodies-contain-statements-and-expressions)]
* *Statements* are instructions that perform some action and do not return a value. *Expressions* evaluate to a resulting value
* Expressions do not include ending semicolons. If you add a semicolon to the end of an expression, you turn it into a statement, which will then not return a value
* Passing a variable to a function will move or copy, just as assignment does [[ch04-01](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#ownership-and-functions)]
* When a closure captures a value from its environment, it uses memory to store the values for use in the closure body. Because functions are never allowed to capture their environment, defining and using functions will never incur this overhead [[ch13-01](https://doc.rust-lang.org/book/ch13-01-closures.html#capturing-the-environment-with-closures)]
* Closures can capture values from their environment in three ways, which directly map to the three ways a function can take a parameter: taking ownership, borrowing mutably, and borrowing immutably. These are encoded in the three `Fn` traits as follows:
    * `FnOnce` consumes the variables it captures from its enclosing scope, known as the closure's environment. To consume the captured variables, the closure must take ownership of these variables and move them into the closure when it is defined. The Once part of the name represents the fact that the closure can't take ownership of the same variables more than once, so it can be called only once
    * `FnMut` can change the environment because it mutably borrows values
    * `Fn` borrows values from the environment immutably
    Rust infers which trait to use based on how the closure uses the values from the environment
* If you want to force the closure to take ownership of the values it uses in the environment, you can use the `move` keyword before the parameter list:
    ```rust
    fn main() {
        let x = vec![1, 2, 3];

        let equal_to_x = move |z| z == x;

        // The following line results in a compilation error:
        println!("can't use x here: {:?}", x);

        let y = vec![1, 2, 3];

        assert!(equal_to_x(y));
    }
    ```
    This technique is mostly useful when passing a closure to a new thread to move the data so it's owned by the new thread


## Methods
* *Methods* are different from *functions* in that they're defined within the context of a struct, an enum or a trait object, and their first parameter is always `self` [[ch05-03](https://doc.rust-lang.org/book/ch05-03-method-syntax.html#method-syntax)]
* Rust has a feature called *automatic referencing and dereferencing*. When you call a method with `object.something()`, Rust automatically adds in `&`, `&mut`, or `*` so `object` matches the signature of the method [[ch05-03](https://doc.rust-lang.org/book/ch05-03-method-syntax.html#wheres-the---operator)]:
    ```rust
    p1.distance(&p2);
    (&p1).distance(&p2); // equivalent to the above
    ```

## Enums
* The values of an enum(eration) are called *variants* [[ch02-00](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html)]
* Each variant can have different types and amounts of associated data. For instance, the following:
    ```rust
    enum IpAddr {
        V4(String),
        V6(String),
    }

    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));
    ```
    Could be rewritten as [[ch06-01](https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html#enum-values)]:
    ```rust
    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
    ```
* Enum example with a wide variety of types embedded in its variants:
    ```rust
    enum Message {
        Quit, // has no data associated with it at all
        Move { x: i32, y: i32 }, // includes an anonymous struct inside it
        Write(String), // includes a single `String`
        ChangeColor(i32, i32, i32), // includes three `i32` values
    }
    ```

## Structs
* To create a new instance of a struct that uses most of an old instance's values we can use the *struct update syntax* [[ch05-01](https://doc.rust-lang.org/book/ch05-01-defining-structs.html#creating-instances-from-other-instances-with-struct-update-syntax)]:
    ```rust
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user1
    };
    ```
* *Tuple structs* have the added meaning the struct name provides but don't have names associated with their fields; rather, they just have the types of the fields:
    ```rust
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    ```
    You can destructure them into their individual pieces, you can use a `.` followed by the index to access an individual value, and so on.
* You can also define *unit-like structs* that don't have any fields, which behave similarly to `()`, the unit type. They can be useful in situations in which you need to implement a trait on some type but don't have any data that you want to store in the type itself [[ch05-10](https://doc.rust-lang.org/book/ch05-01-defining-structs.html#unit-like-structs-without-any-fields)]
* You can deestructure structs like [[ch18-03](https://doc.rust-lang.org/book/ch18-03-pattern-syntax.html#destructuring-structs)]:
    ```rust
    struct Point {
        x: i32,
        y: i32,
    }

    fn main() {
        let p = Point { x: 0, y: 7 };
        let Point { x: a, y: b } = p;
        assert_eq!(0, a);
        assert_eq!(7, b);

        let q = Point { x: 1, y: 8 };
        let Point { x, y } = q;
        assert_eq!(1, x);
        assert_eq!(8, y);
    }
    ```
    Which can be used in `match` expressions as well:
    ```rust
    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0, y } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }
    ```

## Match
* A `match` expression is made up of *arms*, which consists of a *pattern* and the code to be run if the value fits that arm's pattern [[ch02-00](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html#comparing-the-guess-to-the-secret-number)]
* You can match multiple patterns using the `|` syntax, which means *or* [[ch18-03](https://doc.rust-lang.org/book/ch18-03-pattern-syntax.html#multiple-patterns)]
* A *match guard* is an additional `if` condition specified after the pattern in an arm that must also match for that arm to be chosen [[ch18-03](https://doc.rust-lang.org/book/ch18-03-pattern-syntax.html#extra-conditionals-with-match-guards)]:
    ```rust
    let num = Some(4);

    match num {
        Some(x) if x < 5 => println!("less than five: {}", x),
        Some(x) => println!("{}", x),
        None => (),
    }
    ```
    When combined with `|`, the match guard will apply to *all* patterns:
    ```rust
    let x = 4;
    let y = false;

    match x {
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }

    // "no" is printed.
    ```
    is equivalent to `(4 | 5 | 6) if y => ...`, and *not* `4 | 5 | (6 if y) => ...` (which would have printed `"yes"` instead).

## Types
* The `isize` and `usize` types size depend on the computer architecture (i.e. 64-bit or 32-bit)
* Signed integer types are stored using two's complement notation [[ch03-03](https://doc.rust-lang.org/book/ch03-02-data-types.html#integer-types)]
* In debug mode, Rust checks for integer overflow and causes the program to *panic* at runtime if it occurs
* In `--release` mode, Rust does *not* include checks, instead, it will perform two's complement wrapping
* If you want values to "wrap around" you should be explicit, by using the standard library type [`Wrapping`](https://doc.rust-lang.org/std/num/struct.Wrapping.html)
* The `char` type is four bytes in size and represents Unicode Scalar Value, which range from `U+0000` to `U+D7FF` and `U+E000` to `U+10FFFF` inclusive [[ch03-03](https://doc.rust-lang.org/book/ch03-02-data-types.html#the-character-type)]
* *Tuples* have fixed length and can contain values of different types [[ch03-02](https://doc.rust-lang.org/book/ch03-02-data-types.html#the-tuple-type)]
* *Arrays* have fixed length but all its elements must have the same type, they're useful to allocate data on the stack or to ensure that you always have a fixed number of elements [[ch03-02](https://doc.rust-lang.org/book/ch03-02-data-types.html#the-array-type)]
* Rust checks for (and *panics* on) out of bounds indices for arrays [[ch03-02](https://doc.rust-lang.org/book/ch03-02-data-types.html#invalid-array-element-access)]

## Control Flow
* A `loop` block can return a value by placing it after a `break` expression [[ch03-05](https://doc.rust-lang.org/book/ch03-05-control-flow.html#returning-values-from-loops)]:
    ```rust
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    ```
* Simple `for` loop examples [[ch03-05](https://doc.rust-lang.org/book/ch03-05-control-flow.html#looping-through-a-collection-with-for)]:
    ```rust
    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("the value is: {}", element);
    }

    // (1..4) is a `Range`, and `rev` reverses it
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    ```

## Ownership
* Memory is managed through a system of ownership with a set of rules that the compiler checks at compile time [[ch04-01](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#ownership-rules)]:
    * Each value in Rust has a variable that's called its *owner*
    * There can only be one owner at a time
    * When the owner goes out of scope, the value will be dropped (*RAII*)
* The concept of copying the pointer, length, and capacity (in the case of a `String`) without copying the data probably sounds like making a *shallow copy*. But because Rust also invalidates the first variable, instead of being called a shallow copy, it's known as a *move* [[ch04-01](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#memory-and-allocation)]:
    ```rust
    let s1 = String::from("hello");
    let s2 = s1; // `s1` was moved into `s2`
    ```
* Rust will never automatically create "deep" copies of your data. Therefore, any *automatic* copying can be assumed to be inexpensive in terms of runtime performance
* If we *do* want to deeply copy the heap data of the `String`, not just the stack data, we can use a common method called `clone`:
    ```rust
    let s1 = String::from("hello");
    let s2 = s1.clone();
    ```
* Note that copies of "stack-only data" (i.e. types that have a known size at compile time, and thus, are stored entirely on the stack) are quick to make. So, there's no reason to prevent `x` from being valid after we create the variable `y` [[ch04-01](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#stack-only-data-copy)]:
    ```rust
    let x = 5;
    let y = x; // y is still valid
    ```
    Rust has a special annotation called the `Copy` trait that we can place on types like integers that are stored on the stack.
* Rust won't let us annotate a type with the `Copy` trait if the type, or any of its parts, has implemented the Drop trait. If the type needs something special to happen when the value goes out of scope and we add the `Copy` annotation to that type, we'll get a compile-time error.
* As a general rule, any group of simple scalar values can be `Copy`, and nothing that requires allocation or is some form of resource is `Copy`
* We call having references as function parameters *borrowing* [[ch04-02](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html#references-and-borrowing)]
* *Mutable references* (`&mut`) have one big restriction: you can have only one mutable reference to a particular piece of data in a particular scope, and you *also* cannot have a mutable reference while there is an immutable one in scope [[ch04-02](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html#mutable-references)]
    * Because of this, data races can be prevented at compile-time
* A reference's scope starts from where it is introduced and continues through the last time that reference is used. As such, the following works because the last usage of the immutable references occurs before the mutable reference is introduced:
    ```rust
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // r1 and r2 are no longer used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);
    ```

## Module System
* The idiomatic way of bringing a function into scope with `use` is to keep its parent, so that we have to specify the parent module when calling the function. This makes it clear that the function isn't locally defined while still minimizing repetition of the full path [[ch07-04](https://doc.rust-lang.org/book/ch07-04-bringing-paths-into-scope-with-the-use-keyword.html#creating-idiomatic-use-paths)]:
    ```rust
    mod front_of_house {
        pub mod hosting {
            pub fn add_to_waitlist() {}
        }
    }

    // Idiomatic way:
    use self::front_of_house::hosting;
    pub fn eat_at_restaurant() {
        hosting::add_to_waitlist();
        // ...
    }

    // Also valid, but not preferred:
    use crate::front_of_house::hosting::add_to_waitlist;
    pub fn eat_at_restaurant() {
        add_to_waitlist();
        // ...
    }
    ```
* On the other hand, when bringing in structs, enums, and other items with `use`, it's idiomatic to specify the full path:
    ```rust
    use std::collections::HashMap;
    fn main() {
        let mut map = HashMap::new();
        map.insert(1, 2);
    }
    ```
* When bringing two items with the same name into scope we can either add their parents in the `use` statement:
    ```rust
    use std::fmt;
    use std::io;

    fn function1() -> fmt::Result { /* ... */ }
    fn function2() -> io::Result<()> { /* ... */ }
    ```
    Or create a new local name, or alias, for one of the types with `as`:
    ```rust
    use std::fmt::Result;
    use std::io::Result as IoResult;

    fn function1() -> Result { /* ... */ }
    fn function2() -> IoResult<()> { /* ... */ }
    ```
* When a name is brought into scope with `use`, the name available in the new scope is private. To enable the code that calls our code to refer to it as if it was defined in that code's scope, we can combine `pub` and `use`. This technique is called *re-exporting* [[ch07-04](https://doc.rust-lang.org/book/ch07-04-bringing-paths-into-scope-with-the-use-keyword.html#re-exporting-names-with-pub-use)]

## Vector
* Example of iterating over a *vector* with immutable vs. mutable references [[ch08-01](https://doc.rust-lang.org/book/ch08-01-vectors.html#iterating-over-the-values-in-a-vector)]:
    ```rust
    let v = vec![100, 32, 57];

    for i in &v { println!("{}", i); }

    for i in &mut v { *i += 50; }
    ```
    Note that, to change the value that the mutable reference refers to, we use the dereference operator (`*`).

## Strings
* For any type that implements the `Display` trait we can convert it into a `String` in two ways [[ch08-02](https://doc.rust-lang.org/book/ch08-02-strings.html)]:
    ```rust
    let s = "initial contents".to_string();
    // or
    let s = String::from("initial contents");
    ```
* We can grow a `String` by using the `push_str` method to append a string slice (thus, we don't take ownership of the parameter) [[ch08-02](https://doc.rust-lang.org/book/ch08-02-strings.html#appending-to-a-string-with-push_str-and-push)]:
    ```rust
    let mut s = String::from("foo");
    s.push_str("bar"); // `s` is now "foobar"
    ```
* Another way of concatenating strings is with the `+` operator, whose signature is simlar to `fn add(self, s: &str) -> String` (though, generic) [[ch08-02](https://doc.rust-lang.org/book/ch08-02-strings.html#concatenation-with-the--operator-or-the-format-macro)]:
    ```rust
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note `s1` has been moved here and can no longer be used
    ```
    Note that, although the second parameter is a `&str`, `&s2`'s type is actually `&String`. The reason we're able to use `&s2` in the call to `add` is that the compiler can *coerce* the `&String` argument into a `&str`. When we call the `add` method, Rust uses a *deref coercion*, which here turns `&s2` into `&s2[..]`.
* If we need to concatenate multiple strings, the behavior of the `+` operator gets unwieldy:
    ```rust
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3; // `s` is "tic-tac-toe"
    ```
    Because of this, we'd usually use the `format!` macro in this case, which is much easier to read (and doesn't take ownership of any of its parameters):
    ```rust
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3); // `s` is "tic-tac-toe"
    ```
* See [[ch08-02](https://doc.rust-lang.org/book/ch08-02-strings.html#indexing-into-strings)] and [[ch08-02](https://doc.rust-lang.org/book/ch08-02-strings.html#bytes-and-scalar-values-and-grapheme-clusters-oh-my)] for Rust's representation of strings (which are UTF-8 encoded), and the ways of viewing them as: bytes, scalar values, and grapheme clusters (the closest thing to what we would call letters)
* Note that, using string slices can lead to panic at runtime because valid Unicode scalar values may be made up of more than 1 byte, so you may index an invalid position (i.e. not a char boundary) [[ch08-02](https://doc.rust-lang.org/book/ch08-02-strings.html#slicing-strings)]:
    ```rust
    let hello = "Здравствуйте"; // note each character is 2 bytes long

    let s = &hello[0..4]; // valid, `s` will be "Зд" (first 4 bytes of `hello`)
    let s = &hello[0..1]; // panics! "З" is 2 bytes long, so this is an invalid index
    ```
* The standard library provides a few ways of iterating over strings, e.g. individual Unicode scalar values [[ch08-02](https://doc.rust-lang.org/book/ch08-02-strings.html#methods-for-iterating-over-strings)]:
    ```rust
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }
    ```
    Raw bytes:
    ```rust
    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }
    ```
    Getting grapheme clusters from strings is complex though, so this functionality is not provided by the standard library.

## Hash Map
* Another way of constructing a hash map is by using iterators and the collect method on a vector of tuples, where each tuple consists of a key and its value [[ch08-03](https://doc.rust-lang.org/book/ch08-03-hash-maps.html#creating-a-new-hash-map)]:
    ```rust
    use std::collections::HashMap;

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let mut scores: HashMap<_, _> =
        teams.into_iter().zip(initial_scores.into_iter()).collect();

    ```
    Note that the type annotation `HashMap<_, _>` is needed here because it's possible to `collect` into many different data structures.
* For types that implement the `Copy` trait, like `i32`, the values are copied into the hash map. For owned values like `String`, the values will be moved and the hash map will be the owner of those values [[ch08-03](https://doc.rust-lang.org/book/ch08-03-hash-maps.html#hash-maps-and-ownership)]:
    ```rust
    use std::collections::HashMap;

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // `field_name` and `field_value` are invalid at this point
    ```
    If we insert references to values into the hash map, the values won't be moved into the hash map. However, the values that the references point to must be valid for at least as long as the hash map is valid.
* To check whether a particular key has a value and, if it doesn't, insert a value for it, hash maps have a special API for this called `entry` that takes the key you want to check as a parameter [[ch08-03](https://doc.rust-lang.org/book/ch08-03-hash-maps.html#only-inserting-a-value-if-the-key-has-no-value)]:
    ```rust
    use std::collections::HashMap;

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores); // prints: {"Yellow": 50, "Blue": 10}
    ```
    The return value of the `entry` method is an enum called `Entry`, and its `or_insert` method is defined to return a mutable reference to the value for the corresponding `Entry` key if that key exists, and if not, inserts the parameter as the new value for this key and returns a mutable reference to the new value.
    ```rust
    use std::collections::HashMap;

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map); // prints: {"world": 2, "hello": 1, "wonderful": 1}
    ```

## Traits
* To define a function which takes some type with a given trait as a parameter we can use the `impl Trait` syntax [[ch10-02](https://doc.rust-lang.org/book/ch10-02-traits.html#traits-as-parameters)]:
    ```rust
    pub trait Summary {
        fn summarize(&self) -> String;
    }

    pub fn notify(item: &impl Summary) {
        println!("Breaking news! {}", item.summarize());
    }
    ```
    The `impl Trait` syntax works for straightforward cases but is actually syntax sugar for a longer form, which is called a *trait bound*; it looks like this [[ch10-02](https://doc.rust-lang.org/book/ch10-02-traits.html#trait-bound-syntax)]:
    ```rust
    pub fn notify<T: Summary>(item: &T) {
        println!("Breaking news! {}", item.summarize());
    }
    ```
* To specify more than one trait bound, we can use the `+` syntax [[ch10-02](https://doc.rust-lang.org/book/ch10-02-traits.html#specifying-multiple-trait-bounds-with-the--syntax)]:
    ```rust
    pub fn notify(item: &(impl Summary + Display)) { /* ... */ }
    // or
    pub fn notify<T: Summary + Display>(item: &T) { /* ... */ }
    ```
* Rust also has an alternate syntax for specifying trait bounds inside a `where` clause after the function signature, so instead of writing this [[ch10-02](https://doc.rust-lang.org/book/ch10-02-traits.html#clearer-trait-bounds-with-where-clauses)]:
    ```rust
    fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
    ```
    we could write the following, which is less cluttered:
    ```rust
    fn some_function<T, U>(t: &T, u: &U) -> i32
        where T: Display + Clone,
            U: Clone + Debug
    {
    ```
* By using a trait bound with an `impl` block that uses generic type parameters, we can implement methods conditionally for types that implement the specified traits [[ch10-02](https://doc.rust-lang.org/book/ch10-02-traits.html#using-trait-bounds-to-conditionally-implement-methods)]:
    ```rust
    use std::fmt::Display;

    struct Pair<T> {
        x: T,
        y: T,
    }

    impl<T> Pair<T> {
        fn new(x: T, y: T) -> Self {
            Self { x, y }
        }
    }

    impl<T: Display + PartialOrd> Pair<T> {
        fn cmp_display(&self) {
            if self.x >= self.y {
                println!("The largest member is x = {}", self.x);
            } else {
                println!("The largest member is y = {}", self.y);
            }
        }
    }
    ```
    In the example above, `Pair<T>` only implements the `cmp_display` method if its inner type `T` implements the `PartialOrd` trait *and* the `Display` trait [[ch10-02](https://doc.rust-lang.org/book/ch10-02-traits.html#using-trait-bounds-to-conditionally-implement-methods)]
* We can also conditionally implement a trait for any type that implements another trait. Implementations of a trait on any type that satisfies the trait bounds are called *blanket implementations*. For example, the standard library implements the `ToString` trait on any type that implements the `Display` trait:
    ```rust
    impl<T: Display> ToString for T {
        // ...
    }
    ```

## Lifetimes
* Lifetime annotations describe the relationships of the lifetimes of multiple references to each other without affecting the lifetimes [[ch10-03](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html#lifetime-annotation-syntax)]
* One lifetime annotation by itself doesn't have much meaning, because the annotations are meant to tell Rust how generic lifetime parameters of multiple references relate to each other
    ```rust
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }
    ```
    The constraint expressed in this signature is that all the references in the parameters and the return value must have the same lifetime. In practice, it means that the lifetime of the reference returned by the `longest` function is the same as the smaller of the lifetimes of the references passed in [[ch10-03](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html#lifetime-annotations-in-function-signatures)]
* In the example above, when we pass concrete references to `longest`, the concrete lifetime that is substituted for `'a` is the part of the scope of `x` that overlaps with the scope of `y`
    ```rust
    fn main() {
        let string1 = String::from("long string is long"); // valid until the end of the outer scope

        {
            let string2 = String::from("xyz"); // valid until the end of the inner scope
            let result = longest(string1.as_str(), string2.as_str());
            println!("The longest string is {}", result);
        }
    }
    ```
    Since `result` references something that is valid until the end of the inner scope, the borrow checker approves of it, and it will successfully compile. However, if we tried the following:
    ```rust
    fn main() {
        let string1 = String::from("long string is long");
        let result;
        {
            let string2 = String::from("xyz");
            result = longest(string1.as_str(), string2.as_str());
        }
        println!("The longest string is {}", result);
    }
    ```
    I wouldn't compile, as `string2` does not live long enough (i.e. it is dropped at the end of the inner scope while still borrowed).
* Lifetimes on function or method parameters are called *input lifetimes*, and lifetimes on return values are called *output lifetimes*. The Rust compiler has a few *lifetime elision* rules that allow it to infer what lifetimes references have when there aren't explicit annotations [[ch10-03](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html#lifetime-elision)]:
    1. Each parameter that is a reference gets its own lifetime parameter
    2. If there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters
    3. If there are multiple input lifetime parameters, but one of them is `&self` or `&mut self` because this is a method, the lifetime of `self` is assigned to all output lifetime parameters
* Here's an example function signature without lifetime annotations:
    ```rust
    fn first_word(s: &str) -> &str {

    // Compiler applies the first rule, giving each parameter its own lifetime:
    fn first_word<'a>(s: &'a str) -> &str {

    // Compiler applies the second rule, because there is exactly one input lifetime:
    fn first_word<'a>(s: &'a str) -> &'a str {
    ```
    Now all the references in this function signature have lifetimes. Looking at another example:
    ```rust
    fn longest(x: &str, y: &str) -> &str {

    // Applying the first rule, each parameter gets its own lifetime:
    fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &str {

    // Note that the second rule doesn't apply, because there is more than one input lifetime.
    // The third rule doesn't apply either, because `longest` is a function rather than a method.
    ```
    After working through all three rules, we still couldn't figure out what the return type's lifetime is, hence why this gives a compiler error.
* One special lifetime is `'static`, which means that this reference *can* live for the entire duration of the program [[ch10-03](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html#the-static-lifetime)]

## Tests
* You can use the `Result<T, E>` type as a return for test functions instead of panicking, so that you can use the question mark operator in the body of tests [[ch11-01](https://doc.rust-lang.org/book/ch11-01-writing-tests.html#using-resultt-e-in-tests)]
* Some command line options go to `cargo test`, and some go to the resulting test binary. To separate these two types of arguments, you list the arguments that go to `cargo test` followed by the separator `--` and then the ones that go to the test binary [[ch11-02](https://doc.rust-lang.org/book/ch11-02-running-tests.html#controlling-how-tests-are-run)]
* By default, if a test passes, Rust's test library captures anything printed to standard output. If you want to, you can tell Rust to also show the output of successful tests at the end with `cargo test -- --show-output` [[ch11-02](https://doc.rust-lang.org/book/ch11-02-running-tests.html#showing-function-output)]
* We can specify part of a test name, and any test whose name matches that value will be run. For example [[ch11-02](https://doc.rust-lang.org/book/ch11-02-running-tests.html#filtering-to-run-multiple-tests)]:
    ```rust
    pub fn add_two(a: i32) -> i32 {
        a + 2
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn add_two_and_two() { assert_eq!(4, add_two(2)); }

        #[test]
        fn add_three_and_two() { assert_eq!(5, add_two(3)); }

        #[test]
        fn one_hundred() { assert_eq!(102, add_two(100)); }
    }
    ```
    Running `cargo test add` will filter out the test named `one_hundred`.
* Using the `#[ignore]` attribute you can ignore some tests by default. Then, if you want to run only the ignored tests you can use `cargo test -- --ignored` [[ch11-02](https://doc.rust-lang.org/book/ch11-02-running-tests.html#ignoring-some-tests-unless-specifically-requested)]
* There are two main categories: *unit tests* and *integration tests* [[ch11-03](https://doc.rust-lang.org/book/ch11-03-test-organization.html)]
    * The purpose of unit tests is to test each unit of code in isolation, and you'll ususally put them in the *src* directory in each file with the code that they're testing [[ch11-03](https://doc.rust-lang.org/book/ch11-03-test-organization.html#unit-tests)]
    * In Rust, integration tests are entirely external to your library, with the purpose of testing whether many parts of your library work together correctly. To create them, you first need a *tests* directory at the top level of a project, next to *src* [[ch11-03](https://doc.rust-lang.org/book/ch11-03-test-organization.html#unit-tests)]
    * If our project is a binary crate that only contains a *src/main.rs* file and doesn't have a *src/lib.rs* file, we can't create integration tests in the tests directory and bring functions defined in the *src/main.rs* file into scope with a use statement. Only library crates expose functions that other crates can use; binary crates are meant to be run on their own [[ch11-03](https://doc.rust-lang.org/book/ch11-03-test-organization.html#integration-tests-for-binary-crates)]
    * This is one of the reasons Rust projects that provide a binary have a straightforward *src/main.rs* file that calls logic that lives in the *src/lib.rs* file. Using that structure, integration tests can test the library crate with `use` to make the important functionality available

## Iterators
* Iterators are *lazy*, meaning they have no effect until you call methods that consume the iterator to use it up [[ch13-02](https://doc.rust-lang.org/book/ch13-02-iterators.html#processing-a-series-of-items-with-iterators)]
* All iterators implement the `Iterator` trait, defined in the standard library, which looks like [[ch13-02](https://doc.rust-lang.org/book/ch13-02-iterators.html#processing-a-series-of-items-with-iterators)]:
    ```rust
    pub trait Iterator {
        type Item;

        fn next(&mut self) -> Option<Self::Item>;

        // Methods with default implementations elided...
    }
    ```
* The `iter` method produces an iterator over immutable references. If we want to create an iterator that takes ownership of `v1` and returns owned values, we can call `into_iter` instead. Similarly, if we want to iterate over mutable references, we can call `iter_mut` [[ch13-02](https://doc.rust-lang.org/book/ch13-02-iterators.html#the-iterator-trait-and-the-next-method)]:
    ```rust
    #[test]
    fn iterator_demonstration() {
        let v1 = vec![1, 2, 3];

        let mut v1_iter = v1.iter();

        assert_eq!(v1_iter.next(), Some(&1));
        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), Some(&3));
        assert_eq!(v1_iter.next(), None);
    }
    ```
    Note that we needed to make `v1_iter` mutable: calling the next method on an iterator changes internal state that the iterator uses to keep track of where it is in the sequence. In other words, this code *consumes*, or uses up, the iterator.
* Methods that call next are called *consuming adaptors*, because calling them uses up the iterator [[ch13-02](https://doc.rust-lang.org/book/ch13-02-iterators.html#methods-that-consume-the-iterator)]
* Other methods defined on the `Iterator` trait, known as *iterator adaptors*, allow you to change iterators into different kinds of iterators. You can chain multiple calls to iterator adaptors to perform complex actions in a readable way [[ch13-20](https://doc.rust-lang.org/book/ch13-02-iterators.html#methods-that-produce-other-iterators)]

## Documentation
* Documentation comments use three slashes, `///`, instead of two and support Markdown notation for formatting the text. Place documentation comments just before the item they're documenting [[ch14-02](https://doc.rust-lang.org/book/ch14-02-publishing-to-crates-io.html#making-useful-documentation-comments)]:
    ```rust
    /// Adds one to the number given.
    ///
    /// # Examples
    ///
    /// ```
    /// let arg = 5;
    /// let answer = my_crate::add_one(arg);
    ///
    /// assert_eq!(6, answer);
    /// ```
    pub fn add_one(x: i32) -> i32 {
        x + 1
    }
    ```
    Besides "Examples", other commonly used sections are "Panics", "Errors" and "Safety" [[ch14-02](https://doc.rust-lang.org/book/ch14-02-publishing-to-crates-io.html#commonly-used-sections)]
* Running `cargo test` will run the code examples in your documentation as tests [[ch14-02](https://doc.rust-lang.org/book/ch14-02-publishing-to-crates-io.html#documentation-comments-as-tests)]
* Another style of doc comment, `//!`, adds documentation to the item that contains the comments rather than adding documentation to the items following the comments. We typically use these doc comments inside the crate root file (*src/lib.rs* by convention) or inside a module to document the crate or the module as a whole [[ch14-02](https://doc.rust-lang.org/book/ch14-02-publishing-to-crates-io.html#commenting-contained-items)]

## Smart Pointers
* In Rust, which uses the concept of ownership and borrowing, an additional difference between references and smart pointers is that references are pointers that only borrow data; in contrast, in many cases, smart pointers *own* the data they point to [[ch15-00](https://doc.rust-lang.org/book/ch15-00-smart-pointers.html)]
* The characteristic that distinguishes a smart pointer from an ordinary struct is that smart pointers implement the `Deref` and `Drop` traits:
    * `Deref` allows an instance of the smart pointer struct to behave like a reference
    * `Drop` allows you to customize the code that is run when an instance of the smart pointer goes out of scope
* The most common smart pointers in the standard library are:
    * `Box<T>` for allocating values on the heap
    * `Rc<T>`, a reference counting type that enables multiple ownership
    * `Ref<T>` and `RefMut<T>`, accessed through `RefCell<T>`, a type that enforces the borrowing rules at runtime instead of compile time
* Here are the main reasons for choosing them [[ch15-05](https://doc.rust-lang.org/book/ch15-05-interior-mutability.html#enforcing-borrowing-rules-at-runtime-with-refcellt)]:
    * `Rc<T>` enables multiple owners of the same data; `Box<T>` and `RefCell<T>` have single owners
    * `Box<T>` allows immutable or mutable borrows checked at compile time; `Rc<T>` allows only immutable borrows checked at compile time; `RefCell<T>` allows immutable or mutable borrows checked at runtime
    * Because `RefCell<T>` allows mutable borrows checked at runtime, you can mutate the value inside the `RefCell<T>` even when the `RefCell<T>` is immutable
* *Boxes* provide only the indirection and heap allocation; they don't have any other special capabilities. They also don't have any performance overhead that these special capabilities incur [[ch15-01](https://doc.rust-lang.org/book/ch15-01-box.html#computing-the-size-of-a-non-recursive-type)]
### Deref and Drop
* The `Deref` trait, provided by the standard library, requires us to implement one method named `deref` that borrows `self` and returns a reference to the inner data [[ch15-02](https://doc.rust-lang.org/book/ch15-02-deref.html#treating-a-type-like-a-reference-by-implementing-the-deref-trait)]
    ```rust
    struct MyBox<T>(T);
    impl<T> MyBox<T> {
        fn new(x: T) -> MyBox<T> { MyBox(x) }
    }

    use std::ops::Deref;
    impl<T> Deref for MyBox<T> {
        type Target = T;
        fn deref(&self) -> &T { &self.0 }
    }

    fn main() {
        let x = 5;
        let y = MyBox::new(x);

        assert_eq!(5, x);
        assert_eq!(5, *y);
    }
    ```
    In the last line, when we write `*y`, Rust actually substitutes it with `*(y.deref())`. The reason the `deref` method returns a reference to a value, and that the plain dereference outside the parentheses in `*(y.deref())` is still necessary, is the ownership system. If it returned the value directly instead of a reference to the value, the value would be moved out of `self`.
* *Deref coercion* is a convenience that Rust performs on arguments to functions and methods for types that implement the `Deref` trait, converting it into a reference to another type [[ch15-01](https://doc.rust-lang.org/book/ch15-02-deref.html#implicit-deref-coercions-with-functions-and-methods)]
* Rust does deref coercion when it finds types and trait implementations in three cases [[ch15-02](https://doc.rust-lang.org/book/ch15-02-deref.html#how-deref-coercion-interacts-with-mutability)]:
    * From `&T` to `&U` when `T: Deref<Target=U>`
    * From `&mut T` to `&mut U` when `T: DerefMut<Target=U>`
    * From `&mut T` to `&U` when `T: Deref<Target=U>`
* Disabling `drop` isn't usually necessary; the whole point of the `Drop` trait is that it's taken care of automatically. Occasionally, however, you might want to clean up a value early (e.g. when using smart pointers that manage locks). In this cases you have to call the `std::mem::drop` function provided by the standard library if you want to force a value to be dropped before the end of its scope [[ch15-03](https://doc.rust-lang.org/book/ch15-03-drop.html#dropping-a-value-early-with-stdmemdrop)]
### Rc<T> and Reference Count
* Using `Rc<T>` allows a single value to have multiple owners, and the reference count ensures that the value remains valid as long as any of the owners still exist [[ch15-04](https://doc.rust-lang.org/book/ch15-04-rc.html#cloning-an-rct-increases-the-reference-count)]
* Every time we call `Rc::clone`, the reference count to the data within `Rc<T>` will increase, and the data won't be cleaned up unless there are zero references to it [[ch15-04](https://doc.rust-lang.org/book/ch15-04-rc.html#using-rct-to-share-data)]
* We could also `a.clone()` rather than `Rc::clone(&a)`, but Rust's convention is to use `Rc::clone` in this case, as its implementation doesn't make a deep copy of all the data like most types' implementations of clone do
### RefCell<T> and Interior Mutability
* *Interior mutability* is a design pattern in Rust that allows you to mutate data even when there are immutable references to that data; normally, this action is disallowed by the borrowing rules. To mutate data, the pattern uses `unsafe` code inside a data structure to bend Rust's usual rules that govern mutation and borrowing [[ch15-05](https://doc.rust-lang.org/book/ch15-05-interior-mutability.html)]
* We can use types that use the interior mutability pattern when we can ensure that the borrowing rules will be followed at runtime, even though the compiler can't guarantee that. The `unsafe` code involved is then wrapped in a safe API, and the outer type is still immutable
* Unlike `Rc<T>`, the `RefCell<T>` type represents single ownership over the data it holds. With references and `Box<T>`, the borrowing rules' invariants are enforced at compile time. With `RefCell<T>`, these invariants are enforced at *runtime* [[ch15-05](https://doc.rust-lang.org/book/ch15-05-interior-mutability.html#enforcing-borrowing-rules-at-runtime-with-refcellt)]:
    * At any given time, you can have *either* (but not both of) one mutable reference or any number of immutable references
    * References must always be valid

    Thus, it is useful when you're sure your code follows the borrowing rules but the compiler is unable to understand and guarantee that.
* When creating immutable and mutable references, we use the `&` and `&mut` syntax, respectively. With `RefCell<T>`, we use the `borrow` and `borrow_mut` methods, which are part of the safe API that belongs to `RefCell<T>` [[ch15-05](https://doc.rust-lang.org/book/ch15-05-interior-mutability.html#a-use-case-for-interior-mutability-mock-objects)]
* `borrow` returns the smart pointer type `Ref<T>`, and `borrow_mut` returns the smart pointer type `RefMut<T>`, and `RefCell<T>` keeps track of how many of them are currently active to ensure the borrowing rules at runtime, which protects us from data races
* A common way to use `RefCell<T>` is in combination with `Rc<T>`. Since `Rc<T>` lets you have multiple owners of some data, but it only gives immutable access to that data, by having an `Rc<T>` that holds a `RefCell<T>`, you can get a value that can have multiple owners and that you can mutate [[ch15-05](https://doc.rust-lang.org/book/ch15-05-interior-mutability.html#having-multiple-owners-of-mutable-data-by-combining-rct-and-refcellt)]
* Rust's memory safety guarantees make it difficult, but not impossible, to accidentally create *memory leaks*. For instance, by using `Rc<T>` and `RefCell<T>` it's possible to create references where items refer to each other in a cycle, so their reference counts will never reach 0, and thus, the values will never be dropped [[ch15-06](https://doc.rust-lang.org/book/ch15-06-reference-cycles.html#reference-cycles-can-leak-memory)]
### Weak<T> and Weak References
* Creating reference cycles is not easily done, but it's not impossible either. If you have similar nested combinations of types with interior mutability and reference counting, you must ensure that you don't create cycles [[ch15-06](https://doc.rust-lang.org/book/ch15-06-reference-cycles.html#creating-a-reference-cycle)]
* Another solution for avoiding reference cycles is reorganizing your data structures so that some references express ownership and some references don't. As a result, you can have cycles made up of some ownership relationships and some non-ownership relationships, and only the ownership relationships affect whether or not a value can be dropped
* You can also create a *weak reference* to the value within an `Rc<T>` instance by calling `Rc::downgrade` and passing a reference to it, which returns a smart pointer of type `Weak<T>` [[ch15-06](https://doc.rust-lang.org/book/ch15-06-reference-cycles.html#preventing-reference-cycles-turning-an-rct-into-a-weakt)]
* Instead of increasing the `strong_count` in the `Rc<T>` instance by 1, calling `Rc::downgrade` increases the `weak_count` by 1, which doesn't need to be 0 for the `Rc<T>` instance to be cleaned up
* Because the value that `Weak<T>` references might have been dropped, to do anything with the value it's pointing to you must make sure the value still exists. You do this by calling the `upgrade` method on a `Weak<T>` instance, which will return an `Option<Rc<T>>`

## Fearless Concurrency
* *Concurrent programming* is where different parts of a program execute independently, and *parallel programming* is where different parts of a program execute at the same time [[ch16-00](https://doc.rust-lang.org/book/ch16-00-concurrency.html)]
* Programming languages implement threads in a few different ways [[ch16-06](https://doc.rust-lang.org/book/ch16-01-threads.html#using-threads-to-run-code-simultaneously)]:
    * Many operating systems provide an API for creating new threads, and the model where a language calls it to create threads is sometimes called *1:1*, meaning one OS thread per one language thread
    * Many programming languages provide their own special implementation of threads, which are known as *green* threads, and these languages will execute them in the context of a different number of OS threads. For this reason, the green-threaded model is called the *M:N* model

    The green-threading M:N model requires a larger language *runtime* to manage threads. As such, the Rust standard library only provides an implementation of 1:1 threading.
* To create a new thread, we call the `thread::spawn` function and pass it a closure containing the code we want it to run [[ch16-01](https://doc.rust-lang.org/book/ch16-01-threads.html#creating-a-new-thread-with-spawn)]:
    ```rust
    use std::thread;
    use std::time::Duration;

    fn main() {
        thread::spawn(|| {
            for i in 1..10 {
                println!("hi number {} from the spawned thread!", i);
                thread::sleep(Duration::from_millis(1));
            }
        });

        for i in 1..5 {
            println!("hi number {} from the main thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    }
    ```
    Note that with this function, the new thread will be stopped when the main thread ends, whether or not it has finished running. We can however wait for it by calling `join` on the `JoinHandle` returned by `thread::spawn`:
    ```rust
    use std::thread;
    use std::time::Duration;

    fn main() {
        let handle = thread::spawn(|| {
            for i in 1..10 {
                println!("hi number {} from the spawned thread!", i);
                thread::sleep(Duration::from_millis(1));
            }
        });

        for i in 1..5 {
            println!("hi number {} from the main thread!", i);
            thread::sleep(Duration::from_millis(1));
        }

        handle.join().unwrap();
    }
    ```
    Calling `join` on the handle blocks the thread currently running until the thread represented by the handle terminates.
* The `move` closure is often used alongside `thread::spawn` because it allows you to use data from one thread in another thread [[ch16-01](https://doc.rust-lang.org/book/ch16-01-threads.html#using-move-closures-with-threads)]:
    ```rust
    use std::thread;

    fn main() {
        let v = vec![1, 2, 3];

        let handle = thread::spawn(|| { println!("Here's a vector: {:?}", v); });
        handle.join().unwrap();
    }
    ```
    The code above won't compile because Rust *infers* how to capture `v`, and since `println!` only needs a reference to `v`, the closure tries to borrow it. However, Rust can't tell how long the spawned thread will run, so it doesn't know if the reference will always be valid.

    By adding the move keyword before the closure, we force the closure to take ownership of the values it's using rather than allowing Rust to infer that it should borrow the values:
    ```rust
    use std::thread;

    fn main() {
        let v = vec![1, 2, 3];

        let handle = thread::spawn(move || { println!("Here's a vector: {:?}", v); });
        handle.join().unwrap();
    }
    ```
### Message-Passing Concurrency
* *Message-passing* concurrency is where channels send messages between threads [[ch16-00](https://doc.rust-lang.org/book/ch16-00-concurrency.html)]
* One increasingly popular approach to ensuring safe concurrency is *message passing*, where threads or actors communicate by sending each other messages containing data. One major tool Rust has for accomplishing message-sending concurrency is the *channel*, a programming concept that Rust's standard library provides an implementation of [[ch16-02](https://doc.rust-lang.org/book/ch16-02-message-passing.html#using-message-passing-to-transfer-data-between-threads)]
* A channel in programming has two halves: a *transmitter* and a *receiver*. One part of your code calls methods on the transmitter with the data you want to send, and another part checks the receiving end for arriving messages. A channel is said to be *closed* if either the transmitter or receiver half is dropped
* We create a new channel using the `mpsc::channel` function (`mpsc` stands for *multiple producer, single consumer*), which returns a tuple, the first element of which is the sending end and the second element is the receiving end:
    ```rust
    use std::sync::mpsc;
    use std::thread;

    fn main() {
        let (tx, rx) = mpsc::channel(); // (transmitter, receiver)

        thread::spawn(move || {
            let val = String::from("hi");
            tx.send(val).unwrap();
        });

        let received = rx.recv().unwrap();
        println!("Got: {}", received);
    }
    ```
    The `send` method returns a `Result<T, E>` type, so if the receiving end has already been dropped and there's nowhere to send a value, the send operation will return an error.
* The receiving end of a channel has two useful methods: `recv` and `try_recv`:
    * `recv` will block the thread's execution and wait until a value is sent down the channel. Once a value is sent, `recv` will return it in a `Result<T, E>`. When the sending end of the channel closes, `recv` will return an error to signal that no more values will be coming
    * `try_recv` doesn't block, but will instead return a `Result<T, E>` immediately: an `Ok` value holding a message if one is available and an `Err` value if there aren't any messages this time
### Shared-State Concurrency
* *Shared-state* concurrency is where multiple threads have access to some piece of data [[ch16-00](https://doc.rust-lang.org/book/ch16-00-concurrency.html)]
* In a way, channels in any programming language are similar to single ownership, because once you transfer a value down a channel, you should no longer use that value. Shared memory concurrency is like multiple ownership: multiple threads can access the same memory location at the same time [[ch16-03](https://doc.rust-lang.org/book/ch16-03-shared-state.html#shared-state-concurrency)]
* *Mutexes* have a reputation for being difficult to use because you have to remember two rules:
    * You must attempt to acquire the lock before using the data
    * When you're done with the data that the mutex guards, you must unlock the data so other threads can acquire the lock

    Therefore, the mutex is described as *guarding* the data it holds via the locking system [[ch16-03](https://doc.rust-lang.org/book/ch16-03-shared-state.html#using-mutexes-to-allow-access-to-data-from-one-thread-at-a-time)]
* To access the data inside a `Mutex<T>`, we use the `lock` method to acquire the lock. This call will block the current thread so it can't do any work until it's our turn to have the lock [[ch16-03](https://doc.rust-lang.org/book/ch16-03-shared-state.html#the-api-of-mutext)]:
    ```rust
    use std::sync::Mutex;

    fn main() {
        let m = Mutex::new(5);

        {
            let mut num = m.lock().unwrap();
            *num = 6;
        } // The lock is automatically released here as `num` gets dropped.

        println!("m = {:?}", m);
    }

    ```
    The call to `lock` would fail if another thread holding the lock panicked. In that case, no one would ever be able to get the lock, so by using `unwrap` we have this thread panic if we're in that situation.
* `Mutex<T>` provides interior mutability, as the `Cell` family does. In the same way we use `RefCell<T>` to allow us to mutate contents inside an `Rc<T>`, we use `Mutex<T>` to mutate contents inside an `Arc<T>` [[ch16-03](https://doc.rust-lang.org/book/ch16-03-shared-state.html#similarities-between-refcelltrct-and-mutextarct)]
* While using `Rc<T>` comes with the risk of creating reference cycles, where two `Rc<T>` values refer to each other (causing memory leaks), similarly, `Mutex<T>` comes with the risk of creating *deadlocks*
### Sync and Send
* Two concurrency concepts are embedded in the language: the `std::marker` traits `Sync` and `Send` [[ch16-04](https://doc.rust-lang.org/book/ch16-04-extensible-concurrency-sync-and-send.html)]
* The `Send` marker trait indicates that ownership of the type implementing `Send` can be transferred between threads
* Any type composed entirely of `Send` types is automatically marked as `Send` as well. Almost all primitive types are `Send`, aside from raw pointers, and `Rc<T>` is also notably not `Send`
* The `Sync` marker trait indicates that it is safe for the type implementing `Sync` to be referenced from multiple threads. In other words, any type `T` is `Sync` if `&T` (a reference to `T`) is `Send`, meaning the reference can be sent safely to another thread [[ch16-04](https://doc.rust-lang.org/book/ch16-04-extensible-concurrency-sync-and-send.html#allowing-access-from-multiple-threads-with-sync)]
* Similarly, primitive types are `Sync`, and types composed entirely of types that are `Sync` are also `Sync`. `Rc<T>` is also not `Sync` for the same reasons that it's not `Send`, and `RefCell<T>` and the family of related `Cell` types aren't either
* While the implementation of borrow checking that `RefCell<T>` does at runtime is not thread-safe, the smart pointer `Mutex<T>` is `Sync` and can be used to share access with multiple threads

## OOP and Trait Objects
* There is no way to define a struct that inherits the parent struct's fields and method implementations. However, if you're used to having *inheritance*, you can use other solutions in Rust [[ch17-01](https://doc.rust-lang.org/book/ch17-01-what-is-oo.html#inheritance-as-a-type-system-and-as-code-sharing)]:
    * If you choose inheritance for reuse of code: you can share Rust code using *default trait method implementations* instead
    * Another reason relates to the type system: to enable a child type to be used in the same places as the parent type. Instead of providing inheritance, Rust uses generics to abstract over different possible types and trait bounds to impose constraints on what those types must provide (this is sometimes called *bounded parametric polymorphism*)
* A *trait object* points to both an instance of a type implementing our specified trait as well as a table used to look up trait methods on that type at runtime [[ch17-02](https://doc.rust-lang.org/book/ch17-02-trait-objects.html)]
* We create a trait object by specifying some sort of pointer, such as a `&` reference or a `Box<T>` smart pointer, then the `dyn` keyword, and then specifying the relevant trait:
    ```rust
    pub trait Draw {
        fn draw(&self);
    }

    pub struct Screen {
        pub components: Vec<Box<dyn Draw>>,
    }

    impl Screen {
        pub fn run(&self) {
            for component in self.components.iter() {
                component.draw();
            }
        }
    }
    ```
    `Box<dyn Draw>` is a trait object; it's a stand-in for any type inside a `Box` that implements the `Draw` trait.
* A generic type parameter can only be substituted with one concrete type at a time, whereas trait objects allow for multiple concrete types to fill in for the trait object at runtime. For instance, if we had defined the `Screen` struct using a generic type and a trait bound:
    ```rust
    pub struct Screen<T: Draw> {
        pub components: Vec<T>,
    }

    impl<T> Screen<T>
    where
        T: Draw,
    {
        pub fn run(&self) {
            for component in self.components.iter() {
                component.draw();
            }
        }
    }
    ```
    This would restrict us to a `Screen` instance that has a homogeneous list of components (i.e. all with the same type).
* Trait objects are more like objects in other languages in the sense that they combine data and behavior (which `impl` blocks keep separate for enums and structs). But trait objects differ from traditional objects in that we can't add data to a trait object [[ch17-02](https://doc.rust-lang.org/book/ch17-02-trait-objects.html#defining-a-trait-for-common-behavior)]
* The advantage of using trait objects and Rust's type system to write code similar to code using *duck typing* is that we never have to check whether a value implements a particular method at runtime or worry about getting errors if a value doesn't implement a method but we call it anyway, because the code simply wouldn't compile [[ch17-02](https://doc.rust-lang.org/book/ch17-02-trait-objects.html#implementing-the-trait)]
* When we use trait bounds on generics the compiler generates nongeneric implementations of functions and methods for each concrete type that we use in place of a generic type parameter. The code that results from monomorphization is doing *static dispatch*, which is when the compiler knows what method you're calling at compile time. This is opposed to *dynamic dispatch*, which is when the compiler can't tell at compile time which method you're calling, and it's what happens when we use trait objects [[ch17-02](https://doc.rust-lang.org/book/ch17-02-trait-objects.html#trait-objects-perform-dynamic-dispatch)]
* You can only make *object-safe* traits into trait objects, which requires that all methods in the trait have the following properties [[ch17-02](https://doc.rust-lang.org/book/ch17-02-trait-objects.html#object-safety-is-required-for-trait-objects)]:
    * The return type isn't `Self`
    * There are no generic type parameters

## Patterns
* We can mix, match, and nest destructuring patterns in even more complex ways [[ch18-03](https://doc.rust-lang.org/book/ch18-03-pattern-syntax.html#destructuring-structs-and-tuples)]:
    ```rust
    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });
    ```
* The underscore (`_`) is a wildcard pattern that will match any value but not bind to it. Although it's especially useful as the last arm in a `match` expression, it can be used in any pattern, including function parameters [[ch18-03](https://doc.rust-lang.org/book/ch18-03-pattern-syntax.html#ignoring-an-entire-value-with-_)]:
    ```rust
    fn foo(_: i32, y: i32) {
        println!("This code only uses the y parameter: {}", y);
    }

    fn main() {
        foo(3, 4);
    }
    ```
* There is a subtle difference between using only `_` and using a name that starts with an underscore. The syntax `_x` still binds the value to the variable, whereas `_` doesn't bind at all [[ch18-03](https://doc.rust-lang.org/book/ch18-03-pattern-syntax.html#ignoring-an-unused-variable-by-starting-its-name-with-_)]:
    ```rust
    let s = Some(String::from("Hello!"));

    if let Some(_) = s {
        // `s` isn't moved since `_` doesn't bind to anything.
        println!("found a string");
    }

    println!("{:?}", s);
    ```
    If we had used `if let Some(_s) = s` instead, `s` would be moved into `_s`, preventing us from using it again in the last line.
* With values that have many parts, we can use the `..` syntax to use only a few parts and ignore the rest [[ch18-03](https://doc.rust-lang.org/book/ch18-03-pattern-syntax.html#ignoring-remaining-parts-of-a-value-with-)]:
    ```rust
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, .., last) => {
            println!("Some numbers: {}, {}", first, last);
        }
    }
    ```
* The *at* operator (`@`) lets us create a variable that holds a value at the same time we're testing that value to see whether it matches a pattern [[ch18-03](https://doc.rust-lang.org/book/ch18-03-pattern-syntax.html#-bindings)]:
    ```rust
    enum Message {
        Hello { id: i32 },
    }

    let msg = Message::Hello { id: 5 };

    match msg {
        Message::Hello {
            id: id_variable @ 3..=7,
        } => println!("Found an id in range: {}", id_variable),
        Message::Hello { id: 10..=12 } => println!("Found an id in another range"),
        Message::Hello { id } => println!("Found some other id: {}", id),
    }
    ```
    By specifying `id_variable @` before the range `3..=7`, we're capturing whatever value matched the range while also testing that the value matched the range pattern.


<!--
    Next chapter to read:
    https://doc.rust-lang.org/book/ch19-01-unsafe-rust.html
 -->