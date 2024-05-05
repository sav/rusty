// mutability.rs, Examples from the book "Rust by Example"
// Copyright (C) 2024, Savio Sena <savio.sena@gmail.com>

/// Mutability can be changed when moving ownership.

fn main() {
    let immutable_box = Box::new(0_u32);
    println!("{}", immutable_box);

    let mut mutable_box = immutable_box;
    *mutable_box = 42;

    let immutable_box = mutable_box;
    println!("{}", immutable_box);
}
