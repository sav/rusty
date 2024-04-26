// lifetime1.rs, Examples from the book "Rust by Example"
// Copyright (C) 2024, Savio Sena <savio.sena@gmail.com>

#![allow(dead_code)]

/// # Lifetimes

fn main() {
    ex1(); // Functions
    ex2(); // Methods
    ex3(); // Structs
    ex4(); // Traits
    ex5(); // Bounds
    ex6(); // Coercion
    ex7(); // Static
}

/// ## Example-1: Functions
///
/// Ignoring _elision_, function signatures with lifetimes have a few constraints:
///  - any reference _must_ have an annotated lifetime.
///  - any reference being returned _must_ have the same lifetime as an input or be `static`.

fn ex1() {
    fn pass_x<'a, 'b>(x: &'a i32, _: &'b i32) -> &'a i32 {
        x
    }
    let x = pass_x(&1, &2);
    println!("{x}");
}

/// ## Example-2: Methods
/// 
/// Methods are annotated similarly to functions. 

fn ex2() {
    let mut owner = Owner(18);
    owner.add_one();
    owner.print();
}

struct Owner(i32);

impl Owner {
    fn add_one<'a>(&'a mut self) {
        self.0 += 1;
    }
    fn print<'a>(&'a self) {
        println!("`print`: {}", self.0);
    }
}

/// ## Example-3: Structs
/// 
/// Annotation of lifetimes in structures are also similar to functions.

fn ex3() {
    let x = 18;
    let y = 15;

    let single = Borrowed(&x);
    let double = NamedBorrowed { x: &x, y: &y };
    let reference = Either::Ref(&x);
    let number = Either::Num(y);

    println!("x is borrowed in {:?}", single);
    println!("x and y are borrowed in {:?}", double);
    println!("x is borrowed in {:?}", reference);
    println!("y is *not* borrowed in {:?}", number);
}

/// The reference to `i32` must outlive `Borrowed`.
#[derive(Debug)]
struct Borrowed<'a>(&'a i32);

/// Both references here must outlive this structure.
#[derive(Debug)]
struct NamedBorrowed<'a> {
    x: &'a i32,
    y: &'a i32,
}

/// An enum which is either an `i32` or a reference to one.
#[derive(Debug)]
enum Either<'a> {
    Num(i32),
    Ref(&'a i32),
}

/// ## Example-4: Traits
///
/// Annotation of lifetimes in trait methods basically are similar to functions.
/// Note that `impl` may have annotation of lifetimes too.

fn ex4() {
    let b: Borrowed2 = Default::default();
    println!("b is {:?}", b);
}

// A struct with annotation of lifetimes.
#[derive(Debug)]
struct Borrowed2<'a> {
    x: &'a i32,
}

// Annotate lifetimes to impl.
impl<'a> Default for Borrowed2<'a> {
    fn default() -> Self {
        Self { x: &10 }
    }
}

use std::fmt::Debug; // Trait to bound with.

/// ## Example-5: Bounds
///
/// Just like generic types can be bounded, lifetimes (themselves generic) use
/// bounds as well. The `:` character has a slightly different meaning here, but `+`
/// is the same. Note how the following read:
/// 
///  - `T: 'a`: _All_ references in `T` must outlive lifetime `'a`.
///  - `T: Trait + 'a`: Type `T` must implement trait `Trait` and all references
///    in `T` must outlive `'a`.
///  
/// The example below shows the above syntax in action used after keyword where:

fn ex5() {
    let x = 7;
    let ref_x = Ref(&x);

    print_ref(&ref_x);
    print(ref_x);
}

/// `Ref` contains a reference to a generic type `T` that has an unknown
/// lifetime `'a`. `T` is bounded such that any *references* in `T` must outlive
/// `'a`. Additionally, the lifetime of `Ref` may not exceed `'a`.
#[derive(Debug)]
struct Ref<'a, T: 'a>(&'a T);

fn print<T>(t: T)
where
    T: Debug,
{
    println!("`print`: t is {:?}", t);
}

/// Here a reference to `T` is taken where `T` implements `Debug` and all
/// *references* in `T` outlive `'a`. In addition, `'a` must outlive the
/// function.
fn print_ref<'a, T>(t: &'a T)
where
    T: Debug + 'a,
{
    println!("`print_ref`: t is {:?}", t);
}

/// ## Example-6: Coercion
///
/// A longer lifetime can be coerced into a shorter one so that it works inside
/// a scope it normally wouldn't work in. This comes in the form of inferred
/// coercion by the Rust compiler, and also in the form of declaring a lifetime
/// difference:

