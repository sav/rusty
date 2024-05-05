// quiz17.rs, Examples from the book "Rust by Example"
// Copyright (C) 2024, Savio Sena <savio.sena@gmail.com>

//! # [Rust Quiz #17](https://dtolnay.github.io/rust-quiz/17)
//!
//! Unlike C or Java, there is _no_ unary increment or decrement operator in
//! Rust.
//!
//! ## Why doesn't Rust have increment and decrement operators?
//!
//! Preincrement and postincrement (and the decrement equivalents), while
//! convenient, are also fairly complex. They require knowledge of evaluation
//! order, and often lead to subtle bugs and undefined behavior in C and C++. `x
//! = x + 1` or `x += 1` is only slightly longer, but unambiguous.
//!
//! In the absence of postfix and prefix decrement operators, `a-- - --b` is
//! parsed as `a - (-(-(-(-b))))`. In the case of `a = 5` and `b = 3` the value
//! of this expression is `5 - 3` which is `2`.
//!
//! *Answer*: `2`

#![allow(unused_mut)]

fn main() {
    let mut a = 5;
    let mut b = 3;
    println!("{}", a-- - --b); // actually: a - (- (- (- (- b))))
}
