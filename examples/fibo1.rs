// fibo1.rs, Examples from the book "Rust by Example"
// Copyright (C) 2024, Savio Sena <savio.sena@gmail.com>

#[no_mangle]
fn fibo(n: i128) -> i128 {
    if n <= 2 {
        n
    } else {
        fibo(n - 1) + fibo(n - 2)
    }
}

fn main() {
    println!("{}", fibo(44));
}