fn ex6() {
    let first = 2; // Longer lifetime
    {
        let second = 3; // Shorter lifetime
        println!("The product is {}", multiply(&first, &second));
        println!("{} is the first", choose_first(&first, &second));
    };
}

/// Here, Rust infers a lifetime that is as short as possible. The two
/// references are then _coerced_ to that lifetime.
fn multiply<'a>(first: &'a i32, second: &'a i32) -> i32 {
    first * second
}

/// `<'a: 'b, 'b>` reads as lifetime `'a` is at least as long as `'b`. Here, we
/// take in an `&'a i32` and return a `&'b i32` as a result of _coercion_.
fn choose_first<'a: 'b, 'b>(first: &'a i32, _: &'b i32) -> &'b i32 {
    first
}

extern crate rand;
use rand::Fill;

/// ## Example-7: Static
///
/// Rust has a few reserved lifetime names. One of those is `'static`. You might
/// encounter it in two situations:
///
///  - A reference with `'static` lifetime.
///  - `'static` as part of a trait bound.
///  
/// ```no_run
/// let s: &'static str = "hello world";
/// fn generic<T>(x: T) where T: 'static {}
/// ```
///
/// Both are related but subtly different and this is a common source for
/// confusion when learning Rust. Here are some examples for each situation:
///
/// ### Reference Lifetime
///
/// As a reference lifetime `'static` indicates that the data pointed to by the
/// reference lives for the remaining lifetime of the running program. It can
/// still be coerced to a shorter lifetime.
///
/// There are two ways to make variables with `'static` lifetime:
///
///  - Make a constant with the `static` declaration.
///  - Make a `string` literal which has type: `&'static str`.
///
/// ### Trait bound
/// 
/// As a trait bound, it means the type does not contain any non-static
/// references. For example:
/// ```no_run
/// fn print_it(input: impl Debug + 'static) {...}
/// ```
/// Eg. the receiver can hold on to the type for as long as they want and it
/// will never become invalid until they drop it.
///
/// It's important to understand this means that any owned data _always passes a
/// `'static` lifetime bound_, but a reference to that owned data _generally
/// does not_.

fn ex7() {
    // Reference lifetime

    static ANSWER: i32 = 42;

    /// Coerce the lifetime of `ANSWER` to a shorter lifetime, `'a`.
    fn coerce_static<'a>(_: &'a i32) -> &'a i32 {
        &ANSWER
    }
    
    {
        let static_string = "I'm in read-only memory";
        println!("static_string: {}", static_string);
    }

    {
        let lifetime_num = 9;
        // Coerce `ANSWER` to the lifetime of `lifetime_num`:
        let coerced_static = coerce_static(&lifetime_num);
        println!("lifetime_num = {lifetime_num}, coerced_static: {coerced_static}");
    }
    println!("ANSWER: {ANSWER} stays accessible!");

    // A further example demonstrates how to use `Box::leak` to dynamically
    // create `'static` references. 

    fn random_vec() -> &'static [usize; 5] {
        let mut rng = rand::thread_rng();
        let mut boxed = Box::new([0; 5]);
        boxed.try_fill(&mut rng).unwrap();
        Box::leak(boxed)
    }

    // In this case it definitely doesn't live for the "entire duration of the
    // program", but only for the leaking point onward.

    let first: &'static [usize; 5] = random_vec();
    let second: &'static [usize; 5] = random_vec();

    assert_ne!(first, second);

    println!("{first:?}\n{second:?}");

    // One last example to demonstrate clearly the dynamic creation of static
    // lifetime.

    #[derive(Debug)]
    struct A {
        value: i32
    }

    impl Drop for A {
        fn drop(&mut self) {
            println!("Dropping A with value '{}'!", self.value);
        }
    }

    fn leaked_a() -> &'static mut A {
        let boxed = Box::new(A{ value: 31337 });
        Box::leak(boxed)
    }

    let a = A { value: 1 };
    println!("{a:?}");
    
    {
        let a: &'static A = leaked_a();
        println!("{a:?}");
    }

    println!("Leaked data has not yet been dropped at this point.");

    // Trait bound
    
    fn print_it(input: impl Debug + 'static) {
        println!("'static value passed in is: {:?}", input);
    }

    let i = 5; // i is owned and contains no references, thus it's 'static.
    print_it(i);

    // print_it(&i); // error: &i is not 'static.

    fn print_it_alt(input: &'static impl Debug) {
        println!("'static reference passed in to: {:?}", input);
    }

    static I: i32 = 5;
    print_it_alt(&I);

    fn print_it_other<'a>(input: &'a impl Debug) {
        println!("'a reference passed in to: {:?}", input);
    }

    print_it_other(&i);
    print_it_other(&I);
}
