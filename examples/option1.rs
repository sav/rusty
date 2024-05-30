// option1.rs, Examples from the book "Rust by Example"
// Copyright (C) 2024, Savio Sena <savio.sena@gmail.com>

#![allow(dead_code)]

#[derive(Debug)]
enum Food {
    Apple,
    Carrot,
    Potato,
    CordonBleu,
    Steak,
    Sushi,
}

#[derive(Debug)]
struct Peeled(Food);

#[derive(Debug)]
struct Chopped(Food);

#[derive(Debug)]
struct Cooked(Food);

fn peel(food: Option<Food>) -> Option<Peeled> {
    match food {
        Some(food) => Some(Peeled(food)),
        None => None,
    }
}

fn chop(peeled: Option<Peeled>) -> Option<Chopped> {
    match peeled {
        Some(Peeled(food)) => Some(Chopped(food)),
        None => None,
    }
}

fn cook(chopped: Option<Chopped>) -> Option<Cooked> {
    chopped.map(|Chopped(food)| Cooked(food))
}

/// Replaces all previous functions while staying compact.
fn process(food: Option<Food>) -> Option<Cooked> {
    food.map(|f| Peeled(f))
        .map(|Peeled(f)| Chopped(f))
        .map(|Chopped(f)| Cooked(f))
}

/// Check whether there's food or not before trying to eat it!
fn eat(food: Option<Cooked>) {
    match food {
        Some(food) => println!("Mmm. I love {:?}", food),
        None => println!("Oh no! It wasn't edible."),
    }
}

/// `Option` has a built in method called `map()`, a _combinator_ for the simple
/// mapping of `Some -> Some` and `None -> None`. Multiple `map()` calls can be
/// chained together for even more flexibility.
///
/// `map` is a chainable way to simplify `match`. In this example, `process()`
/// replaces all functions previous to it while staying compact.

fn ex1_combinators_map() {
    let apple = Some(Food::Apple);
    let carrot = Some(Food::Carrot);
    let potato = None;

    let cooked_apple = cook(chop(peel(apple)));
    let cooked_carrot = cook(chop(peel(carrot)));

    // Let's try the simpler looking `process()` now.
    let cooked_potato = process(potato);

    eat(cooked_apple);
    eat(cooked_carrot);
    eat(cooked_potato);
}

#[derive(Debug)]
enum Day {
    Monday,
    Tuesday,
    Wednesday,
}

/// We don't have the ingredients to make Sushi.
fn have_ingredients(food: Food) -> Option<Food> {
    match food {
        Food::Sushi => None,
        _ => Some(food),
    }
}

/// We have the recipe for everything except for Cordon Bleu.
fn have_recipe(food: Food) -> Option<Food> {
    match food {
        Food::CordonBleu => None,
        _ => Some(food),
    }
}

/// To make a dish, we need both the recipe and the ingredients.
/// We can represent the logic with a chain of `match`es.
fn cookable_v1(food: Food) -> Option<Food> {
    match have_recipe(food) {
        None => None,
        Some(food) => have_ingredients(food),
    }
}

/// This can conveniently be rewritten more compactly with `and_then()`:
fn cookable_v3(food: Food) -> Option<Food> {
    have_recipe(food).and_then(have_ingredients)
}

/// Otherwise we'd need to `flatten()` an `Option<Option<Food>>`
/// to get an `Option<Food>`.
fn cookable_v2(food: Food) -> Option<Food> {
    have_recipe(food).map(have_ingredients).flatten()
}

fn eat2(food: Food, day: Day) {
    match cookable_v3(food) {
        Some(food) => println!("Yay! On {:?} we get to eat {:?}.", day, food),
        None => println!("Oh no. We don't get to eat on {:?}?", day),
    }
}

/// However, using `map()` on a function that returns an `Option<T>` results in
/// the nested `Option<Option<T>>`. Chaining multiple calls together can then
/// become confusing. That's where another combinator called `and_then()`, known
/// in some languages as _flatmap_, comes in.
///
/// `and_then()` calls its function input with the _wrapped value_ and returns
/// the result. If the `Option` is `None`, then it returns `None` instead.
///
/// In the following example, `cookable_v3()` results in an `Option<Food>`.
/// Using `map()` instead of `and_then()` would have given an
/// `Option<Option<Food>>`, which is an invalid type for `eat()`.

fn ex2_combinators_and_then() {
    let (cordon_bleu, steak, sushi) = (Food::CordonBleu, Food::Steak, Food::Sushi);

    eat2(cordon_bleu, Day::Monday);
    eat2(steak, Day::Tuesday);
    eat2(sushi, Day::Wednesday);
}

#[derive(Debug)]
enum Fruit {
    Apple,
    Orange,
    Banana,
    Kiwi,
    Lemon,
}

