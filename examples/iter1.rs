// iter1.rs, Examples from the book "Rust by Example"
// Copyright (C) 2024, Savio Sena <savio.sena@gmail.com>

/// `std::iter::Iterator::any` is a function which when passed an iterator, will
/// return `true` if any element satisfies the predicate. Otherwise `false`.

fn iter_any() {
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

fn iter_find() {
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

fn iter_hof() {
    fn is_odd(n: u32) -> bool {
        n % 2 == 1
    }
    let upper = 1000;
    let sum_of_squared_odd_numbers: u32 = (0..)
        .map(|n| n * n) // All natural numbers squared
        .take_while(|&n_squared| n_squared < upper) // Below upper limit
        .filter(|&n_squared| is_odd(n_squared)) // That are odd
        .sum(); // Sum them
    println!("functional style: {}", sum_of_squared_odd_numbers);
}

fn iter_aux() {
    let mut sequence = 0..20;

    println!("{:?}", sequence.next());
    println!("{:?}", sequence.next());
    println!("{:?}", sequence.next());

    for i in sequence.skip(4).take(4) {
        println!("{}", i);
    }
}

fn iter_zip() {
    let a1 = [1, 2, 3];
    let a2 = [4, 5, 6];

    let mut iter = a1.iter().zip(a2.iter());

    assert_eq!(iter.next(), Some((&1, &4)));
    assert_eq!(iter.next(), Some((&2, &5)));
    assert_eq!(iter.next(), Some((&3, &6)));
    assert_eq!(iter.next(), None);
}

fn iter_into() {
    let a = 5;

    let v1 = vec![1, 2, 3];
    println!("{:?}", v1);

    let v2: Vec<_> = v1.iter().map(|x| x + a).collect();
    println!("{:?}", v2);

    let v3: Vec<_> = v1.into_iter().map(|x| x + a).collect();
    println!("{:?}", v3);

    // v1 unusable here.
}

fn iter_init() {
    let seq: Vec<_> = (10..8 * 2).collect();
    assert_eq!(seq, [10, 11, 12, 13, 14, 15]);
}

fn iter_api() {
    // --- flat_map ---
    let words = ["alpha", "beta", "gamma"];
    let merged: String = words.iter().flat_map(|s| s.chars()).collect();
    assert_eq!(merged, "alphabetagamma");

    let numbers = vec![1, 2, 3, 4, 5];
    let doubled_numbers: Vec<_> = numbers.iter().flat_map(|&x| vec![x, x]).collect();
    assert_eq!(doubled_numbers, [1, 1, 2, 2, 3, 3, 4, 4, 5, 5]);

    // --- for_each ---
    let mut c = 0;
    let mut last = 0;
    (0..5)
        .flat_map(|x| x * 10..x * 11)
        .enumerate()
        .filter(|&(i, x)| (i + x) % 3 == 0)
        .for_each(|(i, x)| {
            c += x;
            last = i;
        });
    assert_eq!(last, 7);
    assert_eq!(c, 91);

    // --- fold ---
    // Folds every element into an accumulator by applying an operation,
    // returning the final result.
    let a = [1, 2, 3];
    let sum = a.iter().fold(0, |acc, x| acc + x);
    assert_eq!(sum, 6);

    let numbers = [1, 2, 3, 4, 5];
    let result = numbers.iter().fold(0, |acc, &x| acc + x);
    assert_eq!(result, 15);

    // --- reduce ---
    // Reduces the elements to a single one, by repeatedly applying a
    // reducing operation.
    //
    // For iterators with at least one element, this is the same as `fold()`
    // with the first element of the iterator as the initial accumulator value
    let reduced: i32 = (1..10).reduce(|acc, e| acc + e).unwrap();
    assert_eq!(reduced, 45);
    // Which is equivalent to doing it with `fold`:
    let folded: i32 = (1..10).fold(0, |acc, e| acc + e);
    assert_eq!(reduced, folded);

    // --- scan ---
    // An iterator adapter which, like `fold`, holds internal state, but unlike
    // `fold`, produces a new iterator.
    let a = [1, 2, 3, 4];
    let mut iter = a.iter().scan(1, |state, &x| {
        *state = *state * x;
        if *state > 6 {
            return None; // terminate if exceeds 6
        }
        Some(-*state) // yield the negation of the state
    });
    assert_eq!(iter.next(), Some(-1));
    assert_eq!(iter.next(), Some(-2));
    assert_eq!(iter.next(), Some(-6));
    assert_eq!(iter.next(), None);

    // --- chain ---
    let a1 = [1, 2, 3];
    let a2 = [4, 5, 6];
    let mut iter = a1.iter().chain(a2.iter());
    assert_eq!(iter.next(), Some(&1));
    assert_eq!(iter.next(), Some(&2));
    assert_eq!(iter.next(), Some(&3));
    assert_eq!(iter.next(), Some(&4));
    assert_eq!(iter.next(), Some(&5));
    assert_eq!(iter.next(), Some(&6));
    assert_eq!(iter.next(), None);

    // --- cycle ---
    let a = [1, 2, 3];
    let mut it = a.iter().cycle();
    assert_eq!(it.next(), Some(&1));
    assert_eq!(it.next(), Some(&2));
    assert_eq!(it.next(), Some(&3));
    assert_eq!(it.next(), Some(&1));
    assert_eq!(it.next(), Some(&2));
    assert_eq!(it.next(), Some(&3));
    assert_eq!(it.next(), Some(&1));

    // --- filter_map ---
    // Creates an iterator that both filters and maps.
    let a = ["1", "two", "NaN", "four", "5"];
    let mut iter = a.iter().filter_map(|s| s.parse().ok());
    assert_eq!(iter.next(), Some(1));
    assert_eq!(iter.next(), Some(5));
    assert_eq!(iter.next(), None);

    // Equivalent to:
    let a = ["1", "two", "NaN", "four", "5"];
    let mut iter = a
        .iter()
        .map(|s| s.parse())
        .filter(|s| s.is_ok())
        .map(|s| s.unwrap());
    assert_eq!(iter.next(), Some(1));
    assert_eq!(iter.next(), Some(5));
    assert_eq!(iter.next(), None);

    // --- flatten ---
    // Works in any `IntoIterator`.
    let data = vec![vec![1, 2, 3, 4], vec![5, 6]];
    let flattened = data.into_iter().flatten().collect::<Vec<u8>>();
    assert_eq!(flattened, &[1, 2, 3, 4, 5, 6]);

    // Mapping and then flattening:
    let words = ["alpha", "beta", "gamma"];
    let merged: String = words
        .iter()
        .map(|s| s.chars()) /*chars returns an interator*/
        .flatten()
        .collect();
    assert_eq!(merged, "alphabetagamma");

    // Return in terms of `flat_map()`.
    let words = ["alpha", "beta", "gamma"];
    let merged: String = words
        .iter()
        .flat_map(|s| s.chars()) /*chars returns an iterator*/
        .collect();
    assert_eq!(merged, "alphabetagamma");

    // --- cloned ---
    let a = [1, 4, 2, 3];
    let sum = a
        .iter()
        .cloned()
        .filter(|x| x % 2 == 0)
        .fold(0, |sum, i| sum + i);
    assert_eq!(sum, 6);

    // --- zip ---
    let a1 = [1, 2, 3];
    let a2 = [4, 5, 6];
    let mut iter = a1.iter().zip(a2.iter());
    assert_eq!(iter.next(), Some((&1, &4)));
    assert_eq!(iter.next(), Some((&2, &5)));
    assert_eq!(iter.next(), Some((&3, &6)));
    assert_eq!(iter.next(), None);

    // --- cmp ---
    use std::cmp::Ordering;
    assert_eq!([1].iter().cmp([1].iter()), Ordering::Equal);
    assert_eq!([1].iter().cmp([1, 2].iter()), Ordering::Less);
    assert_eq!([1, 2].iter().cmp([1].iter()), Ordering::Greater);

    // --- product ---
    fn factorial(n: u32) -> u32 {
        (1..=n).product()
    }
    assert_eq!(factorial(0), 1);
    assert_eq!(factorial(1), 1);
    assert_eq!(factorial(5), 120);
}

fn main() {
    println!("-=- iter_any() -=-");
    iter_any();

    println!("-=- iter_find() -=-");
    iter_find();

    println!("-=- iter_hof() -=-");
    iter_hof();

    println!("-=- iter_aux() -=-");
    iter_aux();

    println!("-=- iter_into() -=-");
    iter_into();

    println!("-=- iter_zip() -=-");
    iter_zip();

    println!("-=- iter_init() -=-");
    iter_init();

    println!("-=- iter_api() -=-");
    iter_api();
}
