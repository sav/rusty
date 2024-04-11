// inference.rs, Examples from the book "Rust by Example"
// Copyright (C) 2024, Savio Sena <savio.sena@gmail.com>

fn main() {
    let elem = 5u8;
    let mut vec = vec![]; // no type.
    vec.push(elem);
    println!("{:?}", vec);
}
