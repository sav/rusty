// fibo2.rs, Examples from the book "Rust by Example"
// Copyright (C) 2024, Savio Sena <savio.sena@gmail.com>

#[no_mangle]
fn fibo(m: i128) -> i128 {
    let mut n1: i128 = 1;
    let mut n2: i128 = 1;

    for _ in 1..m {
        let tmp = n1 + n2;
        n1 = n2;
        n2 = tmp;
    }
    n2
}

fn main() {
    println!("{}", fibo(44));
}
