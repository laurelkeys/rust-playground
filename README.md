# rust-playground

Trying out this Rust thing... let's see how it goes 🦀

## `book/`
Notes on [_the book_](https://doc.rust-lang.org/book/) "The Rust Programming Language". 🚧 WIP 🚧

## `lrtdw/`
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

## `lrwetmll/`
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
