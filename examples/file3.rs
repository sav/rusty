// file3.rs, Examples from the book "Rust by Example"
// Copyright (C) 2024, Savio Sena <savio.sena@gmail.com>

//! # Filesystem Operations

use std::fs;
use std::fs::{File, OpenOptions};
use std::io;
use std::io::prelude::*;
#[cfg(target_family = "unix")]
use std::os::unix;
#[cfg(target_family = "windows")]
use std::os::windows;
use std::path::Path;

/// A simple `% cat path`.
fn cat(path: &Path) -> io::Result<String> {
    let mut f = File::open(path)?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

/// A simple `% echo s > path`.
fn echo(s: &str, path: &Path) -> io::Result<()> {
    let mut f = File::create(path)?;

    f.write_all(s.as_bytes())
}

// A simple `% touch path`.
fn touch(path: &Path) -> io::Result<()> {
    match OpenOptions::new().create(true).write(true).open(path) {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

fn main() {
    println!("`mkdir /tmp/a`");
    match fs::create_dir("/tmp/a") {
        Err(why) => println!("! {:?}", why.kind()),
        Ok(_) => {}
    }

    println!("`echo hello > /tmp/a/b.txt`");
    echo("hello", &Path::new("/tmp/a/b.txt")).unwrap_or_else(|why| {
        println!("! {:?}", why.kind());
    });

    println!("`mkdir -p /tmp/a/c/d`");
    fs::create_dir_all("/tmp/a/c/d").unwrap_or_else(|why| {
        println!("! {:?}", why.kind());
    });

    println!("`touch /tmp/a/c/e.txt`");
    touch(&Path::new("/tmp/a/c/e.txt")).unwrap_or_else(|why| {
        println!("! {:?}", why.kind());
    });

    println!("`ln -s ../b.txt /tmp/a/c/b.txt`");
    #[cfg(target_family = "unix")]
    {
        unix::fs::symlink("../b.txt", "/tmp/a/c/b.txt").unwrap_or_else(|why| {
            println!("! {:?}", why.kind());
        });
    }
    #[cfg(target_family = "windows")]
    {
        windows::fs::symlink_file("../b.txt", "/tmp/a/c/b.txt").unwrap_or_else(|why| {
            println!("! {:?}", why.to_string());
        });
    }

    println!("`cat /tmp/a/c/b.txt`");
    match cat(&Path::new("/tmp/a/c/b.txt")) {
        Err(why) => println!("! {:?}", why.kind()),
        Ok(s) => println!("> {}", s),
    }

    println!("`ls /tmp/a`");
    match fs::read_dir("/tmp/a") {
        Err(why) => println!("! {:?}", why.kind()),
        Ok(paths) => {
            for path in paths {
                println!("> {:?}", path.unwrap().path());
            }
        }
    }

    println!("`rm /tmp/a/c/e.txt`");
    fs::remove_file("/tmp/a/c/e.txt").unwrap_or_else(|why| {
        println!("! {:?}", why.kind());
    });

    println!("`rmdir /tmp/a/c/d`");
    fs::remove_dir("/tmp/a/c/d").unwrap_or_else(|why| {
        println!("! {:?}", why.kind());
    });
}
