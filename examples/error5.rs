use std::error::Error;
use std::fs::File;
use std::io::Read;

// The main function may return any types that implement the `std::process::Termination` trait,
// which contains a function `report` that returns an `ExitCode`.
// https://doc.rust-lang.org/std/process/trait.Termination.html

fn main() -> Result<(), Box<dyn Error>> {
    let mut s = String::new();
    File::open("/etc/motd")?.read_to_string(&mut s)?; // Box<dyn Error> accepts any kind of Err.
    println!("{} -> {:#?}", s, ());
    Ok(())
}
