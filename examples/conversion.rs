// conversion.rs, Examples from the book "Rust by Example"
// Copyright (C) 2024, Savio Sena <savio.sena@gmail.com>

//! # Conversion
//!
//! Primitive types can be converted to each other through casting (see
//! `examples/casting.rs`).
//!
//! Rust addresses conversion between custom types (i.e., `struct` and `enum`) by
//! the use of _traits_. The generic conversions will use the `From` and `Into`
//! traits. However there are more specific ones for the more common cases, in
//! particular when converting to and from `Strings`.

#![allow(unused_variables)]
#![allow(dead_code)]

use std::convert::{From, Into, TryFrom, TryInto};
use std::fmt;

#[derive(Debug)]
pub struct Number1 {
    value: i32,
}

impl From<i32> for Number1 {
    fn from(item: i32) -> Self {
        Number1 { value: item }
    }
}

#[derive(Debug)]
pub struct Number2 {
    value: i32,
}

impl Into<Number2> for i32 {
    fn into(self) -> Number2 {
        Number2 { value: self }
    }
}

#[derive(Debug, PartialEq)]
pub struct EvenNumber(i32);

impl TryFrom<i32> for EvenNumber {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value % 2 == 0 {
            Ok(EvenNumber(value))
        } else {
            Err(())
        }
    }
}

/// ## `From` and `Into`
///
/// The `From` and `Into` traits are inherently linked, and this is actually
/// part of its implementation. If you are able to convert type `A` from type
/// `B`, then it should be easy to believe that we should be able to convert
/// type `B` to type `A`.
///
/// ## `From`
///
/// The `From` trait allows for a type to define how to create itself from another
/// type, hence providing a very simple mechanism for converting between several
/// types.
///
/// ## `Into`
///
/// The `Into` trait is simply the reciprocal of the `From` trait. That is, if
/// you have implemented the `From` trait for your type, `Into` will call it
/// when necessary. You don't have to (and can't) implement both.
///
/// Using the `Into` trait will typically require specification of the type to
/// convert into as the compiler is unable to determine this most of the time.
/// However this is a small trade-off considering we get the functionality for
/// free.

pub fn from_and_into() {
    // From
    let my_str = "hello";
    let my_string = String::from(my_str);

    let num = Number1::from(30);
    println!("My number is {:?}", num);

    // Into
    let int = 5;
    let num: Number2 = int.into();
    println!("My number is {:?}", num);

    fn get_number(num: Number2) {
        println!("My number is: {:?}", num);
    }
    get_number(5.into());
}

/// ## `TryFrom` and `TryInto`
///
/// Similar to `From` and `Into`, `TryFrom` and `TryInto` are generic traits for
/// converting between types. Unlike `From`/`Into`, the `TryFrom`/`TryInto`
/// traits are used for fallible conversions, and as such, return `Results`.

pub fn tryfrom_and_tryinto() {
    // TryFrom
    assert_eq!(EvenNumber::try_from(8), Ok(EvenNumber(8)));
    assert_eq!(EvenNumber::try_from(5), Err(()));

    // TryInto
    let result: Result<EvenNumber, ()> = 8i32.try_into();
    assert_eq!(result, Ok(EvenNumber(8)));
    let result: Result<EvenNumber, ()> = 5i32.try_into();
    assert_eq!(result, Err(()));
}

#[derive(Debug, PartialEq)]
pub struct Circle {
    radius: i32,
}

impl fmt::Display for Circle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Circle of radius {}", self.radius)
    }
}

use std::str::FromStr;

/// The associated error type (`Err`) for the `FromStr` trait implementation of
/// `Circle`.

#[derive(Debug, PartialEq, Eq)]
pub struct ParseCircleError;

impl FromStr for Circle {
    type Err = ParseCircleError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Trim parentheses, if any, and parse the integer inside
        let cleaned_str = s.trim_matches(|c| c == '(' || c == ')');
        let radius = cleaned_str.parse::<i32>().map_err(|_| ParseCircleError)?;

        Ok(Circle { radius })
    }
}

/// ## To and from Strings
///
/// ### Converting to String
///
/// To convert any type to a `String` is as simple as implementing the
/// `ToString` trait for the type. Rather than doing so directly, you should
/// implement the `fmt::Display` trait which _automagically_ provides `ToString`
/// and also allows printing the type as discussed in the section on `print!`.
///
/// ### Parsing a String
///
/// To convert strings into types we use the `parse` function and arrange for
/// type inference or specify the type to parse using the "_turbofish_" syntax.
/// Both alternatives are shown in the underlying function below.
///
/// This will convert the string into the type specified as long as the `FromStr`
/// trait is implemented for that type. This is implemented for numerous types
/// within the standard library. To obtain this functionality on a user defined
/// type simply implement the `FromStr` trait for that type.

pub fn to_and_from_strings() {
    // To Strings
    let circle = Circle { radius: 6 };
    println!("{}", circle.to_string());

    // Parsing a String
    let parsed: i32 = "5".parse().unwrap();
    let turbo_parsed = "10".parse::<i32>().unwrap();

    let sum = parsed + turbo_parsed;
    println!("Sum: {:?}", sum);

    let input = "(1)";
    let circle: Circle = input.parse().unwrap();
    println!("Parsed circle: {:?}", circle);
    println!("Parsed circle: {:?}", Circle::from_str("(2)").unwrap());
    println!("Parsed circle: {:?}", "(3)".parse::<Circle>().unwrap());

    assert!(Circle::from_str("<2>").is_err());
}

pub fn main() {
    println!("-=- from_and_into() -=-");
    from_and_into();

    println!("-=- tryfrom_and_tryinto() -=-");
    tryfrom_and_tryinto();

    println!("-=- to_and_from_strings() -=-");
    to_and_from_strings();
}
