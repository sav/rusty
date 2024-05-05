// quiz13.rs, Examples from the book "Rust by Example"
// Copyright (C) 2024, Savio Sena <savio.sena@gmail.com>

//! # [Rust Quiz #13](https://dtolnay.github.io/rust-quiz/13)
//!
//! In this code, `S` is a _zero sized type_ or *ZST*. Zero sized types are
//! compile-time concepts that disappear during compilation and have a runtime
//! representation of _zero bytes_.
//!
//! The first line of `main` creates a local value of type `[S; 2]`. Let's refer
//! to that temporary as `tmp`. The let-binding binds two references into `tmp`,
//! `x` referring to `&mut tmp[0]` and `y` referring to `&mut tmp[1]`.
//!
//! The array type `[S; 2]` is _itself_ a zero sized type. You can confirm this
//! by printing the value of `std::mem::size_of::<[S; 2]>()`.
//!
//! Indeed the first and second element of the array have the _same_ memory
//! address.
//!
//! Ordinarily having multiple mutable references to the same memory location
//! would not be safe, but in the case of mutable references to zero sized
//! types, dereferencing is a no-op so there is no way to violate any memory
//! safety guarantees this way.
//!
//! *Answer*: `1`

struct S;

fn main() {
    let [x, y] = &mut [S, S];
    let eq = x as *mut S == y as *mut S;
    print!("{}", eq as u8);
}
