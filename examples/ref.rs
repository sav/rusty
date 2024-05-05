// ref.rs, Examples from the book "Rust by Example"
// Copyright (C) 2024, Savio Sena <savio.sena@gmail.com>

/// # The `ref` pattern
///
/// When doing pattern matching or destructuring via the let binding, the `ref`
/// keyword can be used to take references to the fields of a struct/tuple.
///
/// ## Partial Moves
///
/// Within the _destructuring_ of a single variable, both *by-move* and
/// *by-reference* pattern bindings can be used at the same time.
///
/// Doing this will result in a partial move of the variable, which means that
/// parts of the variable will be moved while other parts stay.
///
/// In such a case, the parent variable _cannot be used_ afterwards as a whole,
/// however the parts that are only referenced (and not moved) _can still be
/// used_.

fn main() {
    ex1();
    ex2();
    ex3();
}

#[derive(Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

fn ex1() {
    let c = 'Q';
    let ref ref_c1 = c; // equivalent to: `let ref_c1 = &c;`
    let ref_c2 = &c;

    println!("ref_c1 equals ref_c2: {}", *ref_c1 == *ref_c2);
}

fn ex2() {
    let point = Point { x: 0, y: 0 };
    let mut mutable_point = point;

    {
        // `ref` can be paired with `mut` to take mutable references.
        let Point {
            x: _,
            y: ref mut mut_ref_to_y,
        } = mutable_point;
        *mut_ref_to_y = 1;
    }

    println!("point is ({}, {})", point.x, point.y);
    println!(
        "mutable_point is ({}, {})",
        mutable_point.x, mutable_point.y
    );
}

fn ex3() {
    #[derive(Debug)]
    struct Person {
        name: String,
        age: Box<u8>,
    }

    let person = Person {
        name: String::from("Alice"),
        age: Box::new(20),
    };

    // If `Person.age` were stored on the stack, `ref` would not be required as
    // the definition of age would copy the data from `person.age` without
    // moving it.
    let Person { name, ref age } = person;

    println!("The person's age is {}", age);
    println!("The person's name is {}", name);
    // println!("The person struct is {:?}", person); // error: borrow of partially moved value.
    println!("The person's age from person struct is {}", person.age);
}
