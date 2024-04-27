// trait1.rs, Examples from the book "Rust by Example"
// Copyright (C) 2024, Savio Sena <savio.sena@gmail.com>

//! # Traits

#![allow(dead_code)]

struct Point {
    x: i32,
    y: i32,
}

use std::f64;

trait Distance {
    fn distance(&self) -> f64;
}

impl Distance for Point {
    fn distance(&self) -> f64 {
        (self.x.pow(2) as f64 + self.y.pow(2) as f64).sqrt()
    }
}

impl Point {
    fn show_distance(&self) {
        println!(
            "distance from origin to ({}, {}): {}",
            self.x,
            self.y,
            self.distance() // this example demonstrates how implementors can
                            // access trait methods.
        );
    }
}

/// ## Trait method access.

fn ex1() {
    Point { x: 3, y: 4 }.show_distance();
}

struct City {
    name: &'static str,
    lat: f64,
    long: f64,
}

impl Distance for City {
    fn distance(&self) -> f64 {
        (self.lat.powi(2) + self.long.powi(2)).sqrt()
    }
}

fn new_point() -> impl Distance {
    Point { x: 3, y: 4 }
}

fn new_city() -> impl Distance {
    City {
        name: "Tokyo",
        lat: 35.682839,
        long: 139.759455,
    }
}

/// Unlike other languages, if you have a trait like `Distance`, you can't write
/// a function that returns `Distance`, because its different implementations
/// will need different amounts of memory, thus will have different sizes.
///
/// There's an easy workaround. Instead of returning a trait object directly, we
/// can return a `Box`, which has a well known size.

fn new_distance(flag: bool) -> Box<dyn Distance> {
    if flag {
        Box::new(new_city())
    } else {
        Box::new(new_point())
    }
}

/// ## Returning Traits with `dyn`.
fn ex2() {
    println!("distance to city: {:.2}", new_distance(true).distance());
    println!("distance to point: {:.2}", new_distance(false).distance());
}

/// ## `impl Trait` as a return type.
fn ex3() {
    use std::iter;
    use std::vec::IntoIter;

    // This function combines two `Vec<i32>` and returns an iterator over it.
    // Look how complicated its return type is!
    fn combine_vecs_explicit_return_type(
        v: Vec<i32>,
        u: Vec<i32>,
    ) -> iter::Cycle<iter::Chain<IntoIter<i32>, IntoIter<i32>>> {
        v.into_iter().chain(u.into_iter()).cycle()
    }

    // This is the exact same function, but its return type uses `impl Trait`.
    // Look how much simpler it is!
    fn combine_vecs(v: Vec<i32>, u: Vec<i32>) -> impl Iterator<Item = i32> {
        v.into_iter().chain(u.into_iter()).cycle()
    }

    let v1 = vec![1, 2, 3];
    let v2 = vec![4, 5];
    let mut v3 = combine_vecs(v1, v2);

    assert_eq!(Some(1), v3.next());
    assert_eq!(Some(2), v3.next());
    assert_eq!(Some(3), v3.next());
    assert_eq!(Some(4), v3.next());
    assert_eq!(Some(5), v3.next());

    println!("{:?}", v3.next());
    println!("{:?}", v3.next());

    // More importantly, some Rust types can't be written out. For example,
    // every closure has its own unnamed concrete type. Before `impl Trait`
    // syntax, you had to allocate on the heap in order to return a closure. But
    // now you can do it all statically, like this:

    fn make_closure(y: i32) -> impl Fn(i32) -> i32 {
        let closure = move |x: i32| x + y;
        closure
    }

    let plus_one = make_closure(1);
    assert_eq!(plus_one(2), 3);

    println!("{:?}", make_closure(1)(1));

    // You can also use `impl Trait` to return an iterator that uses map or
    // filter closures. This makes using `map` and `filter` easier.
    //
    // Because closure types don't have names, you can't write out an explicit
    // return type if your function returns iterators with closures. But with
    // `impl Trait` you can do this easily:

    fn double_positives<'a>(numbers: &'a Vec<i32>) -> impl Iterator<Item = i32> + 'a {
        numbers.iter().filter(|x| x > &&0).map(|x| x * 2)
    }

    let singles = vec![-3, -2, 2, 3];
    let doubles = double_positives(&singles);

    assert_eq!(doubles.collect::<Vec<i32>>(), vec![4, 6]);
}

/// ## Disambiguating overlapping traits.

fn ex4() {
    trait UsernameWidget {
        fn get(&self) -> String;
    }

    trait AgeWidget {
        fn get(&self) -> u8;
    }

    struct Form {
        username: String,
        age: u8,
    }

    impl UsernameWidget for Form {
        fn get(&self) -> String {
            self.username.clone()
        }
    }

    impl AgeWidget for Form {
        fn get(&self) -> u8 {
            self.age
        }
    }

    let form = Form {
        username: "rustacean".to_owned(),
        age: 28,
    };

    // If you uncomment this line, you'll get an error saying
    // "multiple `get` found". Because, after all, there are multiple methods
    // named `get`.
    // println!("{}", form.get());

    let username = <Form as UsernameWidget>::get(&form);
    assert_eq!("rustacean".to_owned(), username);

    let age = <Form as AgeWidget>::get(&form);
    assert_eq!(28, age);
}

fn main() {
    ex1(); // Trait method access.
    ex2(); // Returning Traits with `dyn`.
    ex3(); // `impl Trait` as a return type.
    ex4(); // Disambiguating overlapping traits.
}
