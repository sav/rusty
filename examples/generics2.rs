// generics2.rs, Examples from the book "Rust by Example"
// Copyright (C) 2024, Savio Sena <savio.sena@gmail.com>

fn assert_call<F, Args>(f: F, args: Args)
where
    F: FnOnce(Args),
{
    f(args)
}

fn main() {
    assert_call(|(x, y, z)| println!("{} {} {}", x, y, z), ("foo", 0, 0.0));
}
