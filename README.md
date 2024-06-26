Rusty
=====

This repository contains a collection of **Rust code snippets** inspired by **The Rust Programming Language** book and a few other references (see below). The primary purpose of this repository is to serve as a **personal reference**, helping me remember important concepts and techniques from these sources. The code might be a bit messy, and you'll eventually find some commented-out sections that serve as a reminder of how **not** to do things. But fear not! It could still be helpful for you to take a look at it, especially if you are short on time and won't go through the entire book yourself.

 * [The "Rust Programming Language" Book](https://doc.rust-lang.org/book)
 * [Rust by Example](https://doc.rust-lang.org/stable/rust-by-example/)

 There's an _alternative_ version of the "Rust Programming Language" book with _interactive quizzes_ available [here](https://rust-book.cs.brown.edu/).

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
cargo lint 
cargo fix
```
