# rust-playground

Trying out this Rust thing... let's see how it goes 🦀

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
Following "[Learn Rust With Entirely Too Many Linked Lists](https://rust-unofficial.github.io/too-many-lists/)" chapters. 🚧 WIP 🚧

```bash
.
└── lists/
    ├── src/
    │   ├── first.rs    # ch. 2, a bad singly-linked stack
    │   ├── second.rs   # ch. 3, an ok singly-linked stack
    │   └── lib.rs
    └── Cargo.toml
```