// test1.rs, Examples from the book "Rust by Example"
// Copyright (C) 2024, Savio Sena <savio.sena@gmail.com>

//! # Unit testing
//!
//! `cargo test --example test1`

#![allow(dead_code)]

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[allow(dead_code)]
fn bad_add(a: i32, b: i32) -> i32 {
    a - b
}

#[cfg(test)]
mod tests {
    use super::*; // import names from outer scope (for `mod tests`).

    #[test]
    fn test_add() {
        assert_eq!(add(1, 2), 3);
    }

    #[test]
    fn test_bad_add() {
        assert_eq!(bad_add(1, 2), 3); // will fail!
    }
}

fn main() {
    println!("nothing to do here.");
}
