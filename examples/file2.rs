// file2.rs, Examples from the book "Rust by Example"
// Copyright (C) 2024, Savio Sena <savio.sena@gmail.com>

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufRead};
use std::path::Path;
use std::process;

static FILE_PATH: &str = "/tmp/lorem_ipsum.txt";
static FILE_CONTENT: &str =
    "Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do eiusmod
tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam,
quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo
consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse
cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non
proident, sunt in culpa qui officia deserunt mollit anim id est laborum.
";

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() -> Result<(), Box<dyn Error>> {
    let path = Path::new(FILE_PATH);
    let display = path.display();

    {
        let mut file = match File::create(&path) {
            // RAII.
            Err(why) => {
                println!("couldn't create \"{display}\": {why}");
                process::exit(why.raw_os_error().unwrap());
            }
            Ok(file) => file,
        };

        file.write_all(FILE_CONTENT.as_bytes())?;
    } // close `file`.

    if let Ok(lines) = read_lines(FILE_PATH) {
        for line in lines.flatten() {
            println!("{line}");
        }
    }

    Ok(())
}
