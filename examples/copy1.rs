// copy1.rs, Examples from the book "Rust by Example"
// Copyright (C) 2024, Savio Sena <savio.sena@gmail.com>

/// When a type implements Copy, it forfeits its move semantics entirely.

#[derive(Debug,Copy,Clone)]
struct A {
    i: i64,
}

fn main() {
    let mut x = A {i: 1};
    let y = move || -> A { x }(); // don't move, copy.
    println!("{:?}", x);
    x.i = 10;
    println!("{:?}", x);
    println!("{:?}", y);
}
