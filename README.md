Rusty v0.1.5
============

This repository contains a collection of **Rust code snippets** inspired by **The Rust Programming Language** book. The primary purpose of this repository is to serve as a **personal reference** for myself, helping me remember important concepts and techniques. The code might be a bit messy, and you'll eventually find some commented-out sections that serve as a reminder of how **not** to do things. But fear not! It could still be helpful for you to take a look at it, specially if you are short on time and won't go through the entire book
yourself.

 * [The "Rust programming language" Book](https://doc.rust-lang.org/book)

 Also, there's a new version of the book with interactive quizzes available [here](https://rust-book.cs.brown.edu/).

What's Inside?
--------------

- **Snippets**: You'll discover concise code snippets that demonstrate specific Rust features, syntax, or idioms.
- **Examples**: Small, self-contained programs that showcase how to solve common problems using Rust.
- **Comments**: Some sections may have comments explaining why certain approaches were chosen or why specific code blocks exist.

Building
--------

``` sh
cargo build
```

Running
-------

``` sh
cargo run --bin
cargo run --bin rusty
cargo run --bin minigrep
```

Testing
-------

``` sh
cargo test
```

Examples
--------

``` sh
cargo run --example
cargo run --example <example>
```

Documentation
-------------

```sh
cargo doc
cargo doc open
```

Development
-----------

``` sh
cargo check
cargo clippy
```
