// turbofish.rs, Examples from the book "Rust by Example"
// Copyright (C) 2024, Savio Sena <savio.sena@gmail.com>

#[allow(unused_variables)]

/// # Turbofish Syntax
///
/// The “_turbofish_” syntax in Rust is a quirky and memorable way to explicitly
/// specify the type of a generic function or method. It looks like
/// `::<SomeType>` and is used to tell the compiler what type you expect when
/// working with generics.

pub fn main() {
    // let empty_vec = Vec::new(); // error: type annotations needed for `Vec<T>`.
    let empty_vec: Vec<u8> = Vec::new(); // Explicit type declaration.
    let empty_vec = Vec::<u8>::new(); // Turbofish syntax.
}
