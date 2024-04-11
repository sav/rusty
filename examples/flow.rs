// flow.rs, Examples from the book "Rust by Example"
// Copyright (C) 2024, Savio Sena <savio.sena@gmail.com>

#[allow(unused_variables)]

pub fn if_else() {
    let n = 5;

    let m = if n <= 0 { 10 * n } else { n / 2 };

    println!("{n} {m}");
}

#[allow(unreachable_code, unused_labels)]

pub fn labels() {
    'outer: loop {
        println!("labels(): outer.");
        'inner: loop {
            println!("labels(): inner.");
            break 'outer;
        }
    }
    println!("labels(): end.");
}

pub fn loop_ret() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2; // return from loop
        }
    };

    assert_eq!(result, 20);
    println!("result = {result}");
}

pub fn for_match_str() {
    let names = vec!["foo", "bar", "far"];
    for name in names.iter() {
        match name {
            &"far" => println!("string found"),
            _ => print!(""),
        }
    }
}

pub fn iter_mut() {
    let mut names = vec!["Bob", "Frank", "Ferris"];
    for name in names.iter_mut() {
        *name = match name {
            &mut "Ferris" => "Savio",
            &mut s => s,
        }
    }
    println!("names: {:?}", names);
}

#[allow(dead_code)]
pub enum Temperature {
    Celsius(i32),
    Fahrenheit(i32),
}

pub fn guards() {
    let temperature = Temperature::Celsius(35);
    match temperature {
        Temperature::Celsius(t) if t > 30 => println!("{}C is above 30 Celsius", t),
        // The `if condition` part ^ is a guard
        Temperature::Celsius(t) => println!("{}C is equal to or below 30 Celsius", t),

        Temperature::Fahrenheit(t) if t > 86 => println!("{}F is above 86 Fahrenheit", t),
        Temperature::Fahrenheit(t) => println!("{}F is equal to or below 86 Fahrenheit", t),
    }
}

pub fn binding() {
    fn age() -> u32 {
        15
    }
    match age() {
        0 => println!("I haven't celebrated my first birthday yet"),
        n @ 1..=12 => println!("I'm a child of age {:?}", n),
        n @ 13..=19 => println!("I'm a teen of age {:?}", n),
        n => println!("I'm an old person of age {:?}", n),
    }

    fn some_number() -> Option<u32> {
        Some(42)
    }
    match some_number() {
        Some(n @ 42) => println!("The Answer: {}!", n),
        Some(n) => println!("Not interesting... {}", n),
        _ => (),
    }
}

pub fn if_let() {
    let number = Some(7);
    let letter: Option<i32> = None;

    if let Some(i) = number {
        println!("Matched {:?}!", i);
    }

    if let Some(i) = letter {
        println!("Matched {:?}!", i);
    } else {
        // Destructure failed. Change to the failure case.
        println!("Didn't match a number. Let's go with a letter!");
    }

    enum Foo {
        Bar,
        Baz(u32),
    }

    let a = Foo::Bar;
    let b = Foo::Baz(100);

    if let Foo::Bar = a {
        println!("a is Foo::Bar");
    }

    if let Foo::Baz(value) = b {
        println!("c is {}", value);
    }

    // Binding also works with `if let`
    if let Foo::Baz(value @ 100) = b {
        println!("c is one hundred: {value}");
    }
}

use std::str::FromStr;

pub fn get_count_item(s: &str) -> (u64, &str) {
    let mut it = s.split(' ');
    let (Some(count_str), Some(item)) = (it.next(), it.next()) else {
        panic!("Can't segment count item pair: '{s}'");
    };
    let Ok(count) = u64::from_str(count_str) else {
        panic!("Can't parse integer: '{count_str}'");
    };
    (count, item)
}

/// With `let-else`, a refutable pattern can match and bind variables in the
/// surrounding scope like a normal `let`, or else diverge (e.g. `break`,
/// `return`, `panic!`) when the pattern doesn't match.
///
/// 🛈 stable since: rust 1.65
/// 🛈 you can target specific edition by compiling like this rustc --edition=2021 main.rs

pub fn let_else() {
    assert_eq!(get_count_item("3 chairs"), (3, "chairs"));
}

fn while_let() {
    let mut optional = Some(0);
    while let Some(i) = optional {
        if i > 4 {
            println!("Greater than 4, quit!");
            optional = None;
        } else {
            println!("`i` is `{:?}`. Try again.", i);
            optional = Some(i + 1);
        }
    }
}

pub fn main() {
    println!("-=- if_else() -=-");
    if_else();

    println!("-=- labels() -=-");
    labels();

    println!("-=- loop_ret() -=-");
    loop_ret();

    println!("-=- for_match_str() -=-");
    for_match_str();

    println!("-=- iter_mut() -=-");
    iter_mut();

    println!("-=- guards() -=-");
    guards();

    println!("-=- binding() -=-");
    binding();

    println!("-=- if_let() -=-");
    if_let();

    println!("-=- let-else() -=-");
    let_else();

    println!("-=- while_let() -=-");
    while_let();
}
