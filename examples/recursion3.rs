// recursion3.rs,
// Copyright (C) 2024, Savio Sena <savio.sena@gmail.com>

//! # Tail Call Elimination
//!
//! We'll see a few examples of how to translate a recursion algorithm to use
//! loops. The following study is based on:
//! 
//! - [Tricks of the trade: Recursion to Iteration, Part 1: The Simple Method, secret features, and accumulators](https://blog.moertel.com/posts/2013-05-11-recursive-to-iterative.html)
//! - [From Recursive to Iterative Functions](https://www.baeldung.com/cs/convert-recursion-to-iteration)
//!
//! ## The Simple Method
//!
//!  1. Convert all recursive calls into _tail calls_. (If you can't, stop. Try
//!     another method.)
//!  2. Introduce a one-shot loop around the function body.
//!  3. Convert tail calls into `continue` statements.
//!  4. Tidy the code and make it more idiomatic.

#![allow(unused_variables)]

pub const MAX: i64 = 2_000_000;

/// Original recursive function. Should the language implement
/// tail-call-elimination, we could stop here.
#[no_mangle]
pub fn ex_a_recursive(i: i64) -> i64 {
    if i >= MAX {
        i
    } else {
        // The code below is equivalent to:
        // ```
        // let tmp = ex_a_recursive(i + 1)
        // let ret = i + tmp
        // return ret
        // ```
        // Thus, it's not a tail-call recursion.
        i + ex_a_recursive(i + 1)
    }
}

/// Convert recursive calls to tail calls.
#[no_mangle]
pub fn ex_a_tailcall(i: i64, c: i64) -> i64 {
    if i >= MAX {
        return i + c;
    }
    ex_a_tailcall(i + 1, c + i)
}

/// Introduce a one-shot loop around the function body. Then replace all
/// recursive tail calls `f(x=x1, y=y1, ...)` with `(x, y, ...) = (x1, y1);`
/// then `continue`.
#[no_mangle]
pub fn ex_a_oneshot(mut i: i64, mut c: i64) -> i64 {
    loop {
        if i >= MAX {
            break i + c;
        }
        (i, c) = (i + 1, c + i);
        continue;
    }
}

/// Tidy the code and make it more idiomatic.
#[no_mangle]
pub fn ex_a_final(mut i: i64) -> i64 {
    let mut c = 0;
    while i < MAX {
        (i, c) = (i + 1, c + i);
    }
    c + i
}

pub fn main() {
    println!("{}", ex_a_recursive(0));
    println!("{}", ex_a_tailcall(0, 0));
    println!("{}", ex_a_oneshot(0, 0));
    println!("{}", ex_a_final(0));
}