/// `or()` is chainable, evaluates eagerly, keeps empty value intact.
fn ex3_unpacking_options_or() {
    let apple = Some(Fruit::Apple);
    let orange = Some(Fruit::Orange);
    let no_fruit: Option<Fruit> = None;

    let first_available_fruit = no_fruit.or(orange).or(apple); // `or` moves its argument.
    println!("first_available_fruit: {:?}", first_available_fruit); // Some(Orange)
                                                                    // println!("Variable apple was moved, so this line won't compile: {:?}", apple); // error.
}

/// `or_else()` is chainable, evaluates lazily, keeps empty value intact.
fn ex3_unpacking_options_or_else() {
    let no_fruit: Option<Fruit> = None;
    let get_kiwi_as_fallback = || {
        println!("Providing kiwi as fallback");
        Some(Fruit::Kiwi)
    };
    let get_lemon_as_fallback = || {
        println!("Providing lemon as fallback");
        Some(Fruit::Lemon)
    };

    let first_available_fruit = no_fruit
        .or_else(get_kiwi_as_fallback)
        .or_else(get_lemon_as_fallback);
    println!("first_available_fruit: {:?}", first_available_fruit); // Some(Kiwi)
}

/// `get_or_insert()` evaluates eagerly, modifies empty value in place
fn ex3_unpacking_options_get_or_insert() {
    let mut my_fruit: Option<Fruit> = None;
    let apple = Fruit::Apple;
    let first_available_fruit = my_fruit.get_or_insert(apple);
    println!("first_available_fruit is: {:?}", first_available_fruit); // Apple
    println!("my_fruit is: {:?}", my_fruit); // Some(Apple)
}

/// `get_or_insert_with()` evaluates lazily, modifies empty value in place.
fn ex3_unpacking_options_get_or_insert_with() {
    let mut my_fruit: Option<Fruit> = None;
    let get_lemon_as_fallback = || {
        println!("Providing lemon as fallback");
        Fruit::Lemon
    };
    let first_available_fruit = my_fruit.get_or_insert_with(get_lemon_as_fallback);
    println!("first_available_fruit is: {:?}", first_available_fruit); // Lemon
    println!("my_fruit is: {:?}", my_fruit); // Some(Lemon)

    let mut my_apple = Some(Fruit::Apple);
    let should_be_apple = my_apple.get_or_insert_with(get_lemon_as_fallback);
    println!("should_be_apple is: {:?}", should_be_apple); // Apple
    println!("my_apple is unchanged: {:?}", my_apple); // Some(Apple)
}

/// There is more than one way to unpack an `Option` and fall back on a default
/// if it is `None`. To choose the one that meets our needs, we need to consider
/// the following:
///  - do we need eager or lazy evaluation?
///  - do we need to keep the original empty value intact, or modify it in place?
fn ex3_unpacking_options() {
    println!("-=- ex3_unpacking_options_or() -=-");
    ex3_unpacking_options_or();
    println!("-=- ex3_unpacking_options_or_else() -=-");
    ex3_unpacking_options_or_else();
    println!("-=- ex3_unpacking_options_get_or_insert() -=-");
    ex3_unpacking_options_get_or_insert();
    println!("-=- ex3_unpacking_options_get_or_insert_with() -=-");
    ex3_unpacking_options_get_or_insert_with();
}

fn ex4_turbofish() {
    let none_int = None::<i32>;
    let some_float = Some(0f32);

    println!("{none_int:?}, {some_float:?}");
}

