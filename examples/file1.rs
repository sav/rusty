// file1.rs, Examples from the book "Rust by Example"
// Copyright (C) 2024, Savio Sena <savio.sena@gmail.com>

//! `File` returns `io::Result<T>` which is an alias for `Result<T, io::Error>`.

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::process;

fn main() -> Result<(), Box<dyn Error>> {
    let path = Path::new("/tmp/rusty.txt");
    let display = path.display();

    let mut file = match File::open(&path) {
        // RAII.
        Err(why) => {
            println!("couldn't open \"{display}\": {why}");
            process::exit(why.raw_os_error().unwrap());
        }
        Ok(file) => file,
    };

    let mut s = String::new();
    file.read_to_string(&mut s)?;
    print!("{display}: {s}");

    Ok(())
}
