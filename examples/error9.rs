// error9.rs, Examples from the book "Rust by Example"
// Copyright (C) 2024, Savio Sena <savio.sena@gmail.com>

use std::num::ParseIntError;

/// The most basic way of handling mixed error types is to just embed them in
/// each other.

fn ex1_pulling_result_out_of_options() {
    fn double_first(vec: Vec<&str>) -> Option<Result<i32, ParseIntError>> {
        vec.first().map(|first| first.parse::<i32>().map(|n| 2 * n))
    }

    let numbers = vec!["42", "93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    println!("The first doubled is {:?}", double_first(numbers));
    println!("The first doubled is {:?}", double_first(empty)); // Error: vector is empty.
    println!("The first doubled is {:?}", double_first(strings)); // Error: doesn't parse to a number

    // There are times when we want to stop processing on errors (like with `?`)
    // but keep going when the `Option` is `None`. A couple of combinators come
    // in handy to swap the `Result` and `Option`.

    fn double_first_inv(vec: Vec<&str>) -> Result<Option<i32>, ParseIntError> {
        let opt = vec.first().map(|first| first.parse::<i32>().map(|n| 2 * n));

        opt.map_or(Ok(None), |r| r.map(Some))
    }

    let numbers = vec!["42", "93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    println!("The first doubled is {:?}", double_first_inv(numbers));
    println!("The first doubled is {:?}", double_first_inv(empty));
    println!("The first doubled is {:?}", double_first_inv(strings));
}

use std::error;
use std::fmt;

/// Rust allows us to define our own error types. In general, a "good" error type:
///   - Represents different errors with the same type
///   - Presents nice error messages to the user
///   - Is easy to compare with other types
///     - Good: `Err(EmptyVec)`
///     - Bad: `Err("Please use a vector with at least one element".to_owned())`
///   - Can hold information about the error
///     - Good: `Err(BadChar(c, position))`
///     - Bad: `Err("+ cannot be used here".to_owned())`
///   - Composes well with other errors
fn ex2_defining_errtype() {
    type Result<T> = std::result::Result<T, DoubleError>;

    #[derive(Debug, Clone)]
    struct DoubleError;

    /// Generation of an error is completely separate from how it is displayed.
    impl fmt::Display for DoubleError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "invalid first item to double")
        }
    }

    fn double_first(vec: Vec<&str>) -> Result<i32> {
        vec.first()
            .ok_or(DoubleError) // change to our new type.
            .and_then(|s| {
                s.parse::<i32>()
                    .map_err(|_| DoubleError) // update here also
                    .map(|i| 2 * i)
            })
    }

    fn print(result: Result<i32>) {
        match result {
            Ok(n) => println!("The first doubled is {}", n),
            Err(e) => println!("Error: {}", e),
        }
    }

    let numbers = vec!["42", "93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    print(double_first(numbers));
    print(double_first(empty));
    print(double_first(strings));
}

/// A way to write simple code while preserving the original errors is to `Box`
/// them. The drawback is that the underlying error type is only known at
/// runtime and _not statically determined_.
///
/// The _stdlib_ helps in boxing our errors by having `Box` implement conversion
/// from any type that implements the `Error` trait into the trait object
/// `Box<Error>`, via `From`.
fn ex3_boxing_errors() {
    type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

    #[derive(Debug, Clone)]
    struct EmptyVec;

    impl fmt::Display for EmptyVec {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "invalid first item to double")
        }
    }

    impl error::Error for EmptyVec {}

    fn double_first(vec: Vec<&str>) -> Result<i32> {
        vec.first()
            .ok_or_else(|| EmptyVec.into()) // Converts to Box
            .and_then(|s| {
                s.parse::<i32>()
                    .map_err(|e| e.into()) // Converts to Box
                    .map(|i| 2 * i)
            })
    }

    /// The above function can be simplified with `?`.
    fn double_first_v2(vec: Vec<&str>) -> Result<i32> {
        let first = vec.first().ok_or(EmptyVec)?;
        let parsed = first.parse::<i32>()?;
        Ok(2 * parsed)
    }

    fn print(result: Result<i32>) {
        match result {
            Ok(n) => println!("The first doubled is {}", n),
            Err(e) => println!("Error: {}", e),
        }
    }
    let numbers = vec!["42", "93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    print(double_first(numbers));
    print(double_first(empty));
    print(double_first(strings));

    let numbers = vec!["42", "93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    print(double_first_v2(numbers));
    print(double_first_v2(empty));
    print(double_first_v2(strings));
}

use std::error::Error;

/// An alternative to boxing errors is to wrap them in your own error type.
///
/// This adds a bit more boilerplate for handling errors and might not be needed
/// in all applications. There are some libraries that can take care of the
/// boilerplate for you.
fn ex4_wrapping_errors() {
    type Result<T> = std::result::Result<T, DoubleError>;

    #[derive(Debug)]
    enum DoubleError {
        EmptyVec,
        // We will defer to the parse error implementation for their error.
        // Supplying extra info requires adding more data to the type.
        Parse(ParseIntError),
    }

    impl fmt::Display for DoubleError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match *self {
                DoubleError::EmptyVec => write!(f, "please use a vector with at least one element"),
                // The wrapped error contains additional information and is available
                // via the `source()` method.
                DoubleError::Parse(..) => {
                    write!(f, "the provided string could not be parsed as int")
                }
            }
        }
    }

    impl error::Error for DoubleError {
        fn source(&self) -> Option<&(dyn error::Error + 'static)> {
            match *self {
                DoubleError::EmptyVec => None,
                // The cause is the underlying implementation error type. Is implicitly
                // cast to the trait object `&error::Error`. This works because the
                // underlying type already implements the `Error` trait.
                DoubleError::Parse(ref e) => Some(e),
            }
        }
    }

    // Implement the conversion from `ParseIntError` to `DoubleError`.
    // This will be automatically called by `?` if a `ParseIntError`
    // needs to be converted into a `DoubleError`.
    impl From<ParseIntError> for DoubleError {
        fn from(err: ParseIntError) -> DoubleError {
            DoubleError::Parse(err)
        }
    }

    // Very similar to `double_first_v2` in `ex3_boxing_errros`, but with
    // implicit conversions of the wrapper errors.
    fn double_first(vec: Vec<&str>) -> Result<i32> {
        let first = vec.first().ok_or(DoubleError::EmptyVec)?;
        // Implicitly use the `ParseIntError` implementation of `From` (above)
        // in order to create a `DoubleError`.
        let parsed = first.parse::<i32>()?;

        Ok(2 * parsed)
    }

    fn print(result: Result<i32>) {
        match result {
            Ok(n) => println!("The first doubled is {}", n),
            Err(e) => {
                println!("Error: {}", e);
                if let Some(source) = e.source() {
                    println!("  Caused by: {}", source);
                }
            }
        }
    }

    let numbers = vec!["42", "93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    print(double_first(numbers));
    print(double_first(empty));
    print(double_first(strings));
}

fn main() {
    println!("-=- ex1_pulling_result_out_of_options() -=-");
    ex1_pulling_result_out_of_options();

    println!("-=- ex2_defining_errtype() -=-");
    ex2_defining_errtype();

    println!("-=- ex3_boxing_errors() -=-");
    ex3_boxing_errors();

    println!("-=- ex4_wrapping_errors() -=-");
    ex4_wrapping_errors();
}
