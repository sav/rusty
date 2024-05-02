use std::fs::File;
use std::io::{self, Read};

// long version without the `?` operator.

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("/etc/passwd");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

// much shorter version with the `?` operator.

fn read_username_from_file_short() -> Result<String, io::Error> {
    let mut username_file = File::open("/etc/passwd")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

// using chained `?` operators.

fn read_username_from_file_chained() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

// one-liner, available in the api.
use std::fs;

fn read_username_from_file_oneliner() -> Result<String, io::Error> {
    fs::read_to_string("/etc/passwd")
}

fn main() {
    match read_username_from_file() {
        Ok(s) => println!("{s}"),
        Err(e) => println!("{e}"),
    };

    match read_username_from_file_short() {
        Ok(s) => println!("{s}"),
        Err(e) => println!("{e}"),
    };

    match read_username_from_file_chained() {
        Ok(s) => println!("{s}"),
        Err(e) => println!("{e}"),
    };

    match read_username_from_file_oneliner() {
        Ok(s) => println!("{s}"),
        Err(e) => println!("{e}"),
    };
}
