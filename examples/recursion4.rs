// recursion4.rs,
// Copyright (C) 2024, Savio Sena <savio.sena@gmail.com>

//! # Tail Call Elimination
//!
//! From [Tricks of the trade: Recursion to Iteration, Part 2: Eliminating
//! Recursion with the Time-Traveling Secret Feature
//! Trick](https://blog.moertel.com/posts/2013-05-14-recursive-to-iterative-2.html).
//!
//! ## Computing binomial coefficients
//!
//! Math textbooks define the binomial coefficient `(n k)` ("_n choose k_")
//! coefficient like this:
//! ```text
//! / n \       n!
//! |   | = ----------
//! \ k /   k!(n - k)!
//! ```
//! This form causes all sorts of problems for computers. Fortunately, _Concrete
//! Mathematics_ (Graham, Knuth, Patashnik) helpfully points out a lifesaving
//! absorption identity:
//! ```text
//! / n \   n  / n - 1 \
//! |   | = -- |-------|
//! \ k /   k  \ k - 1 /
//! ```
//! That's a recursive function just waiting to happen.
//!
//! This identity along with the base case `(n 0) = 1` gives the following
//! example.
//!
//! ## Eliminating Recursion with the Time-Traveling Trick
//!
//!  1. Find a recursive call that's not a tail call.
//!  2. Identify what work is being done between that call and its `return`
//!     statement.
//!  3. Extend the function with a secret feature to do that work, as controlled
//!     by a new accumulator argument with a default value that causes it to do
//!     nothing.
//!  4. Use the secret feature to eliminate the old work.
//!  5. You've now got a tail call!
//!  6. Repeat until all recursive calls are tail calls.
//!
//! ### Identify the "extra work"
//! ```text
//! x = binomial_v1(n - 1, k - 1)
//! return n * x / k
//! ```
//! We can visualize the additional work as a separate function operating on
//! `x`:
//! ```
//! fn work(x, lmul, rdiv) -> i64 {
//!     lmul * x / rdiv
//! }
//! ```
//! For functions this simple you can just hold them in your hea and inline them
//! into your code as needed.
//!
//! ### Extend the function with a secret feature to do that work
//! ```
//! fn work(x: i64, lmul: i64, rdiv: i64) -> i64 {
//!     lmul * x / rdiv
//! }
//! fn binomial(n: i64, k: i64, lmul: i64, rdiv: i64) -> i64 {
//!    if k == 0 {
//!        return work(1, lmul, rdiv)
//!    }
//!    work(n * binomial(n - 1, k - 1) / k, lmul, rdiv)
//! }
//! ```
//! Note that all `return {whatever}` were mechanically converted to `return
//! work({whatever}, lmul, rdiv)`, since `x = binomial(n - 1, k - 1)`.
//!
//! ### Use the secret feature to eliminate the old work.
//!
//! Simply re-arrange the factors and pass them in the secret feature.
//! ```
//! fn binomial(n: i64, k: i64, lmul: i64, rdiv: i64) -> i64 {
//!     if k == 0 {
//!         return work(1, lmul, rdiv);
//!     }
//!     binomial(n - 1, k - 1, lmul * n, k * rdiv)
//! }
//! ```
//!
//! ### You've now got a tail call!
//!
//! All that's left is to inline the sole remaining `work` call.
//! ```
//! fn binomial(n: i64, k: i64, lmul: i64, rdiv: i64) -> i64 {
//!     if k == 0 {
//!         return lmul * 1 / rdiv;
//!     }
//!     binomial(n - 1, k - 1, lmul * n, k * rdiv)
//! }
//! ```
//!
//! ### Apply the "Simple Method" to remove the tail call
//!
//! ```
//! fn binomial(n: i64, k: i64, lmul: i64, rdiv: i64) -> i64 {
//!     loop {
//!         if k == 0 {
//!             return lmul * 1 / rdiv;
//!         }
//!         (n, k, lmul, rdiv) = (n - 1, k - 1, lmul * n, k * rdiv);
//!         continue;
//!     }
//! }
//! ```
//! And tidy up:
//! ```
//! fn binomial(n: i64, k: i64) -> i64 {
//!     let (mut lmul, mut rdiv) = (1, 1);
//!     while k > 0 {
//!         (n, k, lmul, rdiv) = (n - 1, k - 1, lmul * n, k * rdiv);
//!     }
//!     lmul / rdiv
//! }
//! ```
//! That's it! We now have an iterative version of the original recursive
//! function.
//!
//! It’s important to note that in the iterative version, `imul` increases at a
//! significantly faster rate compared to any other temporary value in the
//! recursive version. This rapid growth is indeed a drawback of the iterative
//! approach as `imul` can overflow much sooner, thereby considerably restricting
//! the range that the function can handle.

#![allow(unused_variables)]

fn binomial_recursive(n: i128, k: i128) -> i128 {
    if k == 0 {
        return 1;
    }
    return n * binomial_recursive(n - 1, k - 1) / k;
}

fn binomial(mut n: i128, mut k: i128) -> i128 {
    let (mut lmul, mut rdiv) = (1, 1);
    while k > 0 {
        (n, k, lmul, rdiv) = (n - 1, k - 1, lmul * n, k * rdiv);
    }
    lmul / rdiv
}

fn main() {
    let x = binomial_recursive(3000, 10);
    let y = binomial(3000, 10);

    println!("{}\n{}", x, y);
    assert_eq!(x, y);
}
