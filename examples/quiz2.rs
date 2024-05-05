// quiz2.rs, Examples from the book "Rust by Example"
// Copyright (C) 2024, Savio Sena <savio.sena@gmail.com>

//! # [Rust Quiz #2](https://dtolnay.github.io/rust-quiz/2)
//!
//! The closures `f`, `g`, and `h` are all of type `impl Fn()`.
//!
//! The closure `i` is different. Formatting the code with rustfmt makes it
//! clearer how `i` is parsed.
//! ```rust,no_run
//! let i = || {
//!     {}
//!     &S(4)
//! };
//! ```
//! The type of `i` is `impl Fn() -> &'static S`.
//! 
//! *Answer*: `123`

struct S(i32);

impl std::ops::BitAnd<S> for () {
    type Output = ();

    fn bitand(self, rhs: S) {
        print!("{}", rhs.0);
    }
}

#[allow(unused_variables)]

fn main() {
    let f = || ( () & S(1) );
    let g = || { () & S(2) };
    let h = || ( {} & S(3) );
    let i = || { {} & S(4) };
    f();
    g();
    h();
    i();
}
