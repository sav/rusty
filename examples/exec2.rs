// exec2.rs, Examples from the book "Rust by Example"
// Copyright (C) 2024, Savio Sena <savio.sena@gmail.com>

//! # Pipes
//!
//! The `std::Child` struct represents a running child process, and exposes the
//! `stdin`, `stdout` and `stderr` handles for interaction with the underlying
//! process via pipes.

use std::io::prelude::*;
use std::process::{Command, Stdio};

static PANGRAM: &'static str = "the quick brown fox jumps over the lazy dog\n";

fn main() {
    let mut cmd = if cfg!(target_family = "windows") {
        let mut cmd = Command::new("powershell");
        cmd.arg("-Command")
            .arg("$input | Measure-Object -Line -Word -Character");
        cmd
    } else { // Linux
        Command::new("wc")
    };

    let process = match cmd.stdin(Stdio::piped()).stdout(Stdio::piped()).spawn() {
        Err(why) => panic!("couldn't spawn wc: {why}"),
        Ok(process) => process,
    };

    match process.stdin.unwrap().write_all(PANGRAM.as_bytes()) {
        Err(why) => panic!("couldn't write to wc stdin: {why}"),
        Ok(_) => println!("sent pangram to wc: {PANGRAM}"),
    }

    // Because `stdin` does not live after the above calls, it is `drop`ed, and
    // the pipe is closed. This is very important, otherwise `wc` wouldn't start
    // processing the input we just sent.

    let mut s = String::new();
    match process.stdout.unwrap().read_to_string(&mut s) {
        Err(why) => panic!("couldn't read wc stdout: {why}"),
        Ok(_) => print!("wc responded with:\n{s}"),
    }

    // Wait
    let mut child = Command::new("sleep").arg("2s").spawn().unwrap();
    let _ = child.wait().unwrap();
    println!("reached end of main");
}
