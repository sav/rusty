// args1.rs, Examples from the book "Rust by Example"
// Copyright (C) 2024, Savio Sena <savio.sena@gmail.com>

//! # Program arguments.
//!
//! One of the most popular command line argument crates is
//! [clap](https://rust-cli.github.io/book/tutorial/cli-args.html#parsing-cli-arguments-with-clap).

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    println!("My path is {}.", args[0]);
    println!("I got {:?} arguments: {:?}.", args.len() - 1, &args[1..]);
}
