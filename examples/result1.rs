// result1.rs, Examples from the book "Rust by Example"
// Copyright (C) 2024, Savio Sena <savio.sena@gmail.com>

use std::num::ParseIntError;

fn ex1_map() {
    fn multiply(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
        first_number_str.parse::<i32>().and_then(|first_number| {
            second_number_str
                .parse::<i32>()
                .map(|second_number| first_number * second_number)
        })
    }

    fn print(result: Result<i32, ParseIntError>) {
        match result {
            Ok(n) => println!("n is {}", n),
            Err(e) => println!("Error: {}", e),
        }
    }

    let twenty = multiply("10", "2");
    print(twenty);

    let tt = multiply("t", "2");
    print(tt);
}

fn ex2_alias() {
    type AliasedResult<T> = Result<T, ParseIntError>;

    fn multiply(first_number_str: &str, second_number_str: &str) -> AliasedResult<i32> {
        first_number_str.parse::<i32>().and_then(|first_number| {
            second_number_str
                .parse::<i32>()
                .map(|second_number| first_number * second_number)
        })
    }

    fn print(result: AliasedResult<i32>) {
        match result {
            Ok(n) => println!("n is {}", n),
            Err(e) => println!("Error: {}", e),
        }
    }

    print(multiply("10", "2"));
    print(multiply("t", "2"));
}

fn ex3_simple_ret() {
    fn multiply(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
        let first_number = first_number_str.parse::<i32>()?;
        let second_number = second_number_str.parse::<i32>()?;
        Ok(first_number * second_number)
    }

    fn print(result: Result<i32, ParseIntError>) {
        match result {
            Ok(n) => println!("n is {}", n),
            Err(e) => println!("Error: {}", e),
        }
    }

    print(multiply("10", "2"));
    print(multiply("t", "2"));
}

/// An `Iter::map` operation might fail, for example:
/// ```no_run
///     let strings = vec!["tofu", "93", "18"];
/// let numbers: Vec<_> = strings
///     .into_iter()
///     .map(|s| s.parse::<i32>())
///     .collect();
/// println!("Results: {:?}", numbers);    
/// ```
/// Let's step through strategies for handling this.
fn ex4_iter() {
    // Ignore the failed items with `filter_map`.

    let strings = vec!["tofu", "93", "18"];
    let numbers: Vec<_> = strings
        .into_iter()
        .filter_map(|s| s.parse::<i32>().ok())
        .collect();
    println!("Results: {:?}", numbers);

    // Collect the failed items with `map_err()` and `filter_map()`.
    //
    // `map_err` calls a function with the error, so by adding that to the
    // previous `filter_map` solution we can save them off to the side while
    // iterating.

    let strings = vec!["42", "tofu", "93", "999", "18"];
    let mut errors = vec![];
    let numbers: Vec<_> = strings
        .into_iter()
        .map(|s| s.parse::<u8>())
        .filter_map(|r| r.map_err(|e| errors.push(e)).ok())
        .collect();
    println!("Numbers: {:?}", numbers);
    println!("Errors: {:?}", errors);

    // Fail the entire operation with `collect()`.
    //
    // `Result` implements `FromIterator` so that a vector of results
    // (`Vec<Result<T, E>>`) can be turned into a result with a vector
    // (`Result<Vec<T>, E>`). Once an `Result::Err` is found, the iteration will
    // terminate.

    let strings = vec!["10", "tofu", "93", "18"];
    let numbers: Result<Vec<_>, _> = strings.into_iter().map(|s| s.parse::<i32>()).collect();
    println!("Results: {:?}", numbers);

    // Collect all valid values and failures with `partition()`.
    let strings = vec!["tofu", "93", "18"];
    let (numbers, errors): (Vec<_>, Vec<_>) = strings
        .into_iter()
        .map(|s| s.parse::<i32>())
        .partition(Result::is_ok);
    println!("Numbers: {:?}", numbers);
    println!("Errors: {:?}", errors);

    // When you look at the results, you'll note that everything is still
    // wrapped in `Result`. A little more boilerplate is needed for this.
    let numbers: Vec<_> = numbers.into_iter().map(Result::unwrap).collect();
    let errors: Vec<_> = errors.into_iter().map(Result::unwrap_err).collect();
    println!("Numbers: {:?}", numbers);
    println!("Errors: {:?}", errors);
}

/// `Result` is a richer version of the `Option` type that describes possible
/// _error_ instead of possible _absence_.
///
/// `Result` implements `map`, `and_then`, and many other combinators that are
/// found in `Option`.

fn main() {
    println!("-=- ex1_map() -=-");
    ex1_map();
    println!("-=- ex2_alias() -=-");
    ex2_alias();
    println!("-=- ex3_simple_ret() -=-");
    ex3_simple_ret();
    println!("-=- ex4_iter() -=-");
    ex4_iter();
}
