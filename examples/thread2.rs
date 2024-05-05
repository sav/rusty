// thread2.rs, Examples from the book "Rust by Example"
// Copyright (C) 2024, Savio Sena <savio.sena@gmail.com>

//! # Scoped Thread

use std::thread;

fn foo() {
    let s = String::from("Hello");
    thread::scope(|scope| {
        scope.spawn(|| {
            // Normal threads cannot borrow from their environment:
            println!("Length: {}", s.len());
        });
    });
}

fn main() {
    foo();
}
