// iter2.rs - Copyright (C) 2024, Savio Sena <savio.sena@gmail.com>

//! The idea of this example is to demonstrate that, unlike other languages
//! where using `.len()` directly in a loop causes a function call at each
//! iteration, in Rust, once the iterator is constructed and its range is
//! determined, it remains constant during the loop, and therefore the
//! `.len()` function is called only once.

fn case01(v: &Vec<i32>) -> i32 {
    let mut r = 0;
    let n = v.len();
    for i in 0..n {
        r += i;
    }
    r as i32
}
    
fn case02(v: &Vec<i32>) -> i32 {
    let mut r = 0;
    for i in 0..v.len() {
        r += i;
    }
    r as i32
}

fn case03(v: &mut Vec<i32>) -> i32 {
    let mut r = 0;
    println!("case03: len={}", v.len());
    for i in 0..v.len() {
        r += i;
        v.push(1);
        println!("case03: i={i}, len={}", v.len());
    }
    r as i32
}

fn main() {
    let v1 = vec![1, 2, 3, 4, 5, 6];
    let v2 = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    println!("{}", case01(&v1));
    println!("{}", case01(&v2));

    let v1 = vec![1, 2, 3, 4, 5, 6];
    let v2 = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    println!("{}", case02(&v1));
    println!("{}", case02(&v2));

    let mut v1 = vec![1, 2, 3, 4, 5, 6];
    let mut v2 = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    println!("{}", case03(&mut v1));
    println!("{}", case03(&mut v2));        
}
