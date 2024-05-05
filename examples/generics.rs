// generics.rs, Examples from the book "Rust by Example"
// Copyright (C) 2024, Savio Sena <savio.sena@gmail.com>

struct Cardinal;
struct BlueJay;
struct Turkey;

trait Red {}
trait Blue {}

impl Red for Cardinal {}
impl Blue for BlueJay {}

// These functions are only valid for types which implement these
// traits. The fact that the traits are empty is irrelevant.
fn red<T: Red>(_: &T) -> &'static str {
    "red"
}
fn blue<T: Blue>(_: &T) -> &'static str {
    "blue"
}

/// A consequence of how bounds work is that even if a trait doesn't include any
/// functionality, you can still use it as a bound. [`Eq`] and [`Copy`] are
/// examples of such traits from the std library.

fn empty_bounds() {
    let cardinal = Cardinal;
    let blue_jay = BlueJay;
    let _turkey = Turkey;

    // `red()` won't work on a blue jay nor vice versa
    // because of the bounds.
    println!("A cardinal is {}", red(&cardinal));
    println!("A blue jay is {}", blue(&blue_jay));
}

use std::fmt::Debug;

trait PrintInOption {
    fn print_in_option(self);
}

/// The `impl` in this example cannot be directly expressed without a `where`.
/// Because we would otherwise have to express this as `T: Debug` or
/// use another method of indirect approach, this requires a `where` clause:
impl<T> PrintInOption for T
where
    Option<T>: Debug,
{
    // We want `Option<T>: Debug` as our bound because that is what's
    // being printed. Doing otherwise would be using the wrong bound.
    fn print_in_option(self) {
        println!("{:?}", Some(self));
    }
}

/// A couple of cases that a `where` clause is useful:
///  - When specifying generic types and bounds separately is clearer.
///  - When using a `where` clause is more expressive than using normal syntax.
///    The `impl` in this example cannot be directly expressed without a `where`.
fn where_clauses() {
    let vec = vec![1, 2, 3];
    vec.print_in_option();
}

struct Container(i32, i32);

trait Contains {
    type A;
    type B;

    fn contains(&self, _: &Self::A, _: &Self::B) -> bool;
    fn first(&self) -> i32;
    fn last(&self) -> i32;
}

impl Contains for Container {
    type A = i32;
    type B = i32;

    fn contains(&self, number_1: &i32, number_2: &i32) -> bool {
        (&self.0 == number_1) && (&self.1 == number_2)
    }

    fn first(&self) -> i32 {
        self.0
    }
    fn last(&self) -> i32 {
        self.1
    }
}

/// Without associated types_ we would have to specify `A`, `B` and `C`, which
/// would be a nuisance. The function signature would be something like:
/// ```no_run
///fn difference<A, B, C>(container: &C) -> i32 where
///    C: Contains<A, B> {...}
/// ```

fn difference<C: Contains>(container: &C) -> i32 {
    container.last() - container.first()
}

fn associated_types() {
    let number_1 = 3;
    let number_2 = 10;

    let container = Container(number_1, number_2);

    println!(
        "Does container contain {} and {}: {}",
        &number_1,
        &number_2,
        container.contains(&number_1, &number_2)
    );
    println!("First number: {}", container.first());
    println!("Last number: {}", container.last());

    println!("The difference is: {}", difference(&container));
}

fn main() {
    empty_bounds();
    where_clauses();
    associated_types();
}
