// quiz30.rs, Examples from the book "Rust by Example"
// Copyright (C) 2024, Savio Sena <savio.sena@gmail.com>

//! # [Rust Quiz #30](https://dtolnay.github.io/rust-quiz/30)
//!
//! Both of our non-reference types, `()` and `A`, are zero-sized types (ZST).
//!
//! On `p(a.clone())`, if `A` implemented `Clone` then `a.clone()` would be a
//! call to that impl, but since it _doesn't_, the compiler finds another
//! applicable impl which is the implementation of `Clone` for references `&T`
//! -- so concretely, the clone call is calling the impl of `Clone` for `&A`,
//! which turns a `&&A` into a `&A` by simply duplicating the reference.
//!
//! The type `()` _does_ implement `Clone` so `b.clone()` invokes that impl and
//! produces `()`.
//!
//! The implementation of `Clone` for `&()` would also be applicable as happened
//! in the case of `A`, but the compiler prefers calling the trait impl for `()`
//! which converts `&()` to `()` over the trait impl for `&()` which converts
//! `&&()` to `&()` because the former is the one that requires fewer implicit
//! references or dereferences inserted by the trait solver. In the call to
//! `b.clone()`, `b` is of type `&()` which exactly matches the argument of the
//! impl `Clone` for `()`, while in order to obtain a `&&()` to pass as argument
//! to the impl `Clone` for `&()` the trait solver would need to insert an
//! additional layer of referencing implicitly -- effectively computing
//! `(&b).clone()`.
//!
//! What we get is `p(b)` calling `p` with `X = &()` and `p(b.clone())` calling
//! `p` with `X = ()`. Together these print `10`.
//!
//! Finally in the `Rc` case, both calls to `p` are with `X = Rc<()>` which is
//! _non-zero sized_. It is considered idiomatic to clone a `Rc` using
//! `Rc::clone(&c)` instead of `c.clone()` because it makes it _apparent_ that
//! this is a reference count bump rather than cloning underlying data, but
//! ultimately _both refer to the same function_. To call the clone method of a
//! value inside a `Rc`, you would need to dereference it first: `(*c).clone()`.
//!
//! *Asnwer*: `111011`

#![allow(unused_variables,noop_method_call)]

use std::rc::Rc;

struct A;

fn p<X>(x: X) {
    match std::mem::size_of::<X>() {
        0 => print!("0"),
        _ => print!("1"),
    }
}

fn main() {
    let a = &A;
    p(a);
    p(a.clone());
    
    let b = &();
    p(b);
    p(b.clone());
    
    let c = Rc::new(());
    p(Rc::clone(&c));
    p(c.clone());
}
