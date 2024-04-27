// copy1.rs, Examples from the book "Rust by Example"
// Copyright (C) 2024, Savio Sena <savio.sena@gmail.com>

#![allow(dead_code)]

/// When a type implements Copy, it forfeits its move semantics entirely.

#[derive(Debug,Copy,Clone)]
struct A {
    i: i64,
}

impl A {
    fn printaddr(&self) {
        println!("{:p} <-> {:p}", &self, self);
    }
}

fn main() {
    let mut x = A {i: 1};
    let y = move |x| -> A { x }(x); // don't move, copy.

    println!("{:?}", x);
    x.i = 11;

    println!("{:?}", x);
    println!("{:?}", y);
}
