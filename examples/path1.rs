// path1.rs, Examples from the book "Rust by Example"
// Copyright (C) 2024, Savio Sena <savio.sena@gmail.com>

use std::path::Path;

/// Be sure to check at other [`Path`] methods ([`posix::Path`] or
/// [`windows::Path`]) and the `Metadata` struct.
fn main() {
    let path = Path::new(".");

    let _display = path.display(); // returns a `Display`able structure.

    // `join` merges a path with a byte container using the OS specific
    // separator, and returns a [`PathBuf`].
    let mut new_path = path.join("a").join("b");

    // `push` extends the `PathBuf` with a `&Path`.
    new_path.push("c");
    new_path.push("myfile.tar.gz");

    // `set_file_name` updates the file name of the `PathBuf`.
    new_path.set_file_name("package.tgz");

    // Convert the `PathBuf` into a string slice.
    match new_path.to_str() {
        // Note that a `Path` is not internally represented as an UTF-8 string,
        // but instead is stored as an `OsString`. Therefore, converting a
        // `Path` to a `&str` is not free and may fail.
        None => panic!("new path is not a valid UTF-8 sequence."),
        Some(s) => println!("new path is {s}"),
    }
}
