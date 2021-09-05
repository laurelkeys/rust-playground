# rust-playground

Trying out this Rust thing... let's see how it goes 🦀

## book[`/`](book/)
Notes on [_the book_](https://doc.rust-lang.org/book/) "The Rust Programming Language", together with code examples.

```bash
.
│
├── ch02-guessing-game/...
├── ch03-fibonacci/...
├── ch08-mean-median-mode/...
├── ch08-pig-latin/...
├── ch12-grep/...
├── ch14-add/...
├── ch14-art/...
├── ch15-cons-list/...
├── ch15-mock-object/...
├── ch15-weak-ref-tree/...
├── ch16-concurrency/...
├── ch17-blog/...
├── ch17-gui/...
├── ch19-hello-macro/...
├── ch19-pancakes/...
├── ch20-hello/...
│
└── notes.md            # complete book notes
```

## clair[`/`](clair/)
Really simple command line app example, from the Rust CLI [working group](https://github.com/rust-cli/meta)'s book ["Command Line Applications in Rust"](https://doc.rust-lang.org/book/) (CLAiR).

```bash
.
└── grrs/               # super small grep clone ("grass")
    ├── src/
    │   ├── lib.rs      # find patterns in string content
    │   └── main.rs     # command line interface
    ├── tests/
    │   └── cli.rs      # integration tests
    └── Cargo.toml
```

## crust[`/`](crust/)
Coding along [Jon Gjengset](https://github.com/jonhoo)'s "[Crust of Rust](https://www.youtube.com/playlist?list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa)" video series.

```bash
.
├── strsplit/
│   └── src/lib.rs      # "Crust of Rust: Lifetime Annotations"
│
├── vecmac/
│   └── src/lib.rs      # "Crust of Rust: Declarative Macros"
│
├── iterators/
│   └── src/lib.rs      # "Crust of Rust: Iterators"
│
├── pointers/
│   └── src/            # "Crust of Rust: Smart Pointers and Interior Mutability"
│       ├── lib.rs
│       ├── cell.rs     # a mutable memory location
│       ├── refcell.rs  # a mutable memory location with dynamically checked borrow rules
│       └── rc.rs       # a single-threaded reference-counting pointer
│
├── panama/
│   └── src/lib.rs      # "Crust of Rust: Channels"
│
├── orst/
│   └── src/            # "Crust of Rust: Sorting Algorithms"
│       ├── lib.rs
│       ├── bubblesort.rs
│       ├── insertionsort.rs
│       ├── selectionsort.rs
│       ├── quicksort.rs
│       └── bin/
│           └── bench.rs
│
├── strtok/
|   └── src/lib.rs      # "Crust of Rust: Subtyping and Variance"
│
├── boks/
│   └── src/main.rs     # "Crust of Rust: The Drop Check"
│
└── atomics/
    └── src/main.rs     # "Crust of Rust: Atomics and Memory Ordering"
```

## learn-wgpu[`/`](learn-wgpu/)
Following the "[Learn Wgpu](https://sotrh.github.io/learn-wgpu/#what-is-wgpu)" guide on using [`gfx-rs`'s `wgpu`](https://github.com/gfx-rs/wgpu) library.

## lrtdw[`/`](lrtdw/)
Following [Cliff L. Biffle](http://cliffle.com/about/)'s `unsafe`-first approach to "[Learn Rust the Dangerous Way](http://cliffle.com/p/dangerust/)" (LRtDW) series of articles.

```bash
.
├── nbody-c/
│   └── main.c          # reference C code, unchanged
│
├── nbody-rs/
│   ├── src/
│   │   └── main.rs     # Rust code, following LRtDW
│   └── Cargo.toml
│
└ make.py               # build script
```

## lrwetmll[`/`](lrwetmll/)
Following "[Learn Rust With Entirely Too Many Linked Lists](https://rust-unofficial.github.io/too-many-lists/)" chapters.

```bash
.
└── lists/
    ├── src/
    │   ├── first.rs    # ch. 2, a bad singly-linked stack
    │   ├── second.rs   # ch. 3, an ok singly-linked stack
    │   ├── third.rs    # ch. 4, a persistent singly-linked stack
    │   ├── fourth.rs   # ch. 5, a bad but safe doubly-linked deque
    │   ├── fifth.rs    # ch. 6, an unsafe singly-linked queue
    │   └── lib.rs
    └── Cargo.toml
```

## rusty-journal[`/`](rusty-journal/)
Implementation of [Microsoft Learn](https://docs.microsoft.com/en-us/learn/paths/rust-first-steps/)'s "[Build a command-line to-do list program](https://docs.microsoft.com/en-us/learn/modules/rust-create-command-line-program/)" module.

```bash
.
├── src/
│   ├── cli.rs          # command-line interface using structopt
│   └── main.rs
│   └── tasks.rs        # add, complete and list tasks using serde
└── Cargo.toml
```

## wasm[`/`](wasm/)
Implementation of [Conway](https://en.wikipedia.org/wiki/John_Horton_Conway)'s [Game of Life](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life), following the Rust and WebAssembly [working group](https://rustwasm.github.io/)'s "[Rust Wasm Book](https://rustwasm.github.io/docs/book/)".

```bash
.
└── game-of-life/
    ├── src/            # Rust code with Wasm bindings
    │   ├── lib.rs
    │   └── utils.rs
    ├── www/            # JavaScript code using the generated WebAssembly
    │   ├── index.js
    │   ├── index.html
    │   ├── bootstrap.js
    │   ├── package.json
    │   └── webpack.config.js
    └── Cargo.toml
```