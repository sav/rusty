// recursion5.rs,
// Copyright (C) 2024, Savio Sena <savio.sena@gmail.com>

//! # The Time-Traveling Secret Feature trick
//!
//! A summary of the second part of this great article by [Tom
//! Moertel](https://x.com/tmoertel):
//!  - [Tricks of the trade: Recursion to Iteration, Part 2: Eliminating
//! Recursion with the Time-Traveling Secret Feature
//! Trick](https://blog.moertel.com/posts/2013-05-14-recursive-to-iterative-2.html)
//!
//! These are the steps to implement the "_time-traveling secret feature trick_":
//!
//! 1. Move the recursive call and surrounding work into a helper function.
//! 2. Partition the helper function into its 3 fundamental parts.
//!    - the recursive call to get the previous answer `x_{i-1}`,
//!    - the work to compute the current answer `x_{i}` from `x_{i-1}`, and
//!    - a `return` statement applied to `x_{i}` alone
//! 3. Prepare the helper function to receive previous steps as parameter.
//! 4. Modify the helper's return to also pass through its non-secret argument.
//! 5. Apply, to the non-secret arguments in the helper's return statement, the
//!    _opposite_ of the transformations applied to them in the recursive call.
//! 6. Determine the initial conditions at the start of the timeline.
//! 7. In the main function, iterate the `step` heler `t` times, starting from
//!    the _initial conditions_. Then return `x_{t}`.
//! 8. Remove the base cases from the original function.
//! 9. Remove the secret feature from the `step` function.
//! 10. Inline the `step` function.
//! 11. Apply finishing touches.
//!
//! Let's examine each of these steps in detail.
//!
//! ## The original function
//!
//! ```rust
//! fn binomial(n: i128, k: i128) -> i128 {
//!     if k == 0 {
//!         1
//!     } else {
//!         n * binomial(n - 1, k - 1) / k
//!     }
//! }
//! ```
//!
//! ## Move the recursive call into a helper
//!
//! ```rust
//! fn step(n: i128, k: i128) -> i128 {
//!     n * binomial(n - 1, k - 1) / k
//! }
//! fn binomial(n: i128, k: i128) -> i128 {
//!     if k == 0 {
//!         return 1;
//!     }
//!     step(n, k)
//! }
//! ```
//!
//! ## Partition the helper function
//!
//! ```rust
//! fn step(n: i128, k: i128) -> i128 {
//!     let previous_x = binomial(n - 1, k - 1);
//!     let x = n * previous_x / k;
//!     return x;
//! }
//! ```
//!
//! ## Prepare the helper function to receive a non-secret argument
//!
//! Make `step` receive previous steps through a "_non-secret argument_".
//! ```rust
//! fn step(n: i128, k: i128, mut previous_x: i128) -> i128 {
//!     if previous_x == i128::MIN {
//!         previous_x = binomial(n - 1, k - 1)
//!     }
//!     let x = n * previous_x / k;
//!     return x;
//! }
//! ```
//! Note that `x_{i-1}` goes _in_ and `x_{i}` comes _out_, such that:
//!  - `(n_{i}, k_{i}, x_{i-1}) -> x_{i}`
//!
//! This little "_kernel of power_" will literally allows us to _reverse the
//! flow of time_. Let's see how.
//!
//! ## Modify the helper's return to pass through its non-secret argument.
//!
//! ```rust
//! fn step(n: i128, k: i128, mut previous_x: i128) -> (i128, i128, i128) {
//!     if previous_x == i128::MIN {
//!         previous_x = binomial(n - 1, k - 1)
//!     }
//!     let x = n * previous_x / k;
//!     return (n, k, x); // <- here
//! }
//! fn binomial(n: i128, k: i128) -> i128 {
//!     if k == 0 {
//!         return 1;
//!     }
//!     step(n, k, i128::MIN).2 // <- note also
//! }
//! ```
//! Now our helper represents the transformation:
//!  - `(n_{i}, k_{i}, x_{i-1}) -> (n_{i}, k_{i}, x_{i})`
//!
//! ### Apply the opposite of the transformation to the non-secret argument
//!
//! ```rust
//! fn step(n: i128, k: i128, mut previous_x: i128) -> (i128, i128, i128) {
//!     if previous_x == i128::MIN {
//!         previous_x = binomial(n - 1, k - 1) // <- look here.
//!     }
//!     let x = n * previous_x / k;
//!     return (n + 1, k + 1, x); // <- apply the opposite here.
//! }
//! ```
//! Now our helper represents the transformation:
//!  - `(n_{i}, k_{i}, x_{i-1}) -> (n_{i+1}, k_{i+1}, x_{i})`
//!
//! And now we can _reverse the flow of time_.
//!
//! In the first example if we asked for `x_{t}` we would recurse into
//! `x_{t-1}`, and recurse into `x_{t-2}`, so on. Now we can step _the other
//! way_, if we have any `(n_{i}, k_{i}, x_{i-1})` we can get `x_{i}` straight
//! away, no recursion required. _In_ goes `x_{i-1}` out comes `x_{i}`. We can
//! _go foward_.
//!
//! ### Determine the initial conditions at the start of the timeline
//!
//! When recursion is called so many times it finally hits one of its base
//! cases, we say it reached time `i = 0`. In the case of the `binomial`
//! function, when `k = 0` is reached we have `i = 0`, `k = 0` and thus `x = 1`,
//! since its the value returned.
//!
//! We also know that in the end of the recursion, that is, when `i = t`, we'll
//! have `n_{t} = n` and `k_{t} = k`. For example, if we call `binomial(10, 8)`,
//! when `i = t` we'll have `n = 10` and `k = 8`.
//!
//! The answer we ultimately seek is `x_{t}`.
//!
//! The `step` function takes us from `(n_{i}, k_{i}, x_{i-1})` to `(n_{i+1},
//! k_{i+1}, x_{i})`. Let's look at its logic.
//!
//! For `n` and `k`:
//!    - `n_{i+1} = n_{i} + 1`
//!    - `k_{i+1} = k_{i} + 1`
//!
//! Since `k_{i}` is `i` steps from `k_{0} = 0` and each step adds `+1` we have:
//!    - `k_{i} = i`
//!
//! And when `i = t` we know from the equation that `t = k_{t} = k`.
//!
//! Finally we can compute `n_{1}`, which is `t-1` _reverse_ steps from `n_{t} =
//! n`, the only `n_{i}` that we know so far (`n_{t} = n`).
//!   - `n_{1} = n_{t} - (t - 1)(+1) = n - k + 1`
//!
//! So now we have our **initial conditions**:
//!   - `(n_{1), k_{1}, x_{0}) = (n - k + 1, 1, 0)`
//!
//! Now we can step forward through the timeline from `i = 1` to `i = 2`, and so
//! on, until finally, at the last step, when `i = t`, we will have determined
//! `x_{t}`.
//!
//! ### Iterate the `step` helper `t` times, starting from the initial conditions, then return `x_{t}`
//!
//! ```rust
//! fn binomial(n: i128, k: i128) -> i128 {
//!     if k == 0 {
//!         return 1;
//!     }
//!     let t = k;
//!     let (mut n, mut k, mut previous_x) = (n - k + 1, 1, 1);
//!     for _ in 1..(t+1) {
//!         (n, k, previous_x) = step(n, k, previous_x);
//!     }
//!     previous_x // = x_t
//! }
//! ```
//!
//! ### Remove the base cases from the original function
//!
//! Since our iterations start from a base case, the base cases are already
//! incorporated into our new, iterative logic.
//! ```rust
//! fn binomial(n: i128, k: i128) -> i128 {
//!     let t = k;
//!     let (mut n, mut k, mut previous_x) = (n - k + 1, 1, 1);
//!     for _ in 1..(t+1) {
//!         (n, k, previous_x) = step(n, k, previous_x);
//!     }
//!     previous_x // = x_t
//! }
//! ```
//!
//! ### Remove the secret feature from the `step` function.
//!
//! Since `previous_x` always gets a value now, we can make it a required part of
//! the function.
//! ```rust
//! fn step(n: i128, k: i128, previous_x: i128) -> (i128, i128, i128) {
//!     let x = n * previous_x / k;
//!     (n + 1, k + 1, x)
//! }
//! ```
//!
//! ### Inline the `step` function
//!
//! ```rust
//! fn binomial(n: i128, k: i128) -> i128 {
//!     let t = k;
//!     let (mut n, mut k, mut previous_x) = (n - k + 1, 1, 1);
//!     for _ in 1..(t + 1) {
//!         let x = n * previous_x / k;
//!         (n, k, previous_x) = (n + 1, k + 1, x)
//!     }
//!     previous_x
//! }
//! ```
//!
//! ### Apply finishing touches
//!
//! We can substitute away the `x` variable. And, because `k_{i} = i`, we can
//! replace the `for` loop's induction variable `_` with `k_{i}` and
//! correspondingly eliminate `k_{i}` from the step logic.
//!
//! Also, it is a propery of binomial coefficients that:
//! ```text
//! / n \   /  n  \
//! |   | = |     |
//! \ k /   \ n-k /
//! ```
//!
//! One property of our code is that it runs for `t = k` steps. So when `k > n -
//! k`, we can reduce the number of steps by solving for "`n` _choose_ `n-k`"
//! instead.
//! ```
//! fn binomial(n: i128, mut k: i128) -> i128 {
//!     if k > n - k {
//!         k = n - k;
//!     }
//!     let t = k;
//!     let (mut n, mut previous_x) = (n - k + 1, 1);
//!     for k in 1..(t + 1) {
//!         (n, previous_x) = (n + 1, n * previous_x / k)
//!     }
//!     previous_x // = x_{t}
//! }
//! ```

fn binomial_recursive(n: i128, k: i128) -> i128 {
    if k == 0 {
        1
    } else {
        n * binomial_recursive(n - 1, k - 1) / k
    }
}

fn binomial(n: i128, mut k: i128) -> i128 {
    if k > n - k {
        k = n - k;
    }
    let t = k;
    let (mut n, mut previous_x) = (n - k + 1, 1);
    for k in 1..(t + 1) {
        (n, previous_x) = (n + 1, n * previous_x / k)
    }
    previous_x // = x_{t}
}

fn main() {
    let x = binomial_recursive(3000, 10);
    println!("{}", x);

    let x = binomial(3000, 10);
    println!("{}", x);
}
