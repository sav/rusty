// macro1.rs, Examples from the book "Rust by Example"
// Copyright (C) 2024, Savio Sena <savio.sena@gmail.com>

#![allow(dead_code)]

macro_rules! macro_hello {
    () => {
        println!("macro_hello!")
    };
}

/// The arguments of a macro are prefixed by a dollar sign `$` and type
/// annotated with a _designator_:

macro_rules! create_function {
    ($func_name:ident) => {
        fn $func_name() {
            println!("You called {:?}.", stringify!($func_name));
        }
    };
}

macro_rules! print_result {
    ($expression:expr) => {
        println!("{:?} = {:?}", stringify!($expression), $expression);
    };
}

/// These are some of the available designators:
///  - `block`
///  - `expr` is used for expressions
///  - `ident` is used for variable/function names
///  - `item`
///  - `literal` is used for literal constants
///  - `pat` (_pattern_)
///  - `path`
///  - `stmt` (_statement_)
///  - `tt` (_token tree_)
///  - `ty` (_type_)
///  - `vis` (_visibility qualifier_)
fn macro_designators() {
    create_function!(foo);
    foo();

    print_result!(1u32 + 1);
    print_result!({
        let x = 1u32;
        x * x + 2 * x - 1
    });
}

/// `test!` will compare `$left` and `$right` in different ways depending on how
/// you invoke it:
macro_rules! test {
    ($left:expr; and $right:expr) => {
        // and
        println!(
            "{:?} and {:?} is {:?}",
            stringify!($left),
            stringify!($right),
            $left && $right
        )
    };
    ($left:expr; or $right:expr) => {
        // or
        println!(
            "{:?} or {:?} is {:?}",
            stringify!($left),
            stringify!($right),
            $left || $right
        )
    };
}

fn macro_overload() {
    test!(1i32 + 1 == 2i32; and 2i32 * 2 == 4i32);
    test!(true; or false);
}

macro_rules! find_min {
    ($x:expr) => ($x); // base case
    ($x:expr, $($y:expr),+) => (
        std::cmp::min($x, find_min!($($y),+))
    )
}

fn macro_repeat() {
    println!("{}", find_min!(1));
    println!("{}", find_min!(1 + 2, 2));
    println!("{}", find_min!(5, 2 * 3, 4));
}

use std::ops::{Add, Mul, Sub};

macro_rules! assert_equal_len {
    // The `tt` (token tree) designator is used for operators and tokens.
    ($a:expr, $b:expr, $func:ident, $op:tt) => {
        assert!(
            $a.len() == $b.len(),
            "{:?} dimension mismatch: {:?} {:?} {:?}",
            stringify!($func),
            ($a.len(),),
            stringify!($op),
            ($b.len(),)
        );
    };
}

macro_rules! op {
    ($func:ident, $bound:ident, $op:tt, $method:ident) => {
        fn $func<T: $bound<T, Output = T> + Copy>(xs: &mut Vec<T>, ys: &Vec<T>) {
            assert_equal_len!(xs, ys, $func, $op);

            for (x, y) in xs.iter_mut().zip(ys.iter()) {
                *x = $bound::$method(*x, *y);
                // *x = x.$method(*y);
            }
        }
    };
}

use std::iter;

macro_rules! dry {
    ($func:ident, $x:expr, $y:expr, $z:expr) => {
        print!("{} {} {} ", stringify!($func), $x, $y);
        for size in 0usize..10 {
            let mut x: Vec<_> = iter::repeat($x).take(size).collect();
            let y: Vec<_> = iter::repeat($y).take(size).collect();
            let z: Vec<_> = iter::repeat($z).take(size).collect();

            $func(&mut x, &y);
            assert_eq!(x, z);

            print!(".");
        }
        println!("");
    };
}

fn macro_dry() {
    op!(add_assign, Add, +=, add);
    op!(mul_assign, Mul, *=, mul);
    op!(sub_assign, Sub, -=, sub);

    dry!(add_assign, 1u32, 2u32, 3u32);
    dry!(mul_assign, 2u32, 3u32, 6u32);
    dry!(sub_assign, 3u32, 2u32, 1u32);
}

macro_rules! calculate {
    (eval $e:expr) => {
        {
            let val: usize = $e;
            println!("{} = {}", stringify!{$e}, val);
        }
    };
}

fn macro_dsl() {
    calculate! {
        eval 1 + 2
    }
    calculate! {
        eval (1 + 2) * (3 / 4)
    }
}

macro_rules! vcalculate {
    (eval $e:expr) => { // pattern for a single `eval`.
        let val: usize = $e;
        println!("{} = {}", stringify!{$e}, val);
    };
    (eval $e:expr, $(eval $es:expr),+) => {{
        vcalculate! { eval $e }
        vcalculate! { $(eval $es),+ }
    }};
}

fn macro_variadic() {
    vcalculate! {
        eval 1 + 2,
        eval 3 + 4,
        eval (2 * 3) + 1
    }
}

fn main() {
    println!("-=- macro_hello!() -=-");
    macro_hello!();

    println!("-=- macro_designators() -=-");
    macro_designators();

    println!("-=- macro_overload() -=-");
    macro_overload();

    println!("-=- macro_repeat() -=-");
    macro_repeat();

    println!("-=- macro_dry() -=-");
    macro_dry();

    println!("-=- macro_dsl() -=-");
    macro_dsl();

    println!("-=- macro_variadic() -=-");
    macro_variadic();
}
