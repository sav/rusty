// recursion2.rs, Examples from the book "Rust by Example"
// Copyright (C) 2024, Savio Sena <savio.sena@gmail.com>

#![allow(dead_code)]

pub const MAX: i64 = 300_000_000;

#[no_mangle]
pub fn f0(i: i64) -> i64 {
    let mut n = 0_i64;
    for j in i as usize..=MAX as usize {
        n += j as i64;
    }
    n
}

#[no_mangle]
pub fn f1(i: i64) -> i64 {
    if i >= MAX {
        i
    } else {
        i + f1(i + 1)
    }
}

#[no_mangle]
pub fn f2(i: i64) -> i64 {
    if i < MAX {
        i + f2(i + 1)
    } else {
        i
    }
}

#[no_mangle]
pub fn f3(i: i64) -> i64 {
    if i >= MAX {
        i
    } else {
        let r = f3(i + 1);
        i + r
    }
}

pub fn main() {
    println!("0 => {}", f0(0));
    println!("1 => {}", f1(0));
    println!("2 => {}", f2(0));
    println!("3 => {}", f3(0));
}
