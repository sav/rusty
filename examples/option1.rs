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

fn main() {
    println!("-=- ex1_combinators_map() -=-");
    ex1_combinators_map();
    println!("-=- ex2_combinators_and_then() -=-");
    ex2_combinators_and_then();
    println!("-=- ex3_unpacking_options() -=-");
    ex3_unpacking_options();
    println!("-=- ex4_turbofish()");
    ex4_turbofish();
}
