// phantom.rs, Examples from the book "Rust by Example"
// Copyright (C) 2024, Savio Sena <savio.sena@gmail.com>

//! # Phantom Type Parameter
//!
//! A _phantom type parameter_ is one that doesn't show up at runtime, but is
//! checked statically (and only) at compile time.
//!
//! Data types can use extra generic type parameters to act as markers or to
//! perform type checking at compile time. These extra parameters hold no
//! storage values, and have no runtime behavior.
//!
//! ## Use Case
//!
//! Phantom type parameters serves to somewhat _"bind"_ type specifications
//! _"together"_.
//!
//! The following construction would impose: `Self + RHS = Output`
//! where `RHS` defaults to `Self` if not specified in the implementation.
//! ```no_run
//! pub trait Add<RHS = Self> {
//!     type Output;
//!
//!     fn add(self, rhs: RHS) -> Self::Output;
//! }
//!
//! // `Output` must be `T<U>` so that `T<U> + T<U> = T<U>`.
//! impl<U> Add for T<U> {
//!     type Output = T<U>;
//!     ...
//! }
//! ```

use std::marker::PhantomData;
use std::ops::Add;

// Lib code.

#[derive(Debug, Clone, Copy)]
struct Length<Unit>(f64, PhantomData<Unit>);

impl<Unit> Add for Length<Unit> {
    type Output = Length<Unit>;

    fn add(self, rhs: Length<Unit>) -> Length<Unit> {
        Length(self.0 + rhs.0, PhantomData)
    }
}

// User code.

#[derive(Debug, Clone, Copy)]
enum Inch {}

#[derive(Debug, Clone, Copy)]
enum Mm {}

fn main() {
    let one_foot: Length<Inch> = Length(12.0, PhantomData);
    let one_meter: Length<Mm> = Length(1000.0, PhantomData);

    let two_feet = one_foot + one_foot;
    let two_meters = one_meter + one_meter;
    // let one_feter = one_foot + one_meter; // Error: type mismatch.

    println!("one foot + one_foot = {:?} in", two_feet.0);
    println!("one meter + one_meter = {:?} mm", two_meters.0);
}
