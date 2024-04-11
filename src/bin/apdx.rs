// apdx.rs, Examples from the book "Rust Programming Language"
// Copyright (C) 2024, Savio Sena <savio.sena@gmail.com>

/// # Raw Identifiers
///
/// Raw identifiers are the syntax that lets you use keywords where they
/// wouldn’t normally be allowed. You use a raw identifier by prefixing a
/// keyword with `r#`.
///
/// For example, match is a keyword. If you try to compile the following
/// function that uses match as its name.
///
/// The code below will compile without any errors. Note the r# prefix on the
/// function name in its definition as well as where the function is called in
/// main.

fn raw_identifiers() {
    fn r#match(needle: &str, haystack: &str) -> bool {
        haystack.contains(needle)
    }
    println!("match('foo', 'foobar') -> {}", r#match("foo", "foobar"));
    assert!(r#match("foo", "foobar"));
}

/// # `Debug` for Programmer Output
///
/// The `Debug` trait enables debug formatting in format strings, which you
/// indicate by adding `:?` within `{}` placeholders.

#[derive(Debug)]
struct MyDebug {
    attribute_one: i32,
    attribute_two: String,
}

/// # `PartialEq` and `Eq` for Equality Comparisons
///
/// ## `PartialEq`
///
/// The `PartialEq` trait allows you to compare instances of a type to check for
/// equality and enables use of the `==` and `!=` operators. Deriving
/// `PartialEq` implements the `eq` method. Two instances are equal only if
/// _all_ fields are equal.
///
/// ## `Eq`
///
/// The `Eq` trait has no methods. Its purpose is to signal that for every value
/// of the annotated type, the value is equal to itself.
///
/// One example of a type that cannot implement `Eq` is the floating point
/// number types: its implementation states that two instances of the
/// not-a-number (`NaN`) value are not equal to each other.
///
/// An example of when `Eq` is required is for keys in a `HashMap<K, V>` so the
/// `HashMap<K, V>` can tell whether two keys are the same.

#[derive(PartialEq)]
struct MyPartialEq {
    attribute_one: i32,
    attribute_two: String,
}

impl std::fmt::Display for MyPartialEq {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.attribute_one, self.attribute_two)
    }
}

fn my_part_is_equal<T: PartialEq + std::fmt::Display>(x: &T, y: &T) {
    if x == y {
        println!("{x} == {y}");
    } else {
        println!("{x} != {y}");
    }
}

/// # `Hash` for Mapping a Value to a Value of Fixed Size
///
/// The `Hash` trait allows you to take an instance of a type of arbitrary size
/// and map that instance to a value of fixed size using a hash function.
///
/// Deriving `Hash` implements the `hash` method. The derived implementation of the
/// `hash` method combines the result of calling `hash` on each of the parts of the
/// type, meaning _all_ fields or values must also implement `Hash` to derive Hash.

#[derive(Hash)]
struct MyHashable {
    attribute_one: i32,
    attribute_two: String,
}

use std::hash::{Hash, Hasher};

/// Similar to `MyHashable` but implemented manually.
struct MyHashable2 {
    attribute_one: i32,
    attribute_two: String,
}

impl Hash for MyHashable2 {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.attribute_one.hash(state);
        self.attribute_two.hash(state);
    }
}

/// Calculates the hash value for a given value implementing the `Hash` trait.
fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    t.hash(&mut hasher);
    hasher.finish()
}

/// `Default` for Default Values
///
/// The `Default` trait allows you to create a default value for a type. Deriving
/// `Default` implements the `default` function.
///
/// The derived implementation of the `default` function calls the `default`
/// function on _each_ part of the type, meaning _all_ fields or values in the
/// type _must_ also implement `Default` to derive `Default`.
///
/// The `Default::default` function is commonly used in combination with the
/// _struct update_ syntax discussed previously. You can customize a few fields
/// of a struct and then set and use a default value for the rest of the fields
/// by using `..Default::default()`.
///
/// The `Default` trait is required when you use the method `unwrap_or_default`
/// on `Option<T>` instances, for example. If the `Option<T>` is `None`, the
/// method `unwrap_or_default` will return the result of `Default::default` for
/// the type `T` stored in the `Option<T>`.

#[derive(Default)]
struct SomeOptions {
    foo: i32,
    bar: f32,
}

#[derive(Debug, Default)]
enum Kind {
    #[default] A,
    B,
    C,
}

/// # Derivable Traits

fn derivable_traits() {
    // Debug
    let x = MyDebug {
        attribute_one: 1,
        attribute_two: "two".to_string()
    };
    println!("{:?}", x);
    println!("MyDebug {{ attribute_one: {}, attribute_two: \"{}\" }}", x.attribute_one, x.attribute_two);

    // PartialEq
    let x = MyPartialEq {
        attribute_one: 1,
        attribute_two: "two".to_string()
    };
    let mut y = MyPartialEq {
        attribute_one: 1,
        attribute_two: "two".to_string()
    };
    my_part_is_equal(&x, &y);
    y.attribute_one = 2;
    my_part_is_equal(&x, &y);

    // Hash
    let x = MyHashable {
        attribute_one: 1,
        attribute_two: "two".to_string()
    };
    let y = MyHashable {
        attribute_one: 1,
        attribute_two: "two".to_string()
    };
    assert_eq!(calculate_hash(&x), calculate_hash(&y));
    println!("{} <-> {}", calculate_hash(&x), calculate_hash(&y));

    let x = MyHashable2 {
        attribute_one: 1,
        attribute_two: "one".to_string()
    };
    let y = MyHashable2 {
        attribute_one: 2,
        attribute_two: "two".to_string()
    };
    assert_ne!(calculate_hash(&x), calculate_hash(&y));
    println!("{} <-> {}", calculate_hash(&x), calculate_hash(&y));

    // Default
    let options: SomeOptions = Default::default();
    println!("options = {{ foo: {}, bar: {} }}", options.foo, options.bar);

    let options = SomeOptions {
        foo: 42,
        ..Default::default() // Retain other defaults
    };
    println!("options = {{ foo: {}, bar: {} }}", options.foo, options.bar);

    let kind: Kind = Default::default();
    println!("{:?}", kind);
}

fn main() {
    println!("-=- raw_identifier() -=-");
    raw_identifiers();

    println!("-=- derivable_traits() -=-");
    derivable_traits();
}
