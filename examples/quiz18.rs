// quiz18.rs, Examples from the book "Rust by Example"
// Copyright (C) 2024, Savio Sena <savio.sena@gmail.com>

//! # [Rust Quiz #18](https://dtolnay.github.io/rust-quiz/18)
//!
//! A call that looks like `.f()` _always_ resolves to a method, in this case
//! the inherent method `S::f`. If there were no method `f` in scope, a call
//! like this would fail to compile even if a field `f` exists and contains a
//! function pointer.
//!
//! To call the function pointer stored in field `f`, we would need to write
//! parentheses around the field access:
//! ```rust,no_run
//! fn main() {
//!    let print2 = || print!("2");
//!    (S { f: print2 }.f)();
//! }
//! ```
//! 
//! *Answer*: `1`

#![allow(dead_code)]

struct S {
    f: fn(),
}

impl S {
    fn f(&self) {
        print!("1");
    }
}

fn main() {
    let print2 = || print!("2");
    S { f: print2 }.f();
}