fn ex5_api() {
    // --- as_mut ---
    let mut x = Some(2);
    match x.as_mut() {
        Some(v) => *v = 42,
        None => {}
    }
    assert_eq!(x, Some(42));
    println!("{}", x.unwrap());

    // --- as_ref ---
    let text: Option<String> = Some("hello world!".to_string());
    // cast `Option<String>` to `Option<&String>` then consume *that* with map,
    // leaving `text` on the stack.
    let text_length: Option<usize> = text.as_ref().map(|s| s.len());
    println!(
        "still can print text: {} (length: {})",
        text.unwrap(),
        text_length.unwrap()
    );

    // --- as_deref ---
    // Converts from `Option<T>` (or `&Option<T>`) to `Option<&T::Target>`,
    // where `Target` is a associated type of `Deref`.
    // Leaves the original `Option` in-place, creating a new one with a
    // reference to the original one, aditionally coercing the contents via
    // `Deref`.
    let x: Option<String> = Some("hey".to_owned());
    assert_eq!(x.as_deref(), Some("hey"));
    let x: Option<String> = None;
    assert_eq!(x.as_deref(), None);

    // --- as_deref_mut ---
    // Converts from `Option<T>` (or `&Option<T>`) to `Option<&mut T::Target>`,
    // where `Target` is a associated type of `Deref`.
    let mut x: Option<String> = Some("hey".to_owned());
    assert_eq!(
        x.as_deref_mut().map(|x| {
            x.make_ascii_uppercase();
            x
        }),
        Some("HEY".to_owned().as_mut_str())
    );

    // --- take ---
    let mut x = Some(2);
    let y = x.take();
    println!("{:?}, {:?}", x, y);

    // --- is_some ---
    let x = Some(1);
    assert_eq!(x.is_some(), true);
    let y = None::<i128>;
    assert_eq!(y.is_some(), false);
    println!("{}, {}", x.is_some(), y.is_some());

    // --- is_none ---
    let x: Option<u32> = Some(2);
    assert_eq!(x.is_none(), false);
    let x: Option<u32> = None;
    assert_eq!(x.is_none(), true);
    println!("{}, {}", x.is_none(), y.is_none());

    // --- and ---
    let x = Some(2);
    let y: Option<&str> = None;
    assert_eq!(x.and(y), None);
    println!("{:?}, {:?}", x, y);
    let x = Some(1);
    let y = Some("foo");
    assert_eq!(x.and(y), Some("foo"));
    println!("{:?}, {:?}", x, y);

    // --- and_then ---
    fn sq_then_to_string(x: u32) -> Option<String> {
        x.checked_mul(x).map(|sq| sq.to_string())
    }
    assert_eq!(Some(2).and_then(sq_then_to_string), Some(4.to_string()));
    assert_eq!(Some(1_000_000).and_then(sq_then_to_string), None); // overflowed!
    assert_eq!(None.and_then(sq_then_to_string), None);

    // --- is_some_and ---
    let x: Option<u32> = Some(2);
    assert_eq!(x.is_some_and(|x| x > 1), true);
    let x: Option<u32> = Some(0);
    assert_eq!(x.is_some_and(|x| x > 1), false);
    let x: Option<u32> = None;
    assert_eq!(x.is_some_and(|x| x > 1), false);

    // --- inspect ---
    let list = vec![1, 2, 3];
    let _ = list
        .get(1)
        .inspect(|x| println!("got: {x}"))
        .expect("list should be long enough");
    list.get(5).inspect(|x| println!("got: {x}")); // prints nothing

    // --- map ---
    let maybe_some_string = Some(String::from("hello world!"));
    // `Option::map` takes self *by value*, consuming `maybe_some_string`
    let maybe_some_len = maybe_some_string.map(|s| s.len());
    assert_eq!(maybe_some_len, Some(12));
    let x: Option<&str> = None;
    assert_eq!(x.map(|s| s.len()), None);

    // --- map_or ---
    let x = Some("foo");
    assert_eq!(x.map_or(42, |v| v.len()), 3);
    let x: Option<&str> = None;
    assert_eq!(x.map_or(42, |v| v.len()), 42);

    // --- map_or_else ---
    let k = 21;
    let x = Some("foo");
    assert_eq!(x.map_or_else(|| 2 * k, |v| v.len()), 3);
    let x: Option<&str> = None;
    assert_eq!(x.map_or_else(|| 2 * k, |v| v.len()), 42);

    // --- flatten ---
    // Flattening only removes one level of nesting at a time:
    let x: Option<Option<Option<u32>>> = Some(Some(Some(6)));
    assert_eq!(Some(Some(6)), x.flatten());
    assert_eq!(Some(6), x.flatten().flatten());

    // --- zip ---
    let x = Some(1);
    let y = Some("hi");
    let z = None::<u8>;
    assert_eq!(x.zip(y), Some((1, "hi")));
    assert_eq!(x.zip(z), None);

    // --- unzip ---
    let x = Some((1, "hi"));
    let y = None::<(u8, u32)>;
    assert_eq!(x.unzip(), (Some(1), Some("hi")));
    assert_eq!(y.unzip(), (None, None));

    // --- copied ---
    // Maps an `Option<&T>` to an `Option<T>` by _copying_ the contents of the option.
    // There's a version of `copied` for `Option<&mut T>` as well.
    let x = 12;
    let opt_x = Some(&x);
    assert_eq!(opt_x, Some(&12));
    let copied = opt_x.copied();
    assert_eq!(copied, Some(12));

    // --- cloned ---
    // Maps an `Option<&mut T>` to an `Option<T>` by _cloning_ the contents of the option.
    // There's a version of `cloned` for `Option<&T>` as well.
    let mut x = 12;
    let opt_x = Some(&mut x);
    assert_eq!(opt_x, Some(&mut 12));
    let cloned = opt_x.cloned();
    assert_eq!(cloned, Some(12));
}

fn main() {
    println!("-=- ex1_combinators_map() -=-");
    ex1_combinators_map();
    println!("-=- ex2_combinators_and_then() -=-");
    ex2_combinators_and_then();
    println!("-=- ex3_unpacking_options() -=-");
    ex3_unpacking_options();
    println!("-=- ex4_turbofish()");
    ex4_turbofish();
    println!("-=- ex5_api()");
    ex5_api();
}
