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

<!--
    Next chapter to read:
    https://doc.rust-lang.org/book/ch08-00-common-collections.html
 -->