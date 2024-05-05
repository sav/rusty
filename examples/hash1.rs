// hash1.rs, Examples from the book "Rust by Example"
// Copyright (C) 2024, Savio Sena <savio.sena@gmail.com>

#![allow(dead_code)]

use std::collections::{HashMap, HashSet};

/// Keys can be any type that implements [`Eq`] and [`Hash`]. This includes:
///
///  - `bool` (though not very useful)
///  - `int`, `uint`, and all variations thereof
///  - `String` and `&str` (protip: you can have a `HashMap` keyed by `String`
///     and call `.get()` with `&str`.)
///
/// Note that `f32` and `f64` do not implement `Hash`, likely because
/// _floating-point precision errors_ would make using them as hashmap keys
/// horribly error-prone.
///
/// You can easily implement `Eq` and Hash for a custom type with just one line:
/// `#[derive(PartialEq, Eq, Hash)]`
fn ex1_hashmap() {
    let mut contacts = HashMap::new(); // or `HashMap::with_capacity(uint)`.

    contacts.insert("Sav", "987-123");
    contacts.insert("Dm", "313-370");
    contacts.insert("Tmh", "133-700");

    // Takes a reference and returns `Option<&V>`.
    match contacts.get(&"Sav") {
        Some(&number) => println!("calling Sav: {number}"),
        _ => println!("Don't have Sav's number."),
    }

    contacts.remove(&"Sav");

    for (contact, &number) in contacts.iter() {
        println!("calling {contact}: {number}");
    }
}

/// Think of a [`HashSet`] as a [`HashMap`] where we just care about the keys
/// and that guarantees to have no duplicate elements. [`HashSet`] is just one
/// implementation. (see also: [`BTreeSet`])
///
/// `HashSet<T>` is, in actuality, just a wrapper around `HashMap<T, ()>`.
///
/// Sets have 4 primary operations, each returning iterators:
///  - `union`: get all the unique elements in both sets.
///  - `difference`: get all the elements that are in the first set but not the second.
///  - `intersection`: get all the elements that are only in _both_ sets.
///  - `symmetric_difference`: get all the elements that are in one set or the
///     other, but _not_ both.
fn ex2_hashset() {
    let mut a: HashSet<i32> = vec![1i32, 2, 3].into_iter().collect();
    let mut b: HashSet<i32> = vec![2i32, 3, 4].into_iter().collect();

    assert!(a.insert(4));
    assert!(a.contains(&4));

    // `HashSet::insert()` returns false if there was a value already present.
    assert_eq!(b.insert(4), false);

    b.insert(5);

    println!("A: {:?}", a);
    println!("B: {:?}", b);

    println!("Union: {:?}", a.union(&b).collect::<Vec<&i32>>());
    println!("Difference: {:?}", a.difference(&b).collect::<Vec<&i32>>());
    println!(
        "Intersection: {:?}",
        a.intersection(&b).collect::<Vec<&i32>>()
    );
    println!(
        "Symmetric Difference: {:?}",
        a.symmetric_difference(&b).collect::<Vec<&i32>>()
    );
}

fn main() {
    ex1_hashmap();
    ex2_hashset();
}
