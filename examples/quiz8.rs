// quiz8.rs, Examples from the book "Rust by Example"
// Copyright (C) 2024, Savio Sena <savio.sena@gmail.com>

//! # [Rust Quiz #8](https://dtolnay.github.io/rust-quiz/8)
//!
//! Adjacent punctuation characters in the input pattern of a `macro_rules!`
//! macro are grouped according to how those characters are used by native Rust
//! tokens. The parser of `macro_rules!` will decompose the input into Rust
//! tokens according to a greedy process.
//!
//! Let's decompose the rules in the code below the same way:
//!
//!  - `==>` decomposes as `== >`.
//!  - `= = >` is already decomposed.
//!  - `== >` is already decomposed.
//!  - `= =>` is already decomposed.
//!
//! Our macro is the same as if we had written the first rule with a space. The
//! third rule is unreachable:
//! ```rust,no_run
//! macro_rules! m {
//!    (== >) => { print!("1"); };
//!    (= = >) => { print!("2"); };
//!    (== >) => { print!("3"); };
//!    (= =>) => { print!("4"); };
//! }
//! ```
//!
//! *Asnwer*: `1214`

macro_rules! m {
    (==>) => { print!("1"); };
    (= = >) => { print!("2"); };
    (== >) => { print!("3"); };
    (= =>) => { print!("4"); };
}

fn main() {
    m!(==>);
    m!(= = >);
    m!(== >);
    m!(= =>);
}
