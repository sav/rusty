// minigrep.rs, Examples from the book "Rust Programming Language"
// Copyright (C) 2022-2024, Savio Sena <savio.sena@gmail.com>

//! The `minigrep` crate provides a simple command-line utility for searching text files.
//!
//! # Examples
//!
//! ```rust
//! use std::env;
//!
//! minigrep::run(minigrep::Config {
//!     query: "user".to_string(),
//!     file_path: "/etc/passwd".to_string(),
//!     ignore_case: true
//! });
//! ```

use std::env;
use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build_v0(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        // There’s a tendency among many Rustaceans to avoid using clone to
        // fix ownership problems because of its runtime cost. We'll learn
        // ho to use more efficient methods in this type of situation,
        // but for now, it's okay to copy a few strings to continue making
        // progress.
        let query = args[1].clone();
        let file_path = args[2].clone();
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }

    /// The `Config::build` method constructs a `Config` from an `Iterator<Item = String>`
    /// of arguments.
    ///
    /// # Parameters
    ///
    /// - `args`: An iterator over `String` arguments, usually obtained from `env::args()`.
    ///
    /// # Returns
    ///
    /// A `Result<Config, &'static str>` containing either a valid `Config` struct or an
    /// error message (`&'static str`).
    ///
    /// # Example
    ///
    /// ```rust
    ///     use std::env;
    ///     use std::process;
    ///     use minigrep;
    ///
    ///     match minigrep::Config::build(env::args()) {
    ///        Ok(config) => {
    ///            minigrep::run(config);
    ///        }
    ///        Err(err) => {
    ///            eprintln!("Invalid argument: {err}");
    ///        }
    ///     }
    /// ```

    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        // With our knowledge about iterators, we can change the build function
        // to take ownership of an iterator as its argument instead of borrowing a
        // slice. We’ll use the iterator functionality instead of the code that checks
        // the length of the slice and indexes into specific locations. This will
        // clarify what the `Config::build` function is doing because the iterator will
        // access the values.
        //
        // Once `Config::build` takes ownership of the iterator and stops using indexing
        // operations that borrow, we can move the String values from the iterator into
        // `Config` rather than calling clone and making a new allocation.

        args.next(); // ignore args[0]

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string."),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search(&config.query, &contents)
    } else {
        isearch(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}

// case sensitive search.
pub fn search_v0<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

/// Performs a case-sensitive search for the specified query in the given file.

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // making the code cleaner using iterator adaptors...
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

// case insensitive search.
pub fn isearch_v0<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

/// Performs a case-insensitive search for the specified query in the given file.

pub fn isearch<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    // making the code cleaner using iterator adaptors...
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}

// In case you're wondering how the two implementations compare performance-wise,
// using iterators is slightly faster:
//
// test bench_search_v0  ... bench:  19,620,300 ns/iter (+/- 915,700)
// test bench_search     ... bench:  19,234,900 ns/iter (+/- 657,200)
//
// Iterators are one of Rust's zero-cost abstractions, by which we mean using
// the abstraction imposes no additional runtime overhead. This is analogous to
// how Bjarne Stroustrup, the original designer and implementor of C++, defines
// zero-overhead in “Foundations of C++” (2012):
//
//     "In general, C++ implementations obey the zero-overhead principle:
//     What you don’t use, you don’t pay for. And further: What you do use,
//     you couldn’t hand code any better."
//
// The implementations of closures and iterators are such that runtime performance
// is not affected. This is part of Rust’s goal to strive to provide zero-cost
// abstractions.

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUst";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        assert_eq!(vec!["Rust:", "Trust me."], isearch(query, contents));
    }
}
