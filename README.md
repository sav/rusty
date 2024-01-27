Rusty v0.1.3
============

Some Rust snippets I wrote to learn the language and eventually to serve as
a reminder for the future me. All the following code was either copied from
or inspired in this great book:

 * [The "Rust programming language" Book](https://doc.rust-lang.org/book)

Usage
-----

```
cargo run --release
```

Examples
--------

```
cargo run --example <example>
```

To see which examples are available list the contents of `example` directory.

Development
-----------

To check the package:

```
cargo check
```

To run clippy (rust lint):

```
cargo clippy
```

Fix the warnings:

```
cargo clippy --fix
```

A more advanced (and real case scenario) of Clippy use:

```
cargo clippy --all-targets --all-features -- -D warnings
```

To build in development mode:

```
cargo build
```

To run the test suites:

```
cargo test
```

Finally, to view the project dependency tree:

```
cargo modules generate tree --with-types
```
