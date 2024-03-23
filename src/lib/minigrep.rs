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

    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
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

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // making the code cleaner using iterator adaptors...
    contents.lines().filter(|line| line.contains(query)).collect()
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

pub fn isearch<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    // making the code cleaner using iterator adaptors...
    contents.lines().filter(|line| line.to_lowercase().contains(&query)).collect()
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
