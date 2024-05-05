// quiz3.rs, Examples from the book "Rust by Example"
// Copyright (C) 2024, Savio Sena <savio.sena@gmail.com>

//! # [Rust Quiz #3](https://dtolnay.github.io/rust-quiz/3)
//!
//! The semantics of `const` is that any mention of the `const` by name in
//! expression position is substituted with the value of the `const`
//! initializer. In this quiz code below the behavior is equivalent to:
//! ```rust,no_run
//! struct S {
//!     x: i32,
//! }
//!
//! fn main() {
//!     let v = &mut S { x: 2 };
//!     v.x += 1;
//!     S { x: 2 }.x += 1;
//!     print!("{}{}", v.x, S { x: 2 }.x);
//! }
//! ```
//!
//! *Answer*: `32`

#![allow(const_item_mutation)]

struct S {
    x: i32,
}

const S: S = S { x: 2 };

fn main() {
    let v = &mut S;
    v.x += 1;
    S.x += 1;
    print!("{}{}", v.x, S.x);
}
