use std::fs::File;
use std::io::{self, Read};

fn read_user_from_file_syntatic_sugar_chained(path: &str) -> Result<String, io::Error> {
    let mut s = String::new();
    File::open(path)?.read_to_string(&mut s)?;
    Ok(s)
}

fn read_user_from_file_syntatic_sugar(path: &str) -> Result<String, io::Error> {
    let mut f = File::open(path)?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn read_user_from_file(path: &str) -> Result<String, io::Error> {
    let f = File::open(path);

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn main() {
    match read_user_from_file("/tmp/hello.txt") {
        Ok(user) => println!("user(1) = {}", user),
        Err(e) => panic!("error(1): {}", e),
    }
    match read_user_from_file_syntatic_sugar("/tmp/hello.txt") {
        Ok(user) => println!("user(2) = {}", user),
        Err(e) => panic!("error(2): {}", e),
    }
    match read_user_from_file_syntatic_sugar_chained("/tmp/hello.txt") {
        Ok(user) => println!("user(3) = {}", user),
        Err(e) => panic!("error(3): {}", e),
    }
}
