// quiz6.rs, Examples from the book "Rust by Example"
// Copyright (C) 2024, Savio Sena <savio.sena@gmail.com>

//! # [Rust Quiz #6](https://dtolnay.github.io/rust-quiz/6)
//!
//! The value being assigned to the second `a` (after shadowing the first) is
//! the expression `a = true`. In Rust, assignment expressions always have the
//! value `()`, so we end up printing `size_of::<()>()`.
//!
//! `()` is one example of a _zero-sized type_ or ZST.
//! 
//! *Answer*: `0`

#![allow(unused_variables, unused_assignments)]

use std::mem;

fn main() {
    let a;
    let a = a = true;
    print!("{}", mem::size_of_val(&a));
}
