// label1.rs, Examples from the book "Rust by Example"
// Copyright (C) 2024, Savio Sena <savio.sena@gmail.com>

fn main() {
    let mut i = 0;

    let n = 'a: loop {
        i += 1;
        if i >= 1000000 {
            break 'a i;
        }
    };

    println!("{n}");
}
