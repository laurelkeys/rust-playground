# lrtdw

Following [Cliff L. Biffle](http://cliffle.com/about/)'s `unsafe`-first approach to "Learn Rust the Dangerous Way" ([LRtDW](http://cliffle.com/p/dangerust/)) series of articles.

```bash
.
├── nbody-c/
│   ├── nbody.c         # reference C code, unchanged
│   └── make.py         # build script
│
└── nbody-rs/
    ├── src/
    │   ├── nbody.rs    # Rust code, following LRtDW
    │   └── make.py     # build script
    ├── Cargo.lock
    └── Cargo.toml
```
