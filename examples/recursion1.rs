// recursion1.rs, Examples from the book "Rust by Example"
// Copyright (C) 2024, Savio Sena <savio.sena@gmail.com>

// In Rust, as well as in many other languages, tail-call-optimization is not guaranteed
// by the compiler, therefore recursive calls can eventually overflow the stack (by pushing
// too many return addresses and arguments).
//
// One way to avoid these problems is to use loops instead of function calls.

#![allow(dead_code, unused_variables, unused_mut)]

const MAX: i64 = 2_000;

/// sum all numbers from `i` up to `MAX`, recursively.
fn f1(i: i64) -> i64 {
    if i >= MAX {
        i
    } else {
        i + f1(i + 1)
    }
}

fn g1(ini: i64) -> i64 {
    let (mut i, mut r) = (ini, 0i64);
    'a: loop {
        'b: loop {
            if i >= MAX {
                break 'a r
            } else {
                continue 'b
            }
        }
    }
}

fn g1_v0(ini: i64) -> i64 {
    let (mut i, mut r) = (ini, 0i64);
    'a: loop {
        if i > MAX {
            break 'a r;
        }
        r += i;
        i += 1;
    }
}

fn g2(ini: i64) -> i64 {
    let (mut i, mut r) = (ini, 0i64);
    0i64
}

/// sum all even numbers from `i` up to `MAX`, recursively.
fn f2(i: i64) -> i64 {
    if i < MAX {
        if i % 2 != 0 {
            return i + f2(i + 1);
        } else {
            return f2(i + 1);
        }
    }
    if i % 2 != 0 {
        i
    } else {
        0
    }
}

fn main() {
    // TODO: fazer com closure
    println!("{}", f1(0));
    println!("{}", g1(0));
    println!("{}", f2(0));
}
