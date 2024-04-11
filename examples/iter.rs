// iter.rs, Examples from the book "Rust by Example"
// Copyright (C) 2024, Savio Sena <savio.sena@gmail.com>

/// `std::iter::Iterator::any` is a function which when passed an iterator, will
/// return `true` if any element satisfies the predicate. Otherwise `false`.

pub fn iter_any() {
    let vec1 = vec![1, 2, 3];

    println!("{:?}", vec1.iter().any(|&x| x == 2));
    println!("{:?}", vec1.into_iter().any(|x| x == 2));
}

/// `std::iter::Iterator::find` is a function which iterates over an iterator
/// and searches for the first value which satisfies some condition. If none of
/// the values satisfy the condition, it returns `None`.
///
/// `Iterator::find` gives you a reference to the item. But if you want the
/// _index_ of the item, use `Iterator::position`.

pub fn iter_find() {
    // `Iterator::find`.
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];

    let mut iter = vec1.iter();
    let mut into_iter = vec2.into_iter();

    println!("Find 2 in vec1: {:?}", iter.find(|&&x| x == 2));
    println!("Find 2 in vec2: {:?}", into_iter.find(|&x| x == 2));

    let array1 = [1, 2, 3];
    let array2 = [4, 5, 6];

    println!("Find 2 in array1: {:?}", array1.iter().find(|&&x| x == 2));
    println!(
        "Find 2 in array2: {:?}",
        array2.into_iter().find(|&x| x == 2)
    );

    // `Iterator::position`.
    let vec = vec![1, 9, 3, 3, 13, 2];

    let index_of_first_even_number = vec.iter().position(|&x| x % 2 == 0);
    assert_eq!(index_of_first_even_number, Some(5));

    let index_of_first_negative_number = vec.into_iter().position(|x| x < 0);
    assert_eq!(index_of_first_negative_number, None);
}

/// High Order Functions
///
/// Rust provides Higher Order Functions (HOF). These are functions that take
/// one or more functions and/or produce a more useful function. HOFs and lazy
/// iterators give Rust its functional flavor.
///
/// `Option` and `Iterator` implement their fair share of HOFs.

pub fn iter_hof() {
    fn is_odd(n: u32) -> bool { n % 2 == 1 }
    let upper = 1000;
    let sum_of_squared_odd_numbers: u32 = (0..)
        .map(|n| n * n) // All natural numbers squared
        .take_while(|&n_squared| n_squared < upper) // Below upper limit
        .filter(|&n_squared| is_odd(n_squared)) // That are odd
        .sum(); // Sum them
    println!("functional style: {}", sum_of_squared_odd_numbers);
}

pub fn main() {
    println!("-=- iter_any() -=-");
    iter_any();

    println!("-=- iter_find() -=-");
    iter_find();

    println!("-=- iter_hof() -=-");
    iter_hof();
}
