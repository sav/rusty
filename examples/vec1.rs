// vec1.rs, Examples from the book "Rust by Example"
// Copyright (C) 2024, Savio Sena <savio.sena@gmail.com>

fn ex_1() {
    // --- pop ---
    let mut v = vec![1, 2, 3, 4];
    let last = v.pop(); // for `pop_front` see `VecDeque`.
    assert_eq!(last, Some(4));
    assert_eq!(v, [1, 2, 3]);

    // --- push ---
    let mut v = vec![1, 2, 3, 4];
    v.push(5);
    assert_eq!(v, [1, 2, 3, 4, 5]);

    // --- clear ---
    let mut v = vec![1, 2, 3];
    v.clear();
    assert_eq!(v, []);

    // --- for ---
    let v = vec![1, 2, 3, 4, 5];
    for x in &v {
        println!("iter: {x}");
    }

    // --- while let ---
    let mut stack = vec![1, 2, 3, 4, 5];
    while let Some(top) = stack.pop() {
        println!("popped: {top}");
    }

    // --- is_empty ---
    let mut stack = vec![1, 2, 3, 4, 5];
    while !stack.is_empty() {
        stack.pop();
    }

    // --- append ---
    // Moves all the elements of other into self, leaving other empty.
    let mut v1 = vec![1, 2, 3];
    let mut v2 = vec![4, 5, 6];
    v1.append(&mut v2);
    assert_eq!(v1, [1, 2, 3, 4, 5, 6]);
    assert_eq!(v2, []);

    // --- extend ---
    let mut v = vec![1, 2, 3];
    v.extend([4]);
    assert_eq!(v, [1, 2, 3, 4]);

    // --- extend_from_slice ---
    let mut vec = vec![1];
    vec.extend_from_slice(&[2, 3, 4]);
    assert_eq!(vec, [1, 2, 3, 4]);

    // --- extend_from_within ---
    let mut vec = vec![0, 1, 2, 3, 4];

    vec.extend_from_within(2..);
    assert_eq!(vec, [0, 1, 2, 3, 4, 2, 3, 4]);

    vec.extend_from_within(..2);
    assert_eq!(vec, [0, 1, 2, 3, 4, 2, 3, 4, 0, 1]);

    vec.extend_from_within(4..8);
    assert_eq!(vec, [0, 1, 2, 3, 4, 2, 3, 4, 0, 1, 4, 2, 3, 4]);

    // --- split_off ---
    let mut vec = vec![1, 2, 3];
    let vec2 = vec.split_off(1);
    assert_eq!(vec, [1]);
    assert_eq!(vec2, [2, 3]);

    // --- reserve ---
    // Reserves a capacity for _at least_ `additional` more elements.
    // For a version that does not deliberately over-allocate to speculatively
    // avoid frequent allocation see `reserve_exact`.
    let mut vec = vec![1];
    vec.reserve(10);
    assert!(vec.capacity() >= 11);

    // --- resize ---
    let mut vec = vec![1, 2];
    vec.resize(4, 0);
    assert_eq!(vec, [1, 2, 0, 0]);

    let mut vec = vec!["hello"];
    vec.resize(3, "world");
    assert_eq!(vec, ["hello", "world", "world"]);

    let mut vec = vec![1, 2, 3, 4];
    vec.resize(2, 0);
    assert_eq!(vec, [1, 2]);

    // --- resize_with ---
    let mut vec = vec![1, 2, 3];
    vec.resize_with(5, Default::default);
    assert_eq!(vec, [1, 2, 3, 0, 0]);

    let mut vec = vec![];
    let mut p = 1;
    vec.resize_with(4, || {
        p *= 2;
        p
    });
    assert_eq!(vec, [2, 4, 8, 16]);

    // --- retain ---
    let mut vec = vec![1, 2, 3, 4];
    vec.retain(|&x| x % 2 == 0);
    assert_eq!(vec, [2, 4]);

    // --- shrink_to ---
    let mut vec = Vec::with_capacity(10);
    vec.extend([1, 2, 3]);
    assert!(vec.capacity() >= 10);
    vec.shrink_to(4);
    assert!(vec.capacity() >= 4);
    vec.shrink_to(0);
    assert!(vec.capacity() >= 3);

    // --- swap ---
    let mut v = ["a", "b", "c", "d", "e"];
    v.swap(2, 4);
    assert!(v == ["a", "b", "e", "d", "c"]);

    // --- reverse ---
    let mut v = [1, 2, 3];
    v.reverse();
    assert!(v == [3, 2, 1]);

    // --- windows ---
    let slice = ['l', 'o', 'r', 'e', 'm'];
    let mut iter = slice.windows(3);
    assert_eq!(iter.next().unwrap(), &['l', 'o', 'r']);
    assert_eq!(iter.next().unwrap(), &['o', 'r', 'e']);
    assert_eq!(iter.next().unwrap(), &['r', 'e', 'm']);
    assert!(iter.next().is_none());

    let slice = ['f', 'o', 'o'];
    let mut iter = slice.windows(4);
    assert!(iter.next().is_none());

    // --- chunks ---
    let slice = ['l', 'o', 'r', 'e', 'm'];
    let mut iter = slice.chunks(2);
    assert_eq!(iter.next().unwrap(), &['l', 'o']);
    assert_eq!(iter.next().unwrap(), &['r', 'e']);
    assert_eq!(iter.next().unwrap(), &['m']);
    assert!(iter.next().is_none());

    // --- rchunks ---
    let slice = ['l', 'o', 'r', 'e', 'm'];
    let mut iter = slice.rchunks(2);
    assert_eq!(iter.next().unwrap(), &['e', 'm']);
    assert_eq!(iter.next().unwrap(), &['o', 'r']);
    assert_eq!(iter.next().unwrap(), &['l']);
    assert!(iter.next().is_none());

    // --- dedup ---
    let mut vec = vec![1, 2, 2, 3, 2];
    vec.dedup();
    assert_eq!(vec, [1, 2, 3, 2]);

    // --- leak ---
    let x = vec![1, 2, 3];
    let static_ref: &'static mut [usize] = x.leak();
    static_ref[0] += 1;
    assert_eq!(static_ref, &[2, 2, 3]);

    // --- drain ---
    // Removes the specified range from the vector in bulk, returning all removed
    // elements as an iterator. If the iterator is dropped before being fully
    // consumed, it drops the remaining removed elements.
    //
    // Leaking: If the returned iterator goes out of scope without being dropped
    // (due to `mem::forget`, for example), the vector may have lost and leaked
    // elements arbitrarily, including elements outside the range.
    let mut v = vec![1, 2, 3];
    let u: Vec<_> = v.drain(1..).collect();
    assert_eq!(v, &[1]);
    assert_eq!(u, &[2, 3]);
    // A full range clears the vector, like `clear()` does
    v.drain(..);
    assert_eq!(v, &[]);
}

fn main() {
    println!("-=- ex_1 -=-");
    ex_1();
}
