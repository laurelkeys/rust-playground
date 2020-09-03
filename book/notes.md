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

## Match
* A `match` expression is made up of *arms*, which consists of a *pattern* and the code to be run if the value fits that arm's pattern [[ch02-00](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html#comparing-the-guess-to-the-secret-number)]

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


<!--
    Next chapter to read:
    https://doc.rust-lang.org/book/ch10-00-generics.html
 -->