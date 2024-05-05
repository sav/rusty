// test4.rs, Examples from the book "Rust by Example"
// Copyright (C) 2024, Savio Sena <savio.sena@gmail.com>

//! ## Ignoring tests
//!
//! `cargo test --example test4`
//! `cargo test --example test4 -- --ignored`

#![allow(dead_code)]

pub fn add_v2(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add_v2(2, 2), 4);
    }

    #[test]
    fn test_add_hundred() {
        assert_eq!(add_v2(100, 2), 102);
        assert_eq!(add_v2(2, 100), 102);
    }

    #[test]
    #[ignore]
    fn ignored_test() {
        assert_eq!(add_v2(0, 0), 0);
    }
}

fn main() {
    println!("nothing to do here.");
}
