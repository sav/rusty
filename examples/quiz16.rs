// quiz16.rs, Examples from the book "Rust by Example"
// Copyright (C) 2024, Savio Sena <savio.sena@gmail.com>

//! # [Rust Quiz #16](https://dtolnay.github.io/rust-quiz/16)
//!
//! *Asnwer*: `44`

#![allow(unused_mut, unused_must_use)]

fn main() {
    let mut x = 4;
    --x;
    print!("{}{}", --x, --x);
}
